#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![warn(
    clippy::cargo,
    missing_docs,
    // clippy::missing_docs_in_private_items,
    clippy::pedantic,
    future_incompatible,
    rust_2018_idioms,
)]
#![allow(
    clippy::missing_errors_doc,
    clippy::option_if_let_else,
    clippy::module_name_repetitions
)]

use std::borrow::Cow;
use std::fmt::{Debug, Display};
use std::hash::{BuildHasher, Hash};
use std::marker::PhantomData;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

mod pool;
/// Shared interning pools that have no global state.
pub mod shared;
#[cfg(test)]
mod tests;

use crate::pool::{Pool, PoolKindSealed, SharedData};

#[cfg(feature = "fnv")]
use fnv::FnvBuildHasher as DefaultHasher;
#[cfg(not(feature = "fnv"))]
use std::collections::hash_map::RandomState as DefaultHasher;

/// A pooled string that is stored in a [`GlobalPool`].
///
/// This type implements `From<String>` and `From<&str>`.
pub type GlobalString = Pooled<GlobalPool<String>, DefaultHasher>;
/// A pooled path that is stored in a [`GlobalPool`].
///
/// This type implements `From<PathBuf>` and `From<&Path>`.
pub type GlobalPath = Pooled<GlobalPool<PathBuf>, DefaultHasher>;
/// A pooled buffer (`Vec<u8>`) that is stored in a [`GlobalPool`].
///
/// This type implements `From<Vec<u8>>` and `From<&[u8]>`.
pub type GlobalBuffer = Pooled<GlobalPool<Vec<u8>>, DefaultHasher>;

/// A kind of interning pool. Currently there are only two types of pools:
///
/// - Global, used through the [`GlobalString`], [`GlobalPath`], and
///   [`GlobalBuffer`] types.
/// - Shared, used through the [`StringPool`](shared::StringPool),
///   [`PathPool`](shared::PathPool), and [`BufferPool`](shared::BufferPool)
///   types.
pub trait PoolKind<S>: Clone + PartialEq + PoolKindSealed<S> {}

/// A global interning pool.
///
/// This type isn't used directly. Instead, use the [`GlobalString`],
/// [`GlobalPath`], and [`GlobalBuffer`] to intern values in shared global
/// pools.
#[derive(Clone, Debug)]
pub struct GlobalPool<T>(PhantomData<T>);

impl GlobalPool<String> {
    #[must_use]
    const fn strings() -> Self {
        Self(PhantomData)
    }
}

impl GlobalPool<PathBuf> {
    #[must_use]
    const fn paths() -> Self {
        Self(PhantomData)
    }
}

impl GlobalPool<Vec<u8>> {
    #[must_use]
    const fn buffers() -> Self {
        Self(PhantomData)
    }
}

trait GlobalPoolAccess:
    Sized + Debug + Clone + Eq + PartialEq + Hash + Ord + PartialOrd + 'static
where
    GlobalPool<Self>: PoolKind<DefaultHasher, Stored = Self>,
{
    fn storage() -> &'static GlobalPoolStorage<Self>;
}

#[derive(Debug)]
struct GlobalPoolStorage<T>(Mutex<Option<Pool<GlobalPool<T>, DefaultHasher>>>)
where
    GlobalPool<T>: PoolKind<DefaultHasher>;

impl<T> GlobalPoolStorage<T>
where
    GlobalPool<T>: PoolKind<DefaultHasher>,
{
    pub const fn new() -> Self {
        Self(Mutex::new(None))
    }
}

impl GlobalPoolAccess for String {
    fn storage() -> &'static GlobalPoolStorage<Self> {
        static GLOBAL_STRINGS: GlobalPoolStorage<String> = GlobalPoolStorage::new();
        &GLOBAL_STRINGS
    }
}

impl GlobalPoolAccess for PathBuf {
    fn storage() -> &'static GlobalPoolStorage<Self> {
        static GLOBAL_PATHS: GlobalPoolStorage<PathBuf> = GlobalPoolStorage::new();
        &GLOBAL_PATHS
    }
}

impl GlobalPoolAccess for Vec<u8> {
    fn storage() -> &'static GlobalPoolStorage<Self> {
        static GLOBAL_BUFFERS: GlobalPoolStorage<Vec<u8>> = GlobalPoolStorage::new();
        &GLOBAL_BUFFERS
    }
}

impl<T> PoolKind<DefaultHasher> for GlobalPool<T> where T: GlobalPoolAccess {}

impl<T> PartialEq for GlobalPool<T> {
    fn eq(&self, _other: &GlobalPool<T>) -> bool {
        true
    }
}

impl<T> PoolKindSealed<DefaultHasher> for GlobalPool<T>
where
    T: GlobalPoolAccess,
{
    type Stored = T;

    fn with_active_symbols<R>(&self, logic: impl FnOnce(&mut Pool<Self, DefaultHasher>) -> R) -> R {
        let mut symbols = T::storage().0.lock().expect("poisoned");
        if symbols.is_none() {
            *symbols = Some(Pool::default());
        }

        logic(symbols.as_mut().expect("always initialized"))
    }
}

/// A type that ensures only one copy of each value exists in its pool, enabling
/// quicker lookups by not requiring full comparisons.
///
/// After all instances of a given [`Pooled`] are dropped, the underlying
/// storage is released.
///
/// This type's [`Hash`] implementation is different than the wrapped type's
/// hash implementation. This type avoids implementing `Borrow<T>` to prevent
/// using incompatible [`Hash`] implementations to look up values in
/// `HashMap`s/`HashSet`s where this type is
/// used as the key.
#[derive(Debug)]
pub struct Pooled<P, S>(SharedData<P, S>)
where
    P: PoolKind<S>,
    S: BuildHasher;

