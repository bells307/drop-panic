use std::thread;

/// The callback that will be called if the current thread panics
pub struct DropPanic<F>(F)
where
    F: Fn();

impl<F> DropPanic<F>
where
    F: Fn(),
{
    pub fn new(panic_fn: F) -> Self {
        Self(panic_fn)
    }
}

impl<F> Drop for DropPanic<F>
where
    F: Fn(),
{
    fn drop(&mut self) {
        if thread::panicking() {
            (self.0)();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        sync::{
            atomic::{AtomicBool, Ordering},
            Arc,
        },
        thread,
    };

    use super::DropPanic;

    #[test]
    fn test_drop_panic() {
        let panicked = Arc::new(AtomicBool::new(false));

        let jh = thread::spawn({
            let panicked = Arc::clone(&panicked);
            move || {
                let _dp = DropPanic::new(|| panicked.store(true, Ordering::Release));
                panic!("boom");
            }
        });

        assert!(jh.join().is_err());
        assert!(panicked.load(Ordering::Acquire));
    }
}
