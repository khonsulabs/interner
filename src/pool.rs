use std::borrow::{Borrow, Cow};
use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::{BuildHasher, Hash};
use std::marker::PhantomData;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::sync::{atomic, Arc};

use crate::{PoolKind, Pooled};

pub trait PoolKindSealed<Hasher> {
    type Owned: Poolable<Boxed = Self::Pooled> + Debug + Clone + Eq + Hash + Ord;
    type Pooled: Debug + Clone + Eq + Hash + Ord;

    fn with_active_symbols<T>(&self, logic: impl FnOnce(&mut Pool<Self, Hasher>) -> T) -> T;
    fn address_of(&self) -> *const ();
}

pub trait Poolable {
    type Boxed: Debug + Clone + Eq + Hash + Ord;

    fn boxed(self) -> Self::Boxed;
}

impl Poolable for String {
    type Boxed = Box<str>;

    fn boxed(self) -> Self::Boxed {
        self.into_boxed_str()
    }
}

impl Poolable for PathBuf {
    type Boxed = Box<Path>;

    fn boxed(self) -> Self::Boxed {
        self.into_boxed_path()
    }
}

impl Poolable for Vec<u8> {
    type Boxed = Box<[u8]>;

    fn boxed(self) -> Self::Boxed {
        self.into_boxed_slice()
    }
}

#[derive(Debug)]
pub struct SharedData<P, S>(pub Arc<Data<P, S>>)
where
    P: PoolKind<S>,
    S: BuildHasher;

impl<P, S> Clone for SharedData<P, S>
where
    P: PoolKind<S>,
    S: BuildHasher,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<P, S> Hash for SharedData<P, S>
where
    P: PoolKind<S>,
    S: BuildHasher,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.value.hash(state);
    }
}

impl<P, S> Eq for SharedData<P, S>
where
    P: PoolKind<S>,
    S: BuildHasher,
{
}

impl<P, S> PartialEq for SharedData<P, S>
where
    P: PoolKind<S>,
    S: BuildHasher,
{
    fn eq(&self, other: &Self) -> bool {
        self.0.index == other.0.index
    }
}

impl<P, S> Borrow<str> for SharedData<P, S>
where
    P: PoolKind<S, Pooled = Box<str>>,
    S: BuildHasher,
{
    fn borrow(&self) -> &str {
        &self.0.value
    }
}

impl<P, S> Borrow<Path> for SharedData<P, S>
where
    P: PoolKind<S, Pooled = Box<Path>>,
    S: BuildHasher,
{
    fn borrow(&self) -> &Path {
        &self.0.value
    }
}

impl<P, S> Borrow<[u8]> for SharedData<P, S>
where
    P: PoolKind<S, Pooled = Box<[u8]>>,
    S: BuildHasher,
{
    fn borrow(&self) -> &[u8] {
        &self.0.value
    }
}

impl<P, S> Drop for SharedData<P, S>
where
    P: PoolKind<S>,
    S: BuildHasher,
{
    fn drop(&mut self) {
        // The main Symbols structure holds two strong references to the same
        // Arc we hold. Thus, if we reach 3 strong count (our ref included), we
        // need to remove the symbol so it can be freeed.
        //
        // We can use any form of atomics here because if the strong count is 3,
        // we can be guaranteed the only thread able to free our data is this
        // thread.
        if Arc::strong_count(&self.0) == 3
            && self
                .0
                .freeing
                .compare_exchange(
                    false,
                    true,
                    atomic::Ordering::Relaxed,
                    atomic::Ordering::Relaxed,
                )
                .is_ok()
        {
            self.0.pool.with_active_symbols(|symbols| {
                // Check that the strong count hasn't changed. If it has, we
                // need to allow the symbol to stay alive.
                if Arc::strong_count(&self.0) > 3 {
                    self.0.freeing.store(false, atomic::Ordering::Relaxed);
                } else {
                    symbols.active.remove(self);
                    symbols.slots[self.0.index] = None;
                    symbols.free_slots.push(self.0.index);
                }
            });
        }
    }
}

#[derive(Debug)]
pub struct Data<P, S>
where
    P: PoolKind<S>,
{
    pub index: usize,
    pub value: P::Pooled,
    pub freeing: AtomicBool,
    pub pool: P,
    _hasher: PhantomData<S>,
}

#[derive(Debug)]
pub struct Pool<P, S>
where
    P: PoolKind<S>,
    S: BuildHasher,
{
    pub active: HashSet<SharedData<P, S>, S>,
    pub slots: Vec<Option<Pooled<P, S>>>,
    pub free_slots: Vec<usize>,
}

impl<P, S> Pool<P, S>
where
    P: PoolKind<S>,
    S: BuildHasher,
{
    pub fn with_capacity_and_hasher(capacity: usize, hasher: S) -> Self {
        Self {
            active: HashSet::with_capacity_and_hasher(capacity, hasher),
            slots: Vec::with_capacity(capacity),
            free_slots: Vec::new(),
        }
    }

    pub fn get<K>(&mut self, pooled: Cow<'_, K>, pool: &P) -> Pooled<P, S>
    where
        K: ToOwned<Owned = P::Owned> + Hash + Eq + ?Sized,
        P::Owned: Borrow<K> + PartialEq<K>,
        SharedData<P, S>: Borrow<K>,
    {
        if let Some(symbol) = self.active.get(pooled.as_ref()).cloned() {
            Pooled(symbol)
        } else {
            let value = pooled.into_owned();

            let index = if let Some(free_slot) = self.free_slots.pop() {
                free_slot
            } else {
                let slot_id = self.slots.len();
                self.slots.push(None);
                slot_id
            };

            let symbol = Pooled(SharedData(Arc::new(Data {
                index,
                value: value.boxed(),
                freeing: AtomicBool::new(false),
                pool: pool.clone(),
                _hasher: PhantomData,
            })));
            self.active.insert(symbol.0.clone());
            self.slots[index] = Some(symbol.clone());
            symbol
        }
    }
}

impl<P> Default for Pool<P, RandomState>
where
    P: PoolKind<RandomState>,
{
    fn default() -> Self {
        Self {
            active: HashSet::with_hasher(RandomState::default()),
            slots: Vec::new(),
            free_slots: Vec::new(),
        }
    }
}
