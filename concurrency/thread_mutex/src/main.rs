use std::cmp::max;
use std::mem::{align_of, size_of};
use std::ops::{Deref, DerefMut};
use std::ptr;
use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use std::time::Duration;

struct Carton<T>(ptr::NonNull<T>);

impl<T> Carton<T> {
    pub fn new(value: T) -> Self {
        
        // Allocate enough memory on the heap to store one T.
        assert_ne!(size_of::<T>(), 0, "Zero-sized types are out of the scope of this example");

        // Initialize a mutable raw pointer variable (memptr) of type *mut T and 
        // sets its initial value to a null pointer

        let mut memptr: *mut T = ptr::null_mut();

        unsafe {
            
            // If allocation succeeds, posix_memalign overwrites memptr variable with
            // the freshly allocated heap address

            // On success, posix_memalign returns 0 and writes a valid, non-null pointer 
            // address to *memptr

            // If allocation fails (out of memory) or alignment is invalid, it returns an 
            // error code (such as ENOMEM or EINVAL) and leaves memptr unchanged or sets 
            // it to NULL
            
            let ret = libc::posix_memalign(

                // posix_memalign expects a pointer to a pointer (specifically *mut 
                // *mut c_void). Casting it to *mut *mut T converts that reference 
                // into a raw pointer-to-pointer. .cast() converts *mut *mut T to 
                // *mut *mut c_void to match C's void** parameter type
                (&mut memptr as *mut *mut T).cast(),

                // POSIX requires the alignment parameter passed to posix_memalign to 
                // meet two strict requirements:
                //
                //   1. It must be a power of two
                //   2. It must be a multiple of sizeof(void*) (which is size_of::<usize>()
                //      in Rust)
                //
                // Taking max(align_of::<T>(), size_of::<usize>()) guarantees that even if T 
                // has a small alignment requirement (like u8, which has an alignment of 1), 
                // the alignment passed to posix_memalign will be at least size_of::<usize>() 
                // (8 bytes on 64-bit platforms)
                max(align_of::<T>(), size_of::<usize>()),

                // The number of bytes to allocate on the heap (the size of one instance of T)
                size_of::<T>()
            );

            assert_eq!(ret, 0, "Failed to allocate or invalid alignment");
        };

        // posix_memalign succeeded

        // NonNull is just a wrapper that enforces that the pointer isn't null.
        let ptr = {

            // Safety: memptr is dereferenceable because we created it from a
            // reference and have exclusive access

            // Calling NonNull::new(memptr) on a successful posix_memalign address 
            // will always return Some(NonNull)
            ptr::NonNull::new(memptr)
                .expect("Guaranteed non-null if posix_memalign returns 0")
        };

        // Move value from the stack to the location we allocated on the heap.
        unsafe {
            // Safety: If non-null, posix_memalign gives us a ptr that is valid
            // for writes and properly aligned.
            ptr.as_ptr().write(value);
        }

        Self(ptr)
    }
}

impl<T> Deref for Carton<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            // Safety: The pointer is aligned, initialized, and dereferenceable
            //   by the logic in [`Self::new`]. We require readers to borrow the
            //   Carton, and the lifetime of the return value is elided to the
            //   lifetime of the input. This means the borrow checker will
            //   enforce that no one can mutate the contents of the Carton until
            //   the reference returned is dropped.
            self.0.as_ref()
        }
    }
}

impl<T> DerefMut for Carton<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            // Safety: The pointer is aligned, initialized, and dereferenceable
            //   by the logic in [`Self::new`]. We require writers to mutably
            //   borrow the Carton, and the lifetime of the return value is
            //   elided to the lifetime of the input. This means the borrow
            //   checker will enforce that no one else can access the contents
            //   of the Carton until the mutable reference returned is dropped.
            self.0.as_mut()
        }
    }
}

impl<T> Drop for Carton<T> {
    fn drop(&mut self) {
        unsafe {
            libc::free(self.0.as_ptr().cast());
        }
    }
}

// Safety: No one besides us has the raw pointer, so we can safely transfer the
// Carton to another thread if T can be safely transferred
unsafe impl<T> Send for Carton<T> where T: Send {}

