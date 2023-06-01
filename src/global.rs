use std::borrow::Cow;
use std::collections::hash_map::RandomState;
use std::fmt::Debug;
use std::hash::{BuildHasher, Hash};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

use crate::pool::{Pool, PoolKindSealed, Poolable};
use crate::{PoolKind, Pooled};

/// A pooled string that is stored in a [`GlobalPool`].
///
/// This type implements `From<String>` and `From<&str>`.
pub type GlobalString<S = RandomState> = Pooled<&'static GlobalPool<String, S>, S>;
/// A pooled path that is stored in a [`GlobalPool`].
///
/// This type implements `From<PathBuf>` and `From<&Path>`.
pub type GlobalPath<S = RandomState> = Pooled<&'static GlobalPool<PathBuf, S>, S>;
/// A pooled buffer (`Vec<u8>`) that is stored in a [`GlobalPool`].
///
/// This type implements `From<Vec<u8>>` and `From<&[u8]>`.
pub type GlobalBuffer<S = RandomState> = Pooled<&'static GlobalPool<Vec<u8>, S>, S>;

/// A global string interning pool that manages [`GlobalString`]s.
pub type StringPool<S = RandomState> = GlobalPool<String, S>;
/// A global path interning pool that manages [`GlobalPath`]s.
///
/// Each [`PathPool`] has its own storage. When comparing [`GlobalPath`]s
/// from separate pools, the full string comparison function must be used.
pub type PathPool<S = RandomState> = GlobalPool<PathBuf, S>;
/// A global byte buffer interning pool that manages [`GlobalBuffer`]s.
///
/// Each [`BufferPool`] has its own storage. When comparing [`GlobalBuffer`]s
/// from separate pools, the full string comparison function must be used.
pub type BufferPool<S = RandomState> = GlobalPool<Vec<u8>, S>;

/// A global interned pool.
///
/// This type is used to create globally allocated pools.
///
/// ```rust
/// use interner::global::{GlobalPool, GlobalString};
///
/// static GLOBAL_STRINGS: GlobalPool<String> = GlobalPool::new();
///
/// let interned = GLOBAL_STRINGS.get(String::from("hello"));
/// let second = GLOBAL_STRINGS.get("hello");
///
/// assert!(GlobalString::ptr_eq(&interned, &second));
/// ```
#[derive(Debug)]
pub struct GlobalPool<T, S = RandomState>(Mutex<GlobalPoolState<T, S>>)
where
    T: Poolable + Debug + Clone + Eq + PartialEq + Hash + Ord + PartialOrd + 'static,
    S: BuildHasher + 'static;

