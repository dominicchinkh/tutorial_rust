trait Trait {}

fn foo<X: Trait>(t: X) {}

impl<'a> Trait for &'a i32 {}

fn main() 
{
    //----------
    // Coercion
    //----------

    {
        let t: &mut i32 = &mut 0;

        // ❌ `Trait` is implemented for `&i32`, but not for `&mut i32`
        // foo(t);
    }

    //------------------
    // The Dot Operator
    //------------------

    /*
     *  If we call value.foo(), the compiler needs to determine what type Self is 
     *  before it can call the correct implementation of the function
     *
     *  1. The compiler checks if it can call T::foo(value) directly
     *
     *  2. Then, the compiler tries to add in an automatic reference. This means that
     *     the compiler tries <&T>::foo(value) and <&mut T>::foo(value)
     *
     *  3. It dereferences T and tries again. This uses the Deref trait - if 
     *     T: Deref<Target = U> then it tries again with type U instead of T
     *
     *  4. If it can’t dereference T, if T has a size parameter known at compile time, 
     *     it “forgets” it for the purpose of resolving methods. For instance, this 
     *     unsizing step can convert [i32; 2] into [i32] by “forgetting” the size of 
     *     the array
     */

    {
        /*
         *  1. array[0] is really just syntax sugar for the Index trait - the compiler 
         *     will convert array[0] into array.index(0)
         *
         *  2. Then, the compiler checks if Rc<Box<[T; 3]>> implements Index, but it 
         *     does not, and neither do &Rc<Box<[T; 3]>> or &mut Rc<Box<[T; 3]>>
         *
         *  3. The compiler dereferences the Rc<Box<[T; 3]>> into Box<[T; 3]> and tries
         *     again. Box<[T; 3]>, &Box<[T; 3]>, and &mut Box<[T; 3]> do not implement 
         *     Index
         *
         *  4. It dereferences again [T; 3] and its autorefs also do not implement Index
         *
         *  5. The compiler unsizes it, giving [T]. Finally, [T] implements Index, so it
         *     can now call the actual index function.
         */
        
        // let array: Rc<Box<[T; 3]>> = ...;
        // let first_entry = array[0];
    }

    // For any type U, the reference type &U ALWAYS implements Clone
    //   Borrowing an object (&U) creates an immutable, shared pointer, cloning that 
    //   pointer simply makes another copy of the pointer address - which is always 
    //   valid and cheap
    
}
