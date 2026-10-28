use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

/// A simple Spinlock wrapping inner data `T`.
pub struct Spinlock<T> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
}

// Safety: Spinlock provides exclusive access (&mut T) via its lock acquisition,
// making it safe to share across thread boundaries as long as T is Send.
unsafe impl<T: Send> Sync for Spinlock<T> {}

impl<T> Spinlock<T> {
    pub fn new(data: T) -> Self {
        Spinlock {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    /// Acquires the spinlock, executing the closure `f` with exclusive access to the data.
    pub fn lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {

        // 1. Acquire the lock: CAS false -> true
        while self
            .locked

            // The atomic method checks and updates the boolean variable in a single, 
            // indivisible hardware operation:

            //   Expected Value: false (Checks: "Is the lock currently free?")
            //   New Value:      true (Action: "If free, change it to locked!")

            // Ordering::Acquire (On Success)
            //   All writes performed by the previous lock holder are completely 
            //   visible to this thread before it starts touching the inner data
            
            // Ordering::Relaxed (On Failure)

            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)

            // compare_exchange returns a Result<bool, bool>:

            //   Ok(false): The value was indeed false. The operation succeeded, atomically
            //              setting locked = true
            //   Err(true): Another thread was holding the lock (locked was true). The 
            //              operation failed, and locked remains true

            .is_err()
        {
            // If .is_err() is true (lock was held by another thread), the while loop 
            // continues to run ("spins")

            // Spin-wait: hint to the CPU hardware scheduler that we are in a tight spin loop
            std::hint::spin_loop();
        }

        // 2. Critical Section: Safe to dereference UnsafeCell because we hold exclusive lock
        let result = unsafe { f(&mut *self.data.get()) };

        // 3. Release the lock: store false with Release ordering
        self.locked.store(false, Ordering::Release);

        result
    }
}

fn main() {
    // Wrap counter in our custom Spinlock and share across threads using Arc
    let counter = Arc::new(Spinlock::new(0));
    let mut handles = vec![];

    // Spawn 10 threads, each incrementing the counter 1,000 times
    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1_000 {
                counter_clone.lock(|val| {
                    *val += 1;
                });
            }
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Print final result
    counter.lock(|val| {
        println!("Final Counter Value: {}", *val);
        assert_eq!(*val, 10_000);
    });
}