impl<P, S> Pooled<P, S>
where
    P: PoolKind<S>,
    S: BuildHasher,
{
    /// Returns true if `this` and `other` point to the exact same instance of
    /// the value. Returns false if `this` and `other` are from different pools
    /// or if the index within the pool does not match.
    ///
    /// This function never compares the contents of the contained values.
    #[must_use]
    pub fn ptr_eq(this: &Self, other: &Self) -> bool {
        this.0 .0.pool == other.0 .0.pool && this.0 .0.index == other.0 .0.index
    }
}

impl<P, S> Clone for Pooled<P, S>
where
    P: PoolKind<S>,
    S: BuildHasher,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<P, S> Hash for Pooled<P, S>
where
    P: PoolKind<S>,
    S: BuildHasher,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0 .0.index.hash(state);
    }
}

impl<P, S> Eq for Pooled<P, S>
where
    P: PoolKind<S>,
    S: BuildHasher,
{
}

impl<PSelf, POther, SSelf, SOther, T> PartialEq<Pooled<POther, SOther>> for Pooled<PSelf, SSelf>
where
    PSelf: PoolKind<SSelf, Stored = T> + PartialEq<POther>,
    POther: PoolKind<SOther, Stored = T>,
    T: PartialEq,
    SSelf: BuildHasher,
    SOther: BuildHasher,
{
    fn eq(&self, other: &Pooled<POther, SOther>) -> bool {
        if self.0 .0.pool == other.0 .0.pool {
            self.0 .0.index == other.0 .0.index
        } else {
            **self == **other
        }
    }
}

impl<P, S> Display for Pooled<P, S>
where
    P: PoolKind<S>,
    P::Stored: Display,
    S: BuildHasher,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&**self, f)
    }
}

impl<'a> From<&'a str> for GlobalString {
    fn from(sym: &'a str) -> Self {
        GlobalPool::strings()
            .with_active_symbols(|symbols| symbols.get(Cow::Borrowed(sym), &GlobalPool::strings()))
    }
}

impl From<String> for GlobalString {
    fn from(sym: String) -> Self {
        GlobalPool::strings().with_active_symbols(|symbols| {
            symbols.get::<str>(Cow::Owned(sym), &GlobalPool::strings())
        })
    }
}

impl<'a> From<&'a str> for GlobalPath {
    fn from(sym: &'a str) -> Self {
        Self::from(Path::new(sym))
    }
}

impl<'a> From<&'a Path> for GlobalPath {
    fn from(sym: &'a Path) -> Self {
        GlobalPool::paths()
            .with_active_symbols(|symbols| symbols.get(Cow::Borrowed(sym), &GlobalPool::paths()))
    }
}

impl From<PathBuf> for GlobalPath {
    fn from(sym: PathBuf) -> Self {
        GlobalPool::paths().with_active_symbols(|symbols| {
            symbols.get::<Path>(Cow::Owned(sym), &GlobalPool::paths())
        })
    }
}

impl<'a> From<&'a [u8]> for GlobalBuffer {
    fn from(sym: &'a [u8]) -> Self {
        GlobalPool::buffers()
            .with_active_symbols(|symbols| symbols.get(Cow::Borrowed(sym), &GlobalPool::buffers()))
    }
}

impl From<Vec<u8>> for GlobalBuffer {
    fn from(sym: Vec<u8>) -> Self {
        GlobalPool::buffers().with_active_symbols(|symbols| {
            symbols.get::<[u8]>(Cow::Owned(sym), &GlobalPool::buffers())
        })
    }
}

impl<'a, const N: usize> From<&'a [u8; N]> for GlobalBuffer {
    fn from(sym: &'a [u8; N]) -> Self {
        Self::from(&sym[0..N])
    }
}

impl<P, S> Deref for Pooled<P, S>
where
    P: PoolKind<S>,
    S: BuildHasher,
{
    type Target = P::Stored;

    fn deref(&self) -> &Self::Target {
        &self.0 .0.value
    }
}

impl<P, S> PartialEq<str> for Pooled<P, S>
where
    P: PoolKind<S, Stored = String>,
    S: BuildHasher,
{
    fn eq(&self, other: &str) -> bool {
        **self == other
    }
}

impl<'a, P, S> PartialEq<&'a str> for Pooled<P, S>
where
    P: PoolKind<S, Stored = String>,
    S: BuildHasher,
{
    fn eq(&self, other: &&'a str) -> bool {
        self == *other
    }
}

impl<P, S> PartialEq<[u8]> for Pooled<P, S>
where
    P: PoolKind<S, Stored = Vec<u8>>,
    S: BuildHasher,
{
    fn eq(&self, other: &[u8]) -> bool {
        **self == other
    }
}

impl<'a, P, S> PartialEq<&'a [u8]> for Pooled<P, S>
where
    P: PoolKind<S, Stored = Vec<u8>>,
    S: BuildHasher,
{
    fn eq(&self, other: &&'a [u8]) -> bool {
        self == *other
    }
}

impl<P, S> PartialEq<Path> for Pooled<P, S>
where
    P: PoolKind<S, Stored = PathBuf>,
    S: BuildHasher,
{
    fn eq(&self, other: &Path) -> bool {
        **self == other
    }
}

impl<'a, P, S> PartialEq<&'a Path> for Pooled<P, S>
where
    P: PoolKind<S, Stored = PathBuf>,
    S: BuildHasher,
{
    fn eq(&self, other: &&'a Path) -> bool {
        self == *other
    }
}
