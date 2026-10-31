use std::alloc::{dealloc, Layout};
use std::ptr::NonNull;

struct Box<T> {
    ptr: NonNull<T>,
}

impl<T> Box<T> {
    pub fn new(val: T) -> Self {
        unsafe {
            let layout = Layout::new::<T>();

            // Handle zero-sized types (ZSTs) or allocate memory
            let ptr = if layout.size() == 0 {
                NonNull::dangling()
            } else {
                let raw = std::alloc::alloc(layout) as *mut T;
                NonNull::new(raw).unwrap_or_else(|| std::alloc::handle_alloc_error(layout))
            };

            ptr.as_ptr().write(val);
            Box { ptr }
        }
    }
}

impl<T> Drop for Box<T> {
    fn drop(&mut self) {
        unsafe {
            // 1. Runs the destructor (Drop::drop) for the value T stored at the pointer
            //    location, without freeing the memory

            // The memory at self.ptr is now considered uninitialized / dead data, but 
            // the memory allocation itself remains intact.
            std::ptr::drop_in_place(self.ptr.as_ptr());

            // 2. Free memory if layout size > 0

            // Layout::new::<T>(): Constructs a memory layout description specifying 
            // the exact size and alignment requirements of T. The allocator needs to 
            // know how many bytes to free and what byte boundary it was aligned to 
            // when originally allocated
            let layout = Layout::new::<T>();

            if layout.size() > 0 {

                // Returns the raw byte buffer back to the global heap allocator

                // (self.ptr.as_ptr() as *mut u8) converts a typed pointer pointing to
                // a specific Rust value T into a raw byte pointer required by the 
                // system allocator

                dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}

struct SuperBox<T> {
    my_box: Box<T>,
}

impl<T> Drop for SuperBox<T> {
    fn drop(&mut self) {
        unsafe {
            let layout = Layout::new::<T>();

            if layout.size() > 0 {
                // Free memory directly without calling drop_in_place on T
                dealloc(self.my_box.ptr.as_ptr() as *mut u8, layout);
            }
            
            // CRITICAL BUG PREVENTATIVE: Prevent `my_box`'s destructor 
            // from running and double-freeing the memory!
            std::mem::forget(std::ptr::read(&self.my_box));
        }
    }
}

fn main() {

    //--------------------------------------------
    // Ownership Based Resource Management (OBRM)
    //--------------------------------------------

    //-------------
    // Constructor

    //------------
    // Destructor

    // After drop is run, Rust will recursively try to drop all of the fields of self.

    /*
     *  When an object goes out of scope in Rust, two things happen in sequence:
     *
     *    1. Custom Drop::drop(&mut self) runs (if implemented for the type)
     *    2. Recursive Field Drop: Rust automatically drops each of the struct’s or 
     *       enum’s fields in declaration order
     */

    {
        struct Container {
            a: Box<i32>, // Automatically dropped
            b: String,   // Automatically dropped
        } // Even though Container has no custom Drop, `a` and `b` destructors run here.
    }

    /*
     * Because drop receives a mutable reference &mut self, Rust forbids moving fields
     * out of self. If moving out of self were allowed:
     *
     *    1. The moved field would be owned elsewhere
     *    2. But step 2 (automatic recursive drop) would still attempt to drop that 
     *       field on self, leading to double-drops and memory corruption
     */

    // Manual cleanup inside Drop can be dangerous: SuperBox
}
