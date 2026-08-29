use hello_macro::HelloMacro;
use std::fmt;
use std::ops::Add;
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

/*
 *  The Add trait has an associated type named Output that determines the type returned 
 *  from the add method.
 *
 *  Rhs=Self: This syntax is called default type parameters
 *
 *  If we don’t specify a concrete type for Rhs when we implement the Add trait, the type 
 *  of Rhs will default to Self
 *
 *    trait Add<Rhs=Self> {
 *        type Output;
 *   
 *        fn add(self, rhs: Rhs) -> Self::Output;
 *    }
 */

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Millimeters(u32);
struct Meters(u32);

// To add Millimeters and Meters, we specify impl Add<Meters> to set the value of the
// Rhs type parameter instead of using the default of Self

impl Add<Meters> for Millimeters {
    type Output = Millimeters;


    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// We need to specify that the OutlinePrint trait will work only for types that also
// implement Display and provide the functionality that OutlinePrint needs. We can do
// that in the trait definition by specifying OutlinePrint: Display

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl OutlinePrint for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// We’re only allowed to implement a trait on a type if either the trait or the type, 
// or both, are local to our crate. It’s possible to get around this restriction using 
// the newtype pattern, which involves creating a new type in a tuple struct

// Let’s say we want to implement Display on Vec<T>, which the orphan rule prevents us 
// from doing directly because the Display trait and the Vec<T> type are defined outside 
// our crate. We can make a Wrapper struct that holds an instance of Vec<T>; then, we can 
// implement Display on Wrapper and use the Vec<T> value

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        // Use self.0 to access the inner Vec<T> because Wrapper is a tuple struct and 
        // Vec<T> is the item at index 0 in the tuple

        write!(f, "[{}]", self.0.join(", "))
    }
}

