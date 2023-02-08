use interner::global::{BufferPool, GlobalBuffer, StaticPooledBuffer};

static BUFFER_POOL: BufferPool = BufferPool::new();
static STATIC_STRING: StaticPooledBuffer = BUFFER_POOL.get_static(b"a");

fn main() {
    // Get a string from the static instance. This will keep the pooled string
    // alive for the duration of the process.
    let a = STATIC_STRING.get();
    // Request the same string directly from the pool.
    let a_again = BUFFER_POOL.get(&b"a"[..]);

    // The two instances are pointing to the same instance.
    assert!(GlobalBuffer::ptr_eq(&a, &a_again));

    // Verify the pool still contains "a" even after dropping our local
    // instances. This is due to STATIC_STRING still holding a reference.
    drop(a_again);
    drop(a);
    let pooled: Vec<GlobalBuffer> = BUFFER_POOL.pooled();
    assert_eq!(pooled.len(), 1);
    assert_eq!(pooled[0], &b"a"[..]);
}

#[test]
fn runs() {
    main();
}
