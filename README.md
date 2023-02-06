# interner

A interning crate for Rust with no dependencies by default and no unsafe code
(`#![forbid(unsafe_code)]`). Most existing interning crates only offer interning
strings. This crate allows interning paths and byte buffers as well.

## Globally Interned Strings

```rust
use interner::GlobalString;

let my_string = GlobalString::from("hello");
let other_copy = GlobalString::from(String::from("hello"));

// Both `my_string` and `other_copy` are pointing to the same underlying string.
assert!(GlobalString::ptr_eq(&my_string, &other_copy));
```

## Interned Strings from a StringPool

```rust
use interner::shared::{StringPool, SharedString};

let pool = StringPool::default();
let my_string = pool.get("hello");
let other_copy = pool.get_from_owned(String::from("hello"));

// Both `my_string` and `other_copy` are pointing to the same underlying string.
assert!(SharedString::ptr_eq(&my_string, &other_copy));
```

## Globally Interned Paths

```rust
use std::path::{Path, PathBuf};
use interner::GlobalPath;

let my_path = GlobalPath::from(Path::new("hello"));
let other_copy = GlobalPath::from(PathBuf::from("hello"));

// Both `my_path` and `other_copy` are pointing to the same underlying path.
assert!(GlobalPath::ptr_eq(&my_path, &other_copy));
```

## Interned Paths from a PathPool

```rust
use std::path::{Path, PathBuf};
use interner::shared::{PathPool, SharedPath};

let pool = PathPool::default();
let my_string = pool.get(Path::new("hello"));
let other_copy = pool.get_from_owned(PathBuf::from("hello"));

// Both `my_path` and `other_copy` are pointing to the same underlying path.
assert!(SharedPath::ptr_eq(&my_string, &other_copy));
```

## Globally Interned Byte Buffers

```rust
use interner::GlobalBuffer;

let my_buffer = GlobalBuffer::from(b"hello");
let other_copy = GlobalBuffer::from(b"hello".to_vec());

// Both `my_buffer` and `other_copy` are pointing to the same underlying path.
assert!(GlobalBuffer::ptr_eq(&my_buffer, &other_copy));
```

## Interned Byte Buffers from a BufferPool

```rust
use interner::shared::{BufferPool, SharedBuffer};

let pool = BufferPool::default();
let my_buffer = pool.get(b"hello");
let other_copy = pool.get_from_owned(b"hello".to_vec());

// Both `my_path` and `other_copy` are pointing to the same underlying path.
assert!(SharedBuffer::ptr_eq(&my_buffer, &other_copy));
```
