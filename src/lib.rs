#[macro_export]
macro_rules! drop_panic {
    ($($t:tt)*) => {
        let _drop_panic = $crate::guard(|| { $($t)* });
    };
}

pub fn guard<F>(f: F) -> DropPanic<F>
where
    F: Fn(),
{
    DropPanic::new(f)
}

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
        if std::thread::panicking() {
            (self.0)();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::drop_panic;
    use std::{
        sync::{
            atomic::{AtomicBool, Ordering},
            Arc,
        },
        thread,
    };

    #[test]
    fn test_drop_panic() {
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
    }
}