#[derive(Debug)]
enum GlobalPoolState<T, S>
where
    &'static GlobalPool<T, S>: PoolKind<S>,
    T: Poolable + Debug + Clone + Eq + PartialEq + Hash + Ord + PartialOrd + 'static,
    S: BuildHasher + 'static,
{
    Initializing,
    LazyInitialize { capacity: usize, hasher: fn() -> S },
    StaticInitialize { capacity: usize, hasher: S },
    Initialized(Pool<&'static GlobalPool<T, S>, S>),
}

impl<T> GlobalPool<T>
where
    T: Poolable + Debug + Clone + Eq + PartialEq + Hash + Ord + PartialOrd,
{
    /// Returns a new instance using [`RandomState`] for the internal hashing.
    #[must_use]
    pub const fn new() -> Self {
        Self::with_capacity_and_hasher_init(0, RandomState::new)
    }
}
impl<T, S> GlobalPool<T, S>
where
    T: Poolable + Debug + Clone + Eq + PartialEq + Hash + Ord + PartialOrd,
    S: BuildHasher,
{
    /// Returns a collection of the currently pooled items.
    #[must_use]
    pub fn pooled<C>(&'static self) -> C
    where
        C: FromIterator<Pooled<&'static Self, S>>,
    {
        self.with_active_symbols(|pool| {
            pool.active
                .iter()
                .map(|data| Pooled(data.clone()))
                .collect()
        })
    }
}
impl<T, S, S2> PartialEq<GlobalPool<T, S2>> for GlobalPool<T, S>
where
    T: Poolable + Poolable + Debug + Clone + Eq + PartialEq + Hash + Ord + PartialOrd,
    S: BuildHasher,
    S2: BuildHasher,
{
    fn eq(&self, other: &GlobalPool<T, S2>) -> bool {
        (self as *const Self).cast::<()>() == (other as *const GlobalPool<T, S2>).cast::<()>()
    }
}

impl<T, S> PoolKindSealed<S> for &'static GlobalPool<T, S>
where
    T: Poolable + Debug + Clone + Eq + PartialEq + Hash + Ord + PartialOrd,
    S: BuildHasher,
{
    type Owned = T;
    type Pooled = T::Boxed;

    fn with_active_symbols<R>(&self, logic: impl FnOnce(&mut Pool<Self, S>) -> R) -> R {
        let mut symbols = self.0.lock().expect("poisoned");
        if !matches!(*symbols, GlobalPoolState::Initialized(_)) {
            let pool = match std::mem::replace(&mut *symbols, GlobalPoolState::Initializing) {
                GlobalPoolState::LazyInitialize { capacity, hasher } => {
                    Pool::with_capacity_and_hasher(capacity, hasher())
                }
                GlobalPoolState::StaticInitialize { capacity, hasher } => {
                    Pool::with_capacity_and_hasher(capacity, hasher)
                }

                _ => unreachable!("invalid state"),
            };
            *symbols = GlobalPoolState::Initialized(pool);
        }

        let GlobalPoolState::Initialized(pool) = &mut *symbols else { unreachable!("always initialized above") };
        logic(pool)
    }

    fn address_of(&self) -> *const () {
        std::ptr::addr_of!(*self).cast()
    }
}

impl<T, S> PoolKind<S> for &'static GlobalPool<T, S>
where
    T: Poolable + Debug + Clone + Eq + PartialEq + Hash + Ord + PartialOrd,
    S: BuildHasher,
{
}

impl<T, S> GlobalPool<T, S>
where
    T: Poolable + Debug + Clone + Eq + PartialEq + Hash + Ord + PartialOrd,
    S: BuildHasher,
{
    /// Returns a new instance using the provided hasher.
    pub const fn with_hasher(hasher: S) -> Self {
        Self::with_capacity_and_hasher(0, hasher)
    }

    /// Returns a new instance using the function to load the hasher when the
    /// pool is initialized on first use.
    pub const fn with_hasher_init(init: fn() -> S) -> Self {
        Self::with_capacity_and_hasher_init(0, init)
    }

    /// Returns a new instance using the provided hasher with enough capacity to
    /// hold the requested number of items without reallocating.
    pub const fn with_capacity_and_hasher(capacity: usize, hasher: S) -> Self {
        Self(Mutex::new(GlobalPoolState::StaticInitialize {
            capacity,
            hasher,
        }))
    }

    /// Returns a new instance using the function to load the hasher when the
    /// pool is initialized on first use. The returned instance has enough
    /// capacity to hold the requested number of items without reallocating.
    pub const fn with_capacity_and_hasher_init(capacity: usize, init: fn() -> S) -> Self {
        Self(Mutex::new(GlobalPoolState::LazyInitialize {
            capacity,
            hasher: init,
        }))
    }
}

impl<S> GlobalPool<String, S>
where
    S: BuildHasher,
{
    /// Returns a copy of an existing [`GlobalString`] if one is found.
    /// Otherwise, a new [`GlobalString`] is created and returned.
    ///
    /// While any copies of the returned [`GlobalString`] are still allocated,
    /// calling this function is guaranteed to return a copy of the same string.
    pub fn get<'a, V>(&'static self, value: V) -> GlobalString<S>
    where
        V: Into<Cow<'a, str>>,
    {
        let value = value.into();
        self.with_active_symbols(|symbols| symbols.get(value, &self))
    }

    /// Returns a static pooled string, which keeps the pooled string allocated
    /// for the duration of the process.
    ///
    /// The string is not initialized until it is retrieved for the first time.
    pub const fn get_static(&'static self, value: &'static str) -> StaticPooledString<S> {
        StaticPooledString::new(self, value)
    }

    /// Returns a static pooled string, which keeps the pooled string allocated
    /// for the duration of the process. The string is initialized using the
    /// function provided when it is retrieved for the first time.
    pub const fn get_static_with(
        &'static self,
        function: fn() -> Cow<'static, str>,
    ) -> StaticPooledString<S> {
        StaticPooledString::new_fn(self, function)
    }
}

impl<S> GlobalPool<PathBuf, S>
where
    S: BuildHasher,
{
    /// Returns a copy of an existing [`GlobalPath`] if one is found.
    /// Otherwise, a new [`GlobalPath`] is created and returned.
    ///
    /// While any copies of the returned [`GlobalPath`] are still allocated,
    /// calling this function is guaranteed to return a copy of the same path.
    pub fn get<'a, V>(&'static self, value: V) -> GlobalPath<S>
    where
        V: Into<Cow<'a, Path>>,
    {
        let value = value.into();
        self.with_active_symbols(|symbols| symbols.get(value, &self))
    }

    // This function serves no purpose, currently, as there's no way to get a
    // static path in a const context -- Path::new() isn't const.
    // /// Returns a static pooled path, which keeps the pooled path allocated for
    // /// the duration of the process.
    // ///
    // /// The path is not initialized until it is retrieved for the first time.
    // pub const fn get_static(&'static self, value: &'static Path) -> StaticPooledPath<S> {
    //     StaticPooledPath::new(self, value)
    // }

    /// Returns a static pooled path, which keeps the pooled path allocated for
    /// the duration of the process. The path is initialized using the function
    /// provided when it is retrieved for the first time.
    pub const fn get_static_with(
        &'static self,
        function: fn() -> Cow<'static, Path>,
    ) -> StaticPooledPath<S> {
        StaticPooledPath::new_fn(self, function)
    }
}

