use std::mem::{self, MaybeUninit};
use std::ptr;

fn main() {

    //------------------------------
    // Checked Uninitialized Memory
    //------------------------------

    {
        let x: i32;

        // ❌ `x` used here but it isn't initialized
        // println!("{}", x);
    }

    {
        // Rust doesn’t require the variable to be mutable to perform a delayed 
        // initialization if every branch assigns exactly once

        let x: i32;

        if true {
            x = 1;
        } else {
            x = 2;
        }

        println!("{}", x);
    }

    {
        let x: i32;
        if true {
            x = 1;
        }

        // ❌ However the analysis does not take advantage of constant analysis or 
        // anything like that

        // println!("{}", x);
    }

    {
        let x: i32;
        if true {
            x = 1;
            println!("{}", x);
        }

        // Don't care that there are branches where it's not initialized
        // since we don't use the value in those branches
    }

    {
        let x: i32;

        loop {
            // Rust doesn't understand that this branch will be taken unconditionally,
            // because it relies on actual values.
            if true {

                // But it does understand that it will only be taken once because
                // we unconditionally break out of it. Therefore `x` doesn't
                // need to be marked as mutable.
                x = 0;
                break;
            }
        }

        // It also knows that it's impossible to get here without reaching the break.
        // And therefore that `x` must be initialized here!
        println!("{}", x);
    }

    {
        // If a value is moved out of a variable, that variable becomes logically 
        // uninitialized if the type of the value isn’t Copy

        let x = 0;
        let y = Box::new(0);
        let z1 = x; // x is still valid because i32 is Copy
        let z2 = y; // y is now logically uninitialized because Box isn't Copy
    }

    {
        // Reassigning y would require y to be marked as mutable, as a Safe Rust 
        // program could observe that the value of y changed

        let mut y = Box::new(0);
        let z = y;          // y is now logically uninitialized because Box isn't Copy
        y = Box::new(1);    // reinitialize y
    }

    //-----------
    // Drop flag
    //-----------

    {
        let mut x = Box::new(0); // let makes a fresh variable, so never need to drop
        let y = &mut x;
        *y = Box::new(1); // Deref assumes the referent is initialized, so always drops
    }

    {
        let mut x = Box::new(0); // x was uninit; just overwrite.
        let mut y = x;           // y was uninit; just overwrite and make x uninit.
        x = Box::new(0);         // x was uninit; just overwrite.
        y = x;                   // y was init; Drop y, overwrite it, and make x uninit!
    }
                                 // y goes out of scope; y was init; Drop y!
                                 // x goes out of scope; x was uninit; do nothing.

    //--------------------------------
    // Unchecked Uninitialized Memory
    //--------------------------------

    {
        // Size of the array is hard-coded but easy to change (meaning, changing just
        // the constant is sufficient). This means we can't use [a, b, c] syntax to
        // initialize the array, though, as we would have to keep that in sync
        // with `SIZE`!
        const SIZE: usize = 10;

        let x = {
            // Create an uninitialized array of `MaybeUninit`.
            let mut x = [const { MaybeUninit::uninit() }; SIZE];

            // Dropping a `MaybeUninit` does nothing. Thus using raw pointer assignment
            // instead of `ptr::write` does not cause the old uninitialized value to be
            // dropped

            // Exception safety is not a concern because Box can't panic

            for i in 0..SIZE {
                x[i] = MaybeUninit::new(Box::new(i as u32));
            }

            // Everything is initialized. Transmute the array to the
            // initialized type.
            unsafe { mem::transmute::<_, [Box<u32>; SIZE]>(x) }
        };

        println!("{x:?}");
    }

    {
        struct Demo {
            field: bool,
        }

        let mut uninit = MaybeUninit::<Demo>::uninit();

        // `&uninit.as_mut().field` would create a reference to an uninitialized
        // `bool`, and thus be Undefined Behavior!

        /*
         *  A reference &mut T asserts that the memory it points to contains a valid, 
         *  fully initialized instance of T
         *
         *  The raw borrow operator (&raw mut or &raw const) does not create a 
         *  reference. Instead, it creates a raw pointer (*mut T). Raw pointers make 
         *  zero guarantees about the validity, alignment, or initialization of the 
         *  memory they point to. They are simply numerical memory addresses tagged 
         *  with a type
         *
         *  When you write &raw mut (*ptr).field, you are telling the compiler: "Treat 
         *  this as a place expression to calculate an offset/address, but do not load 
         *  from it and do not create a reference."
         */

        let f1_ptr = unsafe { &raw mut (*uninit.as_mut_ptr()).field };
        unsafe { f1_ptr.write(true); }

        let init = unsafe { uninit.assume_init() };
    }
}
