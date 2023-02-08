use interner::shared::{BufferPool, SharedBuffer};

fn main() {
    let pool = BufferPool::default();

    // Get a value from the pool.
    let a = pool.get(vec![b'a']);
    // Request it again.
    let a_again = pool.get(&b"a"[..]);

    // Verify that the strings are the same underlying allocation.
    assert!(SharedBuffer::ptr_eq(&a, &a_again));

    // Once all of our instances are dropped, the value should be freed.
    drop(a);
    drop(a_again);
    let all: Vec<SharedBuffer> = pool.pooled();
    assert!(all.is_empty());
}

#[test]
fn runs() {
    main();
}
