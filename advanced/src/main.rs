use std::slice;

// rust global variable:
static HELLO_WORLD: &str = "Hello, world!";

// Static variables can be mutable. Accessing and modifying mutable static variables 
// is unsafe
static mut COUNTER: u32 = 0;

/*
 *  pub trait Iterator {
 *  
 *      // With associated types, we don’t need to annotate types
 *      type Item;
 *  
 *      fn next(&mut self) -> Option<Self::Item>;
 *  }
 */

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {

    // We can choose what the type of Item will be only once because there can be only 
    // one impl Iterator for Counter

    // Implementors of the trait must provide a type to stand in for the associated type
    // placeholder
    
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {

    //-------------
    // Unsafe Rust
    //-------------

    //-----------------------------
    // Dereferencing a Raw Pointer

    /*
     *  Raw pointers:
     *
     *    1. Are allowed to ignore the borrowing rules by having both immutable and
     *       mutable pointers or multiple mutable pointers to the same location
     *
     *    2. Aren’t guaranteed to point to valid memory
     *
     *    3. Are allowed to be null
     *
     *    4. Don’t implement any automatic cleanup
     */

    {
        let mut num = 5;

        let r1 = &raw const num;
        let r2 = &raw mut num;
    }

    {
        // Create a raw pointer to an arbitrary location in memory
        let address = 0x012345usize;
        let r = address as *const i32;
    }

    {
        let mut num = 5;

        let r1 = &raw const num;
        let r2 = &raw mut num;

        // We can create raw pointers in safe code, but we can’t dereference raw pointers
        // and read the data being pointed to. We use the dereference operator * on a raw
        // pointer that requires an unsafe block

        unsafe {
            println!("r1 is: {}", *r1);
            println!("r2 is: {}", *r2);
        }
    }

    //--------------------------------------
    // Calling an Unsafe Function or Method

    {
        // We must call the dangerous function within a separate unsafe block
        unsafe {
            dangerous();
        }
    }

    // Creating a Safe Abstraction over Unsafe Code

    {
        let mut v = vec![1, 2, 3, 4, 5, 6];

        let r = &mut v[..];

        let (a, b) = r.split_at_mut(3);

        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);
    }

    // Using extern Functions to Call External Code

    {
        unsafe {
            println!("Absolute value of -3 according to C: {}", abs(-3));
        }
    }

    //--------------------------------------------------
    // Accessing or Modifying a Mutable Static Variable

    {
        unsafe {
            // SAFETY: This is only called from a single thread in `main`.
            add_to_count(3);
            println!("COUNTER: {}", *(&raw const COUNTER));
        }
    }
}

unsafe fn dangerous() 
{
    // To perform unsafe operations in the body of an unsafe function, you still need
    // to use an unsafe block, just as within a regular function
}

// Note that we don’t need to mark the resultant split_at_mut function as unsafe

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();

    // Use `as_mut_ptr` method to access the raw pointer of a slice. In this case, 
    // because we have a mutable slice to i32 values, as_mut_ptr returns a raw pointer
    // with the type *mut i32

    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            // The `slice::from_raw_parts_mut` function takes a raw pointer and a length, 
            // and it creates a slice. The function slice::from_raw_parts_mut is unsafe 
            // because it takes a raw pointer and must trust that this pointer is valid

            // The `add` method on `ptr` with `mid` as an argument to get a raw pointer 
            // that starts at mid. The add method on raw pointers is also unsafe because 
            // it must trust that the offset location is also a valid pointer

            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

/*
 *  Rust has the keyword extern that facilitates the creation and use of a Foreign 
 *  Function Interface (FFI), which is a way for a programming language to define 
 *  functions and enable a different (foreign) programming language to call those 
 *  functions
 *
 *  The "C" part defines which application binary interface (ABI) the external function
 *  uses: The ABI defines how to call the function at the assembly level. The "C" ABI 
 *  is the most common and follows the C programming language’s ABI
 */

unsafe extern "C" {

    // We list the names and signatures of external functions from another language 
    // we want to call

    fn abs(input: i32) -> i32;
}

/*
 *  We can also use extern to create an interface that allows other languages to call 
 *  Rust functions
 *
 *  1. We add the extern keyword and specify the ABI to use just before the fn keyword 
 *     for the relevant function
 *
 *  2. We also need to add an #[unsafe(no_mangle)] annotation to tell the Rust compiler 
 *     not to mangle the name of this function. Mangling is when a compiler changes the 
 *     name we’ve given a function to a different name that contains more information 
 *     for other parts of the compilation process to consume but is less human readable
 *
 */

#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

// Whenever we perform an unsafe operation, it is idiomatic to write a comment starting
// with SAFETY to explain how the safety rules are upheld

/// SAFETY: Calling this from more than a single thread at a time is undefined behavior,
///         so you *must* guarantee you only call it from a single thread at a time.

unsafe fn add_to_count(inc: u32) {
    unsafe {

        // Where possible, it’s preferable to use the concurrency techniques and thread
        // safe smart pointers we discussed in Chapter 16 so that the compiler checks 
        // that data access from different threads is done safely

        COUNTER += inc;
    }
}
