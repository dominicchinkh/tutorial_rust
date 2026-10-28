use std::marker::PhantomData;
use std::ops::Deref;
use std::ptr::NonNull;
use std::sync::atomic;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::thread;

pub struct ArcInner<T> {
    rc: AtomicUsize,
    data: T,
}

pub struct Arc<T> {

    // NonNull<T> is a wrapper around a raw pointer that declares that:
    //   * We are covariant over T
    //   * Our pointer is never null
    ptr: NonNull<ArcInner<T>>,

    // This will tell the drop checker that we have some notion of ownership of a
    // value of ArcInner<T> (which itself contains some T)
    phantom: PhantomData<ArcInner<T>>,
}

impl<T> Arc<T> {
    pub fn new(data: T) -> Arc<T> {

        // The Box is used as a convenient tool to create a heap allocation, and 
        // then it is immediately dismantled using Box::into_raw
        let boxed = Box::new(ArcInner {

            // We start the reference count at 1, as that first reference is the
            // current pointer.
            rc: AtomicUsize::new(1),
            data,
        });
        
        Arc {
            // Box::into_raw() consumes the Box, "leaks" its ownership (preventing 
            // it from running its destructor and freeing the memory), and returns 
            // a raw pointer (*mut ArcInner<T>)

            // It is okay to call `.unwrap()` here as we get a pointer from
            // `Box::into_raw` which is guaranteed to not be null.
            ptr: NonNull::new(Box::into_raw(boxed)).unwrap(),
            phantom: PhantomData,
        }
    }
}

unsafe impl<T: Sync + Send> Send for Arc<T> {}
unsafe impl<T: Sync + Send> Sync for Arc<T> {}

impl<T> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        let inner = unsafe { self.ptr.as_ref() };
        &inner.data
    }
}

impl<T> Clone for Arc<T> {

    fn clone(&self) -> Arc<T> {
        let inner = unsafe { self.ptr.as_ref() };

        // Using a relaxed ordering is alright here as we don't need any atomic
        // synchronization here as we're not modifying or accessing the inner
        // data.
        let old_rc = inner.rc.fetch_add(1, Ordering::Relaxed);

        if old_rc >= isize::MAX as usize {
            std::process::abort();
        }

        Self {
            ptr: self.ptr,
            phantom: PhantomData,
        }
    }
}

impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.ptr.as_ref() };

        if inner.rc.fetch_sub(1, Ordering::Release) != 1 {
            return;
        }

        // This fence is needed to prevent reordering of the use and deletion
        // of the data.
        atomic::fence(Ordering::Acquire);

        // This is safe as we know we have the last pointer to the `ArcInner`
        // and that its pointer is valid.
        unsafe { Box::from_raw(self.ptr.as_ptr()); }
    }
}

fn main() {
    println!("--- Testing basic Arc creation and Deref ---");
    let five = Arc::new(5);
    println!("Value via Deref: {}", *five);

    println!("\n--- Testing Arc cloning in a single thread ---");
    let five_clone = Arc::clone(&five);
    println!("Original: {}, Clone: {}", *five, *five_clone);

    println!("\n--- Testing Arc across multiple threads ---");
    let shared_data = Arc::new(42);
    let mut handles = vec![];

    for i in 0..4 {
        // Clone the Arc to share ownership with the thread
        let data_clone = Arc::clone(&shared_data);
        
        let handle = thread::spawn(move || {
            // Because Arc<T> is Send + Sync (when T is Send + Sync), 
            // we can safely access it from another thread.
            println!("Thread {} reading shared value: {}", i, *data_clone);
        });
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Main thread final value: {}", *shared_data);
    println!("--- End of main (All references dropped cleanly) ---");
}