impl<S> GlobalPool<Vec<u8>, S>
where
    S: BuildHasher,
{
    /// Returns a copy of an existing [`GlobalBuffer`] if one is found.
    /// Otherwise, a new [`GlobalBuffer`] is created and returned.
    ///
    /// While any copies of the returned [`GlobalBuffer`] are still allocated,
    /// calling this function is guaranteed to return a copy of the same byte
    /// buffer.
    pub fn get<'a, V>(&'static self, value: V) -> GlobalBuffer<S>
    where
        V: Into<Cow<'a, [u8]>>,
    {
        let value = value.into();
        self.with_active_symbols(|symbols| symbols.get(value, &self))
    }

    /// Returns a static pooled buffer, which keeps the pooled buffer allocated for
    /// the duration of the process.
    ///
    /// The buffer is not initialized until it is retrieved for the first time.
    pub const fn get_static(&'static self, value: &'static [u8]) -> StaticPooledBuffer<S> {
        StaticPooledBuffer::new(self, value)
    }

    /// Returns a static pooled buffer, which keeps the pooled buffer allocated
    /// for the duration of the process. The buffer is initialized using the
    /// function provided when it is retrieved for the first time.
    pub const fn get_static_with(
        &'static self,
        function: fn() -> Cow<'static, [u8]>,
    ) -> StaticPooledBuffer<S> {
        StaticPooledBuffer::new_fn(self, function)
    }
}

