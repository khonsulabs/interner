use std::borrow::Cow;
use std::collections::hash_map::RandomState;
use std::fmt::Debug;
use std::hash::{BuildHasher, Hash};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use crate::global::GlobalPool;
use crate::pool::{Pool, PoolKindSealed};
use crate::{PoolKind, Pooled};

/// A pooled string that belongs to a [`StringPool`].
pub type SharedString<S = RandomState> = Pooled<SharedPool<String, S>, S>;
/// A pooled path that belongs to a [`PathPool`].
pub type SharedPath<S = RandomState> = Pooled<SharedPool<PathBuf, S>, S>;
/// A pooled buffer that belongs to a [`BufferPool`].
pub type SharedBuffer<S = RandomState> = Pooled<SharedPool<Vec<u8>, S>, S>;

/// A string interning pool that manages [`SharedString`]s.
///
/// Each [`StringPool`] has its own storage. When comparing [`SharedString`]s
/// from separate pools, the full string comparison function must be used.
pub type StringPool<S = RandomState> = SharedPool<String, S>;
/// A path interning pool that manages [`SharedPath`]s.
///
/// Each [`PathPool`] has its own storage. When comparing [`SharedPath`]s
/// from separate pools, the full string comparison function must be used.
pub type PathPool<S = RandomState> = SharedPool<PathBuf, S>;
/// A path interning pool that manages [`SharedBuffer`]s.
///
/// Each [`BufferPool`] has its own storage. When comparing [`SharedBuffer`]s
/// from separate pools, the full string comparison function must be used.
pub type BufferPool<S = RandomState> = SharedPool<Vec<u8>, S>;

/// A shared pool of values that ensures only one copy of any given value exists
/// at any time.
///
/// To retrieve a [`Pooled`] value, use [`SharedPool::get()`], which is
/// implemented for these types:
///
/// - [`String`]/[`&str`](str)
/// - [`PathBuf`]/[`&Path`](Path)
/// - [`Vec<u8>`]/`&[u8]`
#[derive(Debug)]
pub struct SharedPool<T, S = RandomState>(Arc<Mutex<Pool<Self, S>>>)
where
    T: Debug + Clone + Eq + PartialEq + Hash + Ord + PartialOrd,
    S: BuildHasher;

impl<S> SharedPool<String, S>
where
    S: BuildHasher,
{
    /// Creates a new pool using the provided [`BuildHasher`] for hashing
    /// values.
    #[must_use]
    pub fn with_hasher(hasher: S) -> Self {
        Self::with_capacity_and_hasher(0, hasher)
    }

    /// Creates a new pool using the provided [`BuildHasher`] for hashing
    /// values. The pool will have enough capacity to allow inserting
    /// `initial_capacity` pooled entries without reallocation.
    #[must_use]
    pub fn with_capacity_and_hasher(initial_capacity: usize, hasher: S) -> Self {
        Self(Arc::new(Mutex::new(Pool::with_capacity_and_hasher(
            initial_capacity,
            hasher,
        ))))
    }

    /// Returns a copy of an existing [`SharedString`] if one is found.
    /// Otherwise, a new [`SharedString`] is created and returned.
    ///
    /// While any copies of the returned [`SharedString`] are still allocated,
    /// calling this function is guaranteed to return a copy of the same string.
    #[must_use]
    pub fn get<'a, V>(&self, value: V) -> SharedString<S>
    where
        V: Into<Cow<'a, str>>,
    {
        let value = value.into();
        self.with_active_symbols(|symbols| symbols.get(value, self))
    }
}

impl<S> SharedPool<PathBuf, S>
where
    S: BuildHasher,
{
    /// Returns a copy of an existing [`SharedPath`] if one is found. Otherwise,
    /// a new [`SharedPath`] is created and returned.
    ///
    /// While any copies of the returned [`SharedPath`] are still allocated,
    /// calling this function is guaranteed to return a copy of the same path.
    #[must_use]
    pub fn get<'a, V>(&self, value: V) -> SharedPath<S>
    where
        V: Into<Cow<'a, Path>>,
    {
        let value = value.into();
        self.with_active_symbols(|symbols| symbols.get(value, self))
    }
}

impl<S> SharedPool<Vec<u8>, S>
where
    S: BuildHasher,
{
    /// Returns a copy of an existing [`SharedBuffer`] if one is found. Otherwise,
    /// a new [`SharedBuffer`] is created and returned.
    ///
    /// While any copies of the returned [`SharedBuffer`] are still allocated,
    /// calling this function is guaranteed to return a copy of the same buffer.
    #[must_use]
    pub fn get<'a, V>(&self, value: V) -> SharedBuffer<S>
    where
        V: Into<Cow<'a, [u8]>>,
    {
        let value = value.into();
        self.with_active_symbols(|symbols| symbols.get(value, self))
    }
}

impl<T, S> Clone for SharedPool<T, S>
where
    T: Debug + Clone + Eq + PartialEq + Hash + Ord + PartialOrd,
    S: BuildHasher,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T, S> PoolKind<S> for SharedPool<T, S>
where
    T: Debug + Clone + Eq + PartialEq + Hash + Ord + PartialOrd,
    S: BuildHasher,
{
}

impl<T, S> PoolKindSealed<S> for SharedPool<T, S>
where
    T: Debug + Clone + Eq + PartialEq + Hash + Ord + PartialOrd,
    S: BuildHasher,
{
    type Stored = T;

    fn with_active_symbols<R>(&self, logic: impl FnOnce(&mut Pool<Self, S>) -> R) -> R {
        let mut symbols = self.0.lock().expect("poisoned");

        logic(&mut symbols)
    }
}

impl<T, S> PartialEq for SharedPool<T, S>
where
    T: Debug + Clone + Eq + PartialEq + Hash + Ord + PartialOrd,
    S: BuildHasher,
{
    fn eq(&self, other: &SharedPool<T, S>) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl<T, S, S2> PartialEq<&'static GlobalPool<T, S2>> for SharedPool<T, S>
where
    T: Debug + Clone + Eq + PartialEq + Hash + Ord + PartialOrd,
    S: BuildHasher,
    S2: BuildHasher,
{
    fn eq(&self, _other: &&'static GlobalPool<T, S2>) -> bool {
        false
    }
}

impl<T, S, S2> PartialEq<SharedPool<T, S>> for &'static GlobalPool<T, S2>
where
    T: Debug + Clone + Eq + PartialEq + Hash + Ord + PartialOrd,
    S: BuildHasher,
    S2: BuildHasher,
{
    fn eq(&self, _other: &SharedPool<T, S>) -> bool {
        false
    }
}

impl<T> Default for SharedPool<T, RandomState>
where
    T: Debug + Clone + Eq + PartialEq + Hash + Ord + PartialOrd,
{
    fn default() -> Self {
        Self(Arc::default())
    }
}
