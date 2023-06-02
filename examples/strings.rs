use interner::shared::{SharedString, StringPool};

fn main() {
    let pool = StringPool::default();

    // Get a value from the pool.
    let a = pool.get(String::from("a"));
    // Request it again.
    let a_again = pool.get("a");

    // Verify that the strings are the same underlying allocation.
    assert!(SharedString::ptr_eq(&a, &a_again));

    // Once all of our instances are dropped, the value should be freed.
    drop(a);
    drop(a_again);
    let all: Vec<SharedString> = pool.pooled();
    assert!(all.is_empty());
}

#[test]
fn runs() {
    main();
}
