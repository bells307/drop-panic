# drop-panic

The callback that will be called if the current thread panics.

## Example
```rust
let panicked = Arc::new(AtomicBool::new(false));

let jh = std::thread::spawn({
    let panicked = Arc::clone(&panicked);
    move || {
        let _dp = DropPanic::new(|| panicked.store(true, Ordering::Release));
        panic!("boom");
    }
});

assert!(jh.join().is_err());
assert!(panicked.load(Ordering::Acquire));
```
