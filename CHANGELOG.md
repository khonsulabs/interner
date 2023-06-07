# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Changed

- `Pooled` now implements `Debug` by passing through to the underlying type's
  `Debug` implementation. Previously, it included extra information about which
  pool it belonged to.

## v0.2.0

### Breaking Changes

- The MSRV is now 1.70.0

### Added

- `StaticPooledString`/`StaticPooledBuffer`/`StaticPooledPath` can now be
  directly compared against their non-static counterparts, rather than requiring
  a `.get()` to do the comparison.

### Changed

- Internally, the pools now stored boxed representations (e.g., `Box<str>`).
  This reduces the pool's internal storage overhead by one usize per stored
  element.

- Internally, `StaticPooledString`, `StaticPooledBuffer`, and `StaticPooledPath`
  now use `std::sync::OnceLock`. This allows these types to implement `Deref`
  into their global counterparts.

## v0.1.1

### Fixed

- `Pooled<T,S>`'s Debug implementation no longer prints the entire pool's state.
  Instead, the new debug implementation prints the pooled value, the index in
  the pool, and the address of the pool. Here's an example from the unit test:

  `Pooled { value: "test", index: 0, pool: 0x7f1480000d10 }`

### Added

- `StaticPooledString`, `StaticPooledBuffer`, and `StaticPooledPath` have all
  been added. These types can be used to create statically pooled values that
  initialize upon access and never release the pooled value. These values are
  created by using `get_static` or `get_static_with` on the global pool type in
  question.
- All pool types now have a `pooled()` function to retrieve the currently pooled
  items.
