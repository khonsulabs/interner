use interner::global::{GlobalString, StaticPooledString, StringPool};

static STRING_POOL: StringPool = StringPool::new();
static STATIC_STRING: StaticPooledString = STRING_POOL.get_static("a");

fn main() {
    // Get a string from the static instance. This will keep the pooled string
    // alive for the duration of the process.
    let a = STATIC_STRING.get();
    // Request the same string directly from the pool.
    let a_again = STRING_POOL.get("a");

    // The two instances are pointing to the same instance.
    assert!(GlobalString::ptr_eq(&a, &a_again));

    // Verify the pool still contains "a" even after dropping our local
    // instances. This is due to STATIC_STRING still holding a reference.
    drop(a_again);
    drop(a);
    let pooled: Vec<GlobalString> = STRING_POOL.pooled();
    assert_eq!(pooled.len(), 1);
    assert_eq!(pooled[0], "a");
}

#[test]
fn runs() {
    main();
}
