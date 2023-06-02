use std::path::{Path, PathBuf};

use interner::shared::{PathPool, SharedPath};

fn main() {
    let pool = PathPool::default();

    // Get a value from the pool.
    let a = pool.get(PathBuf::from("a"));
    // Request it again.
    let a_again = pool.get(Path::new("a"));

    // Verify that the paths are the same underlying allocation.
    assert!(SharedPath::ptr_eq(&a, &a_again));

    // Once all of our instances are dropped, the value should be freed.
    drop(a);
    drop(a_again);
    let all: Vec<SharedPath> = pool.pooled();
    assert!(all.is_empty());
}

#[test]
fn runs() {
    main();
}