// Safety: Since there exists a public way to go from a `&Carton<T>` to a `&T`
// in an un-synchronized fashion (such as `Deref`), then `Carton<T>` can't be
// `Sync` if `T` isn't

// Conversely, `Carton` itself does not use any interior mutability whatsoever:
// all the mutations are performed through an exclusive reference (`&mut`). This
// means it suffices that `T` be `Sync` for `Carton<T>` to be `Sync`:
unsafe impl<T> Sync for Carton<T> where T: Sync  {}

fn main() {
    
    //------------------------------------------
    // Using Threads to Run Code Simultaneously
    //------------------------------------------

    {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

        // Rust infers how to capture v, and because println! only needs a reference to v, 
        // the closure tries to borrow v

        // By adding the move keyword before the closure, we force the closure to take 
        // ownership of the values it’s using rather than allowing Rust to infer that it 
        // should borrow the values

        let handle = thread::spawn(move || {
            for i in &v {
                println!("hi number {i} from the spawned thread!");
                thread::sleep(Duration::from_millis(1));
            }
        });

        // ❌ By telling Rust to move ownership of v to the spawned thread, we’re guaranteeing 
        //    to Rust that the main thread won’t use v anymore
        // drop(v);

        for i in 1..5 {
            println!("hi number {i} from the main thread!");
            thread::sleep(Duration::from_millis(1));
        }

        // Calling join on the handle blocks the thread currently running until the thread 
        // represented by the handle terminates

        handle.join().unwrap();
    }

    //----------------------------------------------------
    // Transfer Data Between Threads with Message Passing
    //----------------------------------------------------

    {
        // `mpsc` stands for multiple producer, single consumer
        let (tx, rx) = mpsc::channel();

        let tx1 = tx.clone();
        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {

                // The send method returns a Result<T, E> type, so if the receiver has
                // already been dropped and there’s nowhere to send a value, the send 
                // operation will return an error

                tx1.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));

                // ❌ The send function takes ownership of its parameter, and when the 
                //     value is moved the receiver takes ownership of it

                // println!("val is {val}");
            }
        });

        thread::spawn(move || {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        // `recv`, short for receive, will block the main thread’s execution and wait
        // until a value is sent down the channel. Once a value is sent, recv will return
        // it in a Result<T, E>. When the transmitter closes, recv will return an error
        // to signal that no more values will be coming

        // The try_recv method doesn’t block, but will instead return a Result<T, E> 
        // immediately: an Ok value holding a message if one is available and an Err value 
        // if there aren’t any messages this time

        // let received = rx.recv().unwrap();

        for received in rx {
            println!("Got: {received}");
        }
    }

    //--------------------------
    // Shared-State Concurrency
    //--------------------------

    // Controlling Access with Mutexes

    // 1. You must attempt to acquire the lock before using the data.
    // 2. When you’re done with the data that the mutex guards, you must unlock the 
    //    data so that other threads can acquire the lock.

    // Mutex<T> comes with the risk of creating deadlocks. These occur when an operation 
    // needs to lock two resources and two threads have each acquired one of the locks, 
    // causing them to wait for each other forever

    {
        // Arc<T> is a type like Rc<T> that is safe to use in concurrent situations

        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);

            // To access the data inside the mutex, we use the lock method to acquire 
            // the lock. This call will block the current thread so that it can’t do 
            // any work until it’s our turn to have the lock

            // The exact type of num is std::sync::MutexGuard<'_, i32>

            // The MutexGuard type implements Deref to point at our inner data; the 
            // type also has a Drop implementation that releases the lock automatically 
            // when a MutexGuard goes out of scope, which happens at the end of the 
            // inner scope

            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();

                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
    }

    //-------------------------------------------
    // Extensible Concurrency with Send and Sync
    //-------------------------------------------

    // The ownership of values of the type implementing `Send` can be transferred between threads

    // it is safe for the type implementing `Sync` to be referenced from multiple threads
    
    //---------------
    // Send and Sync
    //---------------

    // Types that aren’t automatically derived can simply implement them if desired

    {
        struct MyBox(*mut u8);

        unsafe impl Send for MyBox {}
        unsafe impl Sync for MyBox {}
    }
}