/// A lazily-initialized [`GlobalString`] that stays allocated for the duration
/// of the process.
#[derive(Debug)]
pub struct StaticPooledString<S = RandomState>
where
    S: BuildHasher + 'static,
{
    init: StaticStringInit<S>,
    cell: OnceLock<GlobalString<S>>,
}

/// A lazily-initialized [`GlobalBuffer`] that stays allocated for the duration
/// of the process.
#[derive(Debug)]
pub struct StaticPooledBuffer<S = RandomState>
where
    S: BuildHasher + 'static,
{
    init: StaticBufferInit<S>,
    cell: OnceLock<GlobalBuffer<S>>,
}

/// A lazily-initialized [`GlobalPath`] that stays allocated for the duration
/// of the process.
#[derive(Debug)]
pub struct StaticPooledPath<S = RandomState>
where
    S: BuildHasher + 'static,
{
    init: StaticPathInit<S>,
    cell: OnceLock<GlobalPath<S>>,
}

macro_rules! impl_static_pooled {
    ($name:ident, $pooled:ident, $statename:ident, $owned:ty, $borrowed:ty) => {
        impl<S> $name<S>
        where
            S: BuildHasher + 'static,
        {
            #[allow(dead_code)] // This function isn't called for StaticPooledPath, because there's no way to get a static Path.
            const fn new(pool: &'static GlobalPool<$owned, S>, value: &'static $borrowed) -> Self {
                Self {
                    init: $statename::Static(pool, value),
                    cell: OnceLock::new(),
                }
            }

            const fn new_fn(
                pool: &'static GlobalPool<$owned, S>,
                init: fn() -> Cow<'static, $borrowed>,
            ) -> Self {
                Self {
                    init: $statename::Fn(pool, init),
                    cell: OnceLock::new(),
                }
            }

            /// Returns a reference-counted clone of the contained resource.
            ///
            /// If this is the first time the contents are accessed, the value
            /// will be initialized from the pool on the first access. This
            /// requires synchronization and can block the current thraed very
            /// briefly.
            ///
            /// All subsequent accesses will be non-blocking.
            pub fn get(&self) -> &$pooled<S> {
                self.cell.get_or_init(|| match self.init {
                    $statename::Static(pool, value) => pool.get(value).clone(),
                    $statename::Fn(pool, init) => pool.get(init()).clone(),
                })
            }
        }

        #[derive(Debug, Clone, Copy)]
        #[allow(dead_code)] // Path can't use the Static variant.
        enum $statename<S>
        where
            S: BuildHasher + 'static,
        {
            Static(&'static GlobalPool<$owned, S>, &'static $borrowed),
            Fn(
                &'static GlobalPool<$owned, S>,
                fn() -> Cow<'static, $borrowed>,
            ),
        }

        impl<S> std::ops::Deref for $name<S>
        where
            S: BuildHasher,
        {
            type Target = $pooled<S>;

            fn deref(&self) -> &Self::Target {
                self.get()
            }
        }

        impl<S, S2> PartialEq<Pooled<&'static GlobalPool<$owned, S2>, S2>> for $name<S>
        where
            S: BuildHasher,
            S2: BuildHasher,
        {
            fn eq(&self, other: &Pooled<&'static GlobalPool<$owned, S2>, S2>) -> bool {
                self.get() == other
            }
        }

        impl<S, S2> PartialEq<$name<S>> for Pooled<&'static GlobalPool<$owned, S2>, S2>
        where
            S: BuildHasher,
            S2: BuildHasher,
        {
            fn eq(&self, other: &$name<S>) -> bool {
                self == other.get()
            }
        }
    };
}

impl_static_pooled!(
    StaticPooledString,
    GlobalString,
    StaticStringInit,
    String,
    str
);
impl_static_pooled!(
    StaticPooledBuffer,
    GlobalBuffer,
    StaticBufferInit,
    Vec<u8>,
    [u8]
);
impl_static_pooled!(StaticPooledPath, GlobalPath, StaticPathInit, PathBuf, Path);
