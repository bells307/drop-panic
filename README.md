# drop-panic

The callback that will be called if the current thread panics.

## Example
```rust
let panicked = Arc::new(AtomicBool::new(false));

let jh = thread::spawn({
    let panicked = Arc::clone(&panicked);
    move || {
        drop_panic! {
            panicked.store(true, Ordering::Release);
        };

        panic!("boom");
    }
});

assert!(jh.join().is_err());
assert!(panicked.load(Ordering::Acquire));
```
