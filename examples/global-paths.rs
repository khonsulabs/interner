use std::borrow::Cow;
use std::path::Path;

use interner::global::{GlobalPath, PathPool, StaticPooledPath};

static PATH_POOL: PathPool = PathPool::new();
// Because there is no way to get a Path in a const context, we have to use the
// lazy option, which is a little more verbose due to it accepting both owned or
static STATIC_PATH: StaticPooledPath = PATH_POOL.get_static_with(|| Cow::Borrowed(Path::new("a")));

fn main() {
    // Get a path from the static instance. This will keep the pooled path
    // alive for the duration of the process.
    STATIC_PATH.get();
    // Request the same path directly from the pool.
    let a_again = PATH_POOL.get(Path::new("a"));

    // The two instances are pointing to the same instance.
    assert!(GlobalPath::ptr_eq(&*STATIC_PATH, &a_again));

    // Verify the pool still contains "a" even after dropping our local
    // instances. This is due to STATIC_PATH still holding a reference.
    drop(a_again);
    let pooled: Vec<GlobalPath> = PATH_POOL.pooled();
    assert_eq!(pooled.len(), 1);
    assert_eq!(pooled[0], Path::new("a"));
}

#[test]
fn runs() {
    main();
}
