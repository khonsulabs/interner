use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::hash::{BuildHasher, Hasher};
use std::path::{Path, PathBuf};
use std::thread;

use crate::global::GlobalPool;
use crate::pool::PoolKindSealed;
use crate::shared::{SharedPool, SharedString, StringPool};
use crate::Pooled;

static GLOBAL_STRINGS: GlobalPool<String> = GlobalPool::new();
static GLOBAL_PATHS: GlobalPool<PathBuf> = GlobalPool::new();
static GLOBAL_BUFFERS: GlobalPool<Vec<u8>> = GlobalPool::new();

#[test]
fn basics() {
    let first_symbol = GLOBAL_STRINGS.get("basics-test-symbol");
    let slot = first_symbol.0 .0.index;
    let first_again = GLOBAL_STRINGS.get(String::from("basics-test-symbol"));
    assert_eq!(slot, first_again.0 .0.index);
    assert_eq!(first_symbol, first_again);
    assert_eq!(first_symbol, "basics-test-symbol");
    assert_eq!(first_symbol.to_string(), "basics-test-symbol");
    drop(first_again);
    // Dropping the second copy shouldn't free the underlying symbol
    (&GLOBAL_STRINGS).with_active_symbols(|symbols| {
        assert!(symbols.active.contains("basics-test-symbol"));
        assert!(!symbols.slots.is_empty());
        assert!(symbols.slots[slot].is_some());
        assert!(!symbols.free_slots.iter().any(|free| *free == slot));
    });
    drop(first_symbol);
    (&GLOBAL_STRINGS).with_active_symbols(|symbols| {
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
    let first_symbol = GLOBAL_STRINGS.get("shared-is-separate-ignored");
    let from_global = GLOBAL_STRINGS.get("shared_is_separate");
    // Create our local pool and request the same string.
    let shared = SharedPool::<String>::default();
    let from_shared = shared.get(String::from("shared_is_separate"));
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
    let first_symbol = GLOBAL_PATHS.get(PathBuf::from("ignored-global-path"));
    assert_eq!(first_symbol, Path::new("ignored-global-path"));
    let from_global = GLOBAL_PATHS.get(Path::new("shared_is_separate_path"));
    let shared = SharedPool::<PathBuf>::default();
    let from_shared = shared.get(PathBuf::from("shared_is_separate_path"));
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
    let first_symbol = GLOBAL_BUFFERS.get(b"ignored-global-buffer".to_vec());
    assert_eq!(first_symbol, &b"ignored-global-buffer"[..]);
    let from_global = GLOBAL_BUFFERS.get(&b"shared_is_separate_buffer"[..]);
    let shared = SharedPool::<Vec<u8>>::default();
    let from_shared = shared.get(b"shared_is_separate_buffer".to_vec());
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
                let _ = GLOBAL_STRINGS.get("multithreaded");
            }
        }));
    }
    for t in threads {
        t.join().unwrap();
    }
    // The failure case for the code would end up not freing the string.
    (&GLOBAL_STRINGS).with_active_symbols(|symbols| {
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

#[test]
fn custom_global_pool() {
    #[derive(Default, Clone)]
    struct BadHasher(u8);
    impl Hasher for BadHasher {
        fn finish(&self) -> u64 {
            u64::from(self.0)
        }

        fn write(&mut self, bytes: &[u8]) {
            for byte in bytes {
                self.0 ^= *byte;
            }
        }
    }

    impl BuildHasher for BadHasher {
        type Hasher = BadHasher;

        fn build_hasher(&self) -> Self::Hasher {
            self.clone()
        }
    }

    static CUSTOM_POOL: GlobalPool<String, BadHasher> = GlobalPool::with_hasher(BadHasher(0));

    let from_custom = CUSTOM_POOL.get("hello");
    let global = GLOBAL_STRINGS.get("hello");
    assert!(!Pooled::ptr_eq(&from_custom, &global));
}

#[test]
fn pooled_debug() {
    let shared_pool = StringPool::default();
    let string = shared_pool.get("test");

    let debugged = format!("{string:?}");
    println!("{debugged}");
    let expected = format!(
        "Pooled {{ value: \"test\", index: {:?}, pool: {:?} }}",
        string.0 .0.index,
        string.0 .0.pool.address_of()
    );
    assert_eq!(debugged, expected);

    let second = shared_pool.get("test");
    let second_debugged = format!("{second:?}");
    assert_eq!(second_debugged, debugged);

    let other_pool = StringPool::default();

    let third = other_pool.get("test");
    let third_debugged = format!("{third:?}");
    assert_ne!(third_debugged, debugged);
}