// If we wanted the new type to have every method the inner type has, implementing the 
// Deref trait on the Wrapper to return the inner type would be a solution

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
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

    //-----------------
    // Advanced traits
    //-----------------

    //-----------------------------------------------------------
    // Using Default Generic Parameters and Operator Overloading

    // When we use generic type parameters, we can specify a default concrete type for the
    // generic type. This eliminates the need for implementors of the trait to specify a 
    // concrete type if the default type works

    {
        assert_eq!(
            Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
            Point { x: 3, y: 3 }
        );
    }

    //--------------------------------------------------
    // Disambiguating Between Identically Named Methods

    // When calling methods with the same name, you’ll need to tell Rust which one 
    // you want to use

    {
        let person = Human;

        // To call the fly methods from either the Pilot trait or the Wizard trait, we
        // need to use more explicit syntax to specify which fly method we mean

        Pilot::fly(&person);
        Wizard::fly(&person);

        // When we call fly on an instance of Human, the compiler defaults to calling 
        // the method that is directly implemented on the type
        person.fly();
    }

    // Associated functions that are not methods don’t have a self parameter

    {
        println!("A baby dog is called a {}", Dog::baby_name());

        // To disambiguate and tell Rust that we want to use the implementation of Animal 
        // for Dog as opposed to the implementation of Animal for some other type, we need 
        // to use fully qualified syntax
        println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    }

    //-------------------
    // Using Supertraits

    {
        let point = Point { x: 3, y: 3 };
        point.outline_print();
    }

    //-------------------------------------------------------
    // Implementing External Traits with the Newtype Pattern

    {
        let w = Wrapper(vec![String::from("hello"), String::from("world")]);
        println!("w = {w}");
    }

    //----------------
    // Advanced Types
    //----------------

    //------------------------------------------------------
    // Type Safety and Abstraction with the Newtype Pattern

    /*
     *  1. Statically enforcing that values are never confused and indicating the units
     *     of a value: The Millimeters and Meters structs wrapped u32 values in a newtype. 
     *     If we wrote a function with a parameter of type Millimeters, we wouldn’t be 
     *     able to compile a program that accidentally tried to call that function with a 
     *     value of type Meters or a plain u32
     *
     *  2. We can also use the newtype pattern to abstract away some implementation details
     *     of a type: The new type can expose a public API that is different from the API
     *     of the private inner type
     *
     *  3. Newtypes can also hide internal implementation
     *
     */

    //--------------------------------
    // Type Synonyms and Type Aliases

    // Rust provides the ability to declare a type alias to give an existing type 
    // another name

    {
        // Kilometers is not a separate, new type. Values that have the type Kilometers
        // will be treated the same as values of type i32. We don’t get the type-checking
        // benefits 

        type Kilometers = i32;

        let x: i32 = 5;
        let y: Kilometers = 5;

        println!("x + y = {}", x + y);

        // The main use case for type synonyms is to reduce repetition
    }

    //-----------------------------------
    // The Never Type That Never Returns

    // Rust has a special type named ! that’s known in type theory lingo as the empty
    // type because it has no values

    {
        // `continue` has a ! value. when Rust computes the type of guess, it looks at 
        // both match arms, the former with a value of u32 and the latter with a ! 
        // value. Because ! can never have a value, Rust decides that the type of guess
        // is u32

       /*
        *    loop {
        *        // --snip--
        *
        *        // Convert the string to a u32
        *        let guess: u32 = match guess.trim().parse() {
        *            Ok(num) => num,
        *            Err(_) => continue,
        *        };
        *
        *        // --snip--
        *    }
        */
    }

    //---------------------------------------------
    // Dynamically Sized Types and the Sized Trait

    //---------------------------------
    // Advanced Functions and Closures
    //---------------------------------

    //-------------------
    // Function Pointers

    // Functions coerce to the type fn (with a lowercase f), not to be confused with 
    // the Fn closure trait. The fn type is called a function pointer

    // Unlike closures, fn is a type rather than a trait, so we specify fn as the 
    // parameter type directly rather than declaring a generic type parameter with 
    // one of the Fn traits as a trait bound

    {
        fn add_one(x: i32) -> i32 {
            x + 1
        }

        fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
            f(arg) + f(arg)
        }

        let answer = do_twice(add_one, 5);
        println!("The answer is: {answer}");
    }

    // Function pointers implement all three of the closure traits (Fn, FnMut, and 
    // FnOnce), meaning you can always pass a function pointer as an argument for a 
    // function that expects a closure

    {
        {
            // Use a closure
            let list_of_numbers = vec![1, 2, 3];
            let list_of_strings: Vec<String> =
                list_of_numbers.iter().map(|i| i.to_string()).collect();
        }

        {
            // Use a function
            let list_of_numbers = vec![1, 2, 3];
            let list_of_strings: Vec<String> =
                list_of_numbers.iter().map(ToString::to_string).collect();
        }
    }

    // The name of each enum variant that we define also becomes an initializer function. 
    // We can specify the initializer functions as arguments for methods that take closures
    
    {
        enum Status {
            Value(u32),
            Stop,
        }

        let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    }

    //--------------------
    // Returning Closures

    {
        fn returns_closure() -> impl Fn(i32) -> i32 {
            |x| x + 1
        }
    }

    {
        /*
         *  ❌ Rust creates a unique opaque type, a type where we cannot see into the 
         *  details of what Rust constructs for us, nor can we guess the type Rust 
         *  will generate to write ourselves. So, even though these functions return 
         *  closures that implement the same trait, Fn(i32) -> i32, the opaque types 
         *  Rust generates for each are distinct
         *
         *    fn returns_closure() -> impl Fn(i32) -> i32 {
         *        |x| x + 1
         *    }
         *
         *    fn returns_initialized_closure(init: i32) -> impl Fn(i32) -> i32 {
         *        move |x| x + init
         *    }
         */

        // We can use a trait object

        fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
            Box::new(|x| x + 1)
        }

        fn returns_initialized_closure(init: i32) -> Box<dyn Fn(i32) -> i32> {
            Box::new(move |x| x + init)
        }

        let handlers = vec![returns_closure(), returns_initialized_closure(123)];
        for handler in handlers {
            let output = handler(5);
            println!("{output}");
        }
    }

    //-------
    // Macro
    //-------

    // Macros are a way of writing code that writes other code, which is known as 
    // metaprogramming

    // The Difference Between Macros and Functions:
    // 1. Macros can take a variable number of parameters
    // 2. Macros are expanded before the compiler interprets the meaning of the code
    // 3. You must define macros or bring them into scope before you call them in a file

    //------------------------------------------------
    // Declarative Macros for General Metaprogramming

    // Macros compare a value to patterns that are associated with particular code: In 
    // this situation, the value is the literal Rust source code passed to the macro;  
    // the patterns are compared with the structure of that source code; and the code  
    // associated with each pattern, when matched, replaces the code passed to the macro

    {
        // Indicate that this macro should be made available whenever the crate in which 
        // the macro is defined is brought into scope
        #[macro_export]

        // Start the macro definition with macro_rules! and the name of the macro we’re 
        // defining without the exclamation mark
        macro_rules! vec {

            // If the pattern `( $( $x:expr ),* )` matches, the associated block of code
            // will be emitted

            // ($): declare a variable in the macro system that will contain the Rust code 
            //      matching the pattern
            // $x:expr: match any Rust expression and gives the expression the name `$x`
            // ,: a literal comma separator character must appear between each instance of 
            //    the code that matches the code in $()
            // *: specifi that the pattern matches zero or more of whatever precedes the *

            // When we call this macro with vec![1, 2, 3];, the $x pattern matches three 
            // times with the three expressions 1, 2, and 3

            ( $( $x:expr ),* ) => {
                {
                    let mut temp_vec = Vec::new();

                    // `temp_vec.push()` is generated for each part that matches $() in the 
                    // pattern zero or more times
                    $(
                        temp_vec.push($x);
                    )*
                    temp_vec
                }
            };
        }
    }

    //-------------------------------------------------------
    // Procedural Macros for Generating Code from Attributes

    // Procedural macros accept some code as an input, operate on that code, and produce
    // some code as an output

    //----------------------
    // Custom derive Macros

    {
        Pancakes::hello_macro()
    }

    //-----------------------
    // Attribute-Like Macros
    
    {
        /*
         *  We have two parameters of type TokenStream. The first is for the contents 
         *  of the attribute: the GET, "/" part. The second is the body of the item the 
         *  attribute is attached to: in this case, fn index() {} and the rest of the 
         *  function’s body
         *
         *    #[route(GET, "/")]
         *    fn index() {
         *    
         *    #[proc_macro_attribute]
         *    pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
         *
         */
    }

    //----------------------
    // Function-Like Macros

    // Function-like macros take a TokenStream parameter, and their definition 
    // manipulates that TokenStream using Rust code as the other two types of 
    // procedural macros do

    {
        /*
         *  This macro would parse the SQL statement inside it and check that it’s 
         *  syntactically correct
         *
         *    let sql = sql!(SELECT * FROM posts WHERE id=1);
         *    
         *    #[proc_macro]
         *    pub fn sql(input: TokenStream) -> TokenStream {
         *
         */
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

fn bar() -> ! {
    // The ! indicates that a function will never return control back to the caller -
    // either because it terminates the process, enters an infinite loop, or panics

    panic!("This function panics and never returns");
}
