# pythonic_global_lock

This crate provides a `GLock<T>`, that is globally locked. Every `GLock<T>` uses the same global lock, so locking on
will lock all. Sounds like a dumb idea? One of the most popular programming implementations does it, so it must be
smart.

```rust
fn main() {
    let lock1 = pythonic_global_lock::GLock::new(1);
    let lock2 = pythonic_global_lock::GLock::new(2);

    {
        let locked1 = lock1.lock();
        println!("{}", &*locked1)
        // locking lock2 here would be a deadlock
    }
}
```