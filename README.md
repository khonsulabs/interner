# interner

![interner forbids unsafe code](https://img.shields.io/badge/unsafe-forbid-success)
[![crate version](https://img.shields.io/crates/v/interner.svg)](https://crates.io/crates/interner)
[![Live Build Status](https://img.shields.io/github/actions/workflow/status/khonsulabs/interner/tests.yml?branch=main)](https://github.com/khonsulabs/interner/actions?query=workflow:Tests)
[![HTML Coverage Report for `main` branch](https://khonsulabs.github.io/interner/coverage/badge.svg)](https://khonsulabs.github.io/interner/coverage/)
[![Documentation](https://img.shields.io/badge/docs-main-informational)](https://docs.rs/interner)

An interning crate for Rust with no dependencies and no unsafe code
(`#![forbid(unsafe_code)]`). Most existing interning crates only offer interning
strings. This crate allows interning paths and byte buffers as well.

## How this crate works

This crate uses a `HashSet` and a `Vec` to store its entries. When a value is
looked up, if it cannot be found in the `HashSet`, it is assigned a slot in the
`Vec`. Future lookups will return a clone to the `Arc`-wrapped data.

When the final reference to a `Pooled<T>` value is dropped, the pool's `HashMap`
has the value removed and the `Vec`'s slot is made available for re-use.

This crate does not do sub-value interning. Each value is stored independently
with its own allocation.

## `Hash` for Pooled Values

The `Pooled<T>` type implements `Hash` by hashing its internal unique id rather
than using `T::hash()`. This allows pooled objects to be used as efficent keys
in hash maps and sets.

Because `T::hash()` is different than `Pooled<T>::hash()`, `Pooled<T>` does not
implement `Borrow<T>`. This prevents using an `&str` to look up a value in a
`HashMap`.

Another important caveat of using `Pooled<T>` values as keys in a hash-based
collection is that all `Pooled<T>` values must be from the same pool. Using
values from separate pools will cause lookups to be unable to find the contained
keys in most situations. When using this unsupported flow, incorrect matches
will never occur because `Pooled<T>::eq()` is implemented to verify the values
are from the same pool, otherwise the underlying values are compared.

## Globally Interned Strings

```rust
use interner::global::{GlobalString, GlobalPool};

static STRINGS: GlobalPool<String> = GlobalPool::new();

let my_string = STRINGS.get("hello");
let other_copy = STRINGS.get(String::from("hello"));

// Both `my_string` and `other_copy` are pointing to the same underlying string.
assert!(GlobalString::ptr_eq(&my_string, &other_copy));
```

## Interned Strings from a StringPool

```rust
use interner::shared::{StringPool, SharedString};

let pool = StringPool::default();
let my_string = pool.get("hello");
let other_copy = pool.get(String::from("hello"));

// Both `my_string` and `other_copy` are pointing to the same underlying string.
assert!(SharedString::ptr_eq(&my_string, &other_copy));
```

## Globally Interned Paths

```rust
use std::path::{Path, PathBuf};
use interner::global::{GlobalPath, GlobalPool};

static PATHS: GlobalPool<PathBuf> = GlobalPool::new();

let my_path = PATHS.get(Path::new("hello"));
let other_copy = PATHS.get(PathBuf::from("hello"));

// Both `my_path` and `other_copy` are pointing to the same underlying path.
assert!(GlobalPath::ptr_eq(&my_path, &other_copy));
```

## Interned Paths from a PathPool

```rust
use std::path::{Path, PathBuf};
use interner::shared::{PathPool, SharedPath};

let pool = PathPool::default();
let my_string = pool.get(Path::new("hello"));
let other_copy = pool.get(PathBuf::from("hello"));

// Both `my_path` and `other_copy` are pointing to the same underlying path.
assert!(SharedPath::ptr_eq(&my_string, &other_copy));
```

## Globally Interned Byte Buffers

```rust
use interner::global::{GlobalBuffer, GlobalPool};

static BUFFERS: GlobalPool<Vec<u8>> = GlobalPool::new();

let my_buffer = BUFFERS.get(&b"hello"[..]);
let other_copy = BUFFERS.get(b"hello".to_vec());

// Both `my_buffer` and `other_copy` are pointing to the same underlying path.
assert!(GlobalBuffer::ptr_eq(&my_buffer, &other_copy));
```

## Interned Byte Buffers from a BufferPool

```rust
use interner::shared::{BufferPool, SharedBuffer};

let pool = BufferPool::default();
let my_buffer = pool.get(&b"hello"[..]);
let other_copy = pool.get(b"hello".to_vec());

// Both `my_path` and `other_copy` are pointing to the same underlying path.
assert!(SharedBuffer::ptr_eq(&my_buffer, &other_copy));
```

## Open-source Licenses

This project, like all projects from [Khonsu Labs](https://khonsulabs.com/), are
open-source. This repository is available under the [MIT License](./LICENSE-MIT)
or the [Apache License 2.0](./LICENSE-APACHE).

To learn more about contributing, please see [CONTRIBUTING.md](./CONTRIBUTING.md).
