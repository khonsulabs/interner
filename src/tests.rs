use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::thread;

use crate::pool::PoolKindSealed;
use crate::shared::{SharedPool, SharedString, StringPool};
use crate::{GlobalBuffer, GlobalPath, GlobalPool, GlobalString, Pooled};

#[test]
fn basics() {
    let first_symbol = GlobalString::from("basics-test-symbol");
    let slot = first_symbol.0 .0.index;
    let first_again = GlobalString::from(String::from("basics-test-symbol"));
    assert_eq!(slot, first_again.0 .0.index);
    assert_eq!(first_symbol, first_again);
    assert_eq!(first_symbol, "basics-test-symbol");
    assert_eq!(first_symbol.to_string(), "basics-test-symbol");
    drop(first_again);
    // Dropping the second copy shouldn't free the underlying symbol
    GlobalPool::strings().with_active_symbols(|symbols| {
        assert!(symbols.active.contains("basics-test-symbol"));
        assert!(!symbols.slots.is_empty());
        assert!(symbols.slots[slot].is_some());
        assert!(!symbols.free_slots.iter().any(|free| *free == slot));
    });
    drop(first_symbol);
    GlobalPool::strings().with_active_symbols(|symbols| {
        assert!(!symbols.active.contains("basics-test-symbol"));
        match &symbols.slots[slot] {
            Some(new_symbol) => {
                // This test isn't run in isolation, so other symbols may get
                // registered between the drop and this block. Very unlikely,
                // but possible.
                assert_ne!(new_symbol, "basics-test-symbol");
            }
            None => {
                assert!(symbols.free_slots.iter().any(|free| *free == slot));
            }
        }
    });
}

#[test]
fn shared_is_separate() {
    // First get a global string to ensure that we get a non-zero index for the
    // string that will have the same contents as the local pool.
    let first_symbol = GlobalString::from("shared-is-separate-ignored");
    let from_global = GlobalString::from("shared_is_separate");
    // Create our local pool and request the same string.
    let shared = SharedPool::<String>::default();
    let from_shared = shared.get_from_owned(String::from("shared_is_separate"));
    // Verify that the strings are indeed different.
    assert!(!Pooled::ptr_eq(&from_shared, &from_global));
    let from_shared_borrowed = shared.get("shared_is_separate");
    assert!(Pooled::ptr_eq(&from_shared, &from_shared_borrowed));

    // Test both directions of partialeq
    assert_eq!(from_shared, from_global);
    assert_eq!(from_global, from_shared);

    // And test not equal against the first symbol, despite the indexes
    // potentialy being equal (but not guaranteed since other tests can be
    // running simultaneously).
    assert_ne!(first_symbol, from_shared);
    assert_ne!(from_shared, first_symbol);
}

#[test]
fn paths() {
    let first_symbol = GlobalPath::from(PathBuf::from("ignored-global-path"));
    assert_eq!(first_symbol, Path::new("ignored-global-path"));
    let from_global = GlobalPath::from("shared_is_separate_path");
    let shared = SharedPool::<PathBuf>::default();
    let from_shared = shared.get_from_owned(PathBuf::from("shared_is_separate_path"));
    assert_ne!(from_shared.0 .0.index, from_global.0 .0.index);
    let from_shared_borrowed = shared.get(Path::new("shared_is_separate_path"));
    assert_eq!(from_shared.0 .0.index, from_shared_borrowed.0 .0.index);

    // Test both directions of partialeq
    assert_eq!(from_shared, from_global);
    assert_eq!(from_global, from_shared);

    assert_ne!(first_symbol, from_shared);
    assert_ne!(from_shared, first_symbol);
}
#[test]
fn buffers() {
    let first_symbol = GlobalBuffer::from(b"ignored-global-buffer".to_vec());
    assert_eq!(first_symbol, &b"ignored-global-buffer"[..]);
    let from_global = GlobalBuffer::from(b"shared_is_separate_buffer");
    let shared = SharedPool::<Vec<u8>>::default();
    let from_shared = shared.get_from_owned(b"shared_is_separate_buffer".to_vec());
    assert_ne!(from_shared.0 .0.index, from_global.0 .0.index);
    let from_shared_borrowed = shared.get(&b"shared_is_separate_buffer"[..]);
    assert_eq!(from_shared.0 .0.index, from_shared_borrowed.0 .0.index);

    // Test both directions of partialeq
    assert_eq!(from_shared, from_global);
    assert_eq!(from_global, from_shared);

    assert_ne!(first_symbol, from_shared);
    assert_ne!(from_shared, first_symbol);
}

#[test]
fn hashing() {
    let mut set = HashSet::new();
    let shared = StringPool::default();
    set.insert(shared.get("hello"));
    assert!(set.contains(&shared.get("hello")));
    assert!(!set.contains(&shared.get("world")));
}

#[test]
fn with_hasher() {
    let mut set = HashSet::new();
    let shared = StringPool::with_hasher(RandomState::default());
    set.insert(shared.get("hello"));
    assert!(set.contains(&shared.get("hello")));
    assert!(!set.contains(&shared.get("world")));
}

#[test]
fn multithreaded_reaquire() {
    // We have an edge case code path that's hard to test. In SharedData::drop,
    // there is a path that gets hit only when other threads are getting a new
    // reference to the string being dropped.
    let mut threads = Vec::new();
    for _ in 0..4 {
        threads.push(thread::spawn(|| {
            for _ in 0..1000 {
                let _ = GlobalString::from("multithreaded");
            }
        }));
    }
    for t in threads {
        t.join().unwrap();
    }
    // The failure case for the code would end up not freing the string.
    GlobalPool::strings().with_active_symbols(|symbols| {
        assert!(!symbols.active.contains("multithreaded"));
    });
}

#[test]
fn ptr_eq() {
    let pool_a = StringPool::default();
    let from_a = pool_a.get("hello");
    let pool_b = StringPool::default();
    let from_b = pool_b.get("hello");
    assert_eq!(from_a, from_b);
    assert!(!SharedString::ptr_eq(&from_a, &from_b));

    assert!(SharedString::ptr_eq(&from_a, &pool_a.get("hello")));
}
