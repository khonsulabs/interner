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

use std::fmt::{Debug, Display};
use std::hash::{BuildHasher, Hash};
use std::ops::Deref;
use std::path::{Path, PathBuf};

/// Global interning pools.
pub mod global;
mod pool;
/// Shared interning pools that have no global state.
pub mod shared;
#[cfg(test)]
mod tests;

use crate::pool::{PoolKindSealed, SharedData};

/// A kind of interning pool. Currently there are only two types of pools:
///
/// - Global, used through the [`global::StringPool`],
///   [`GlobalPath`](global::GlobalPath), and
///   [`GlobalBuffer`](global::GlobalBuffer) types.
/// - Shared, used through the [`StringPool`](shared::StringPool),
///   [`PathPool`](shared::PathPool), and [`BufferPool`](shared::BufferPool)
///   types.
pub trait PoolKind<S>: Clone + PartialEq + PoolKindSealed<S> {}

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
    pub fn ptr_eq<P2, S2>(this: &Self, other: &Pooled<P2, S2>) -> bool
    where
        P: PartialEq<P2>,
        P2: PoolKind<S2>,
        S2: BuildHasher,
    {
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

impl<P, S> Debug for Pooled<P, S>
where
    P: PoolKind<S>,
    P::Stored: Debug,
    S: BuildHasher,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pooled")
            .field("value", &**self)
            .field("index", &self.0 .0.index)
            .field("pool", &self.0 .0.pool.address_of())
            .finish()
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
