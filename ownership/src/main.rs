fn main() {

    //----------------
    // Ownership rule
    //
    // 1. Each value in Rust has an owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    //----------------
    // Variable scope
    {                       // `s` is not valid here, since it's not yet declared
        let s = "hello";   // `s` is valid from this point forward

        // do stuff with s
    }                       // This scope is now over, and s is no longer valid

    //------------------------------------------
    // Variables and Data Interacting with Move

    // To ensure memory safety, after the line let s2 = s1;, Rust considers s1 as 
    // no longer valid
    let s1 = String::from("hello");
    let s2 = s1;

    // ❌ `s1` is no longer valid
    // println!("{s1}, world!");

    //-------------------------------------------
    // Variables and Data Interacting with Clone

    // If we do want to deeply copy the heap data of the String, not just the stack data, 
    // we can use a common method called clone

    let s1 = String::from("hello");

    // The heap data does get copied!
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

    //-----------------------
    // Stack only data: copy

    // Here are some of the types that implement `Copy`
    // * All the integer types, such as u32.
    // * The Boolean type, bool, with values true and false.
    // * All the floating-point types, such as f64.
    // * The character type, char.
    // * Tuples, if they only contain types that also implement Copy.

    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");

    //-------------------------
    // Ownership and Functions

    let s = String::from("hello");  // `s` comes into scope

    takes_ownership(s);             // `s` value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // `x` comes into scope

    makes_copy(x);                  // Because i32 implements the Copy trait,
                                    // `x` does NOT move into the function,
                                    // so it's okay to use `x` afterward.

    //-------------------------
    // Return Values and Scope

    let s1 = gives_ownership();        // gives_ownership moves its return
                                        // value into `s1`

    let s2 = String::from("hello");     // `s2` comes into scope

    let s3 = takes_and_gives_back(s2); // `s2` is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into `s3`

    //--------------------------
    // References and Borrowing

    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    //-------------------
    // Mutable reference
    let mut s2 = String::from("hello");

    change(&mut s2);

    //--------------------------------
    let mut s = String::from("hello");

    let r1 = &mut s;

    // ❌ If you have a mutable reference to a value, you can have no other references 
    //     to that value
    // let r2 = &mut s;
    // println!("{_r1}, {_r2}");

    //--------------------------------
    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;

    //--------------------------------
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem

    // ❌ We also cannot have a mutable reference while we have an immutable one to the 
    //     same value.
    // let r3 = &mut s;
    // println!("{_r1}, {_r2}, and {_r3}");

    //--------------------------------
    
    // A reference’s scope starts from where it is introduced and continues through the 
    // last time that reference is used

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // Variables r1 and r2 will not be used after this point.

    let r3 = &mut s; // no problem
    println!("{r3}");

    //---------------
    // String slices

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    // These are equal:

    // let slice = &s[0..2];
    // let slice = &s[..2];

    // let len = s.len();
    // let slice = &s[3..len];
    // let slice = &s[3..];

    // let slice = &s[0..len];
    // let slice = &s[..];

    let word = first_word(&s);

    // ❌ if we have an immutable reference to something, we cannot also take a mutable 
    //    reference. `clear` needs to truncate the String, it needs to get a mutable 
    //    reference. The `println!` after the call to `clear` uses the reference in `word`, 
    //    so the immutable reference must still be active at that point. 
    // s.clear();

    println!("the first word is: {word}");

    //--------------------------------

    // The type of `s` here is `&str`: It’s a slice pointing to that specific point of the 
    // binary. This is also why string literals are immutable; `&str` is an immutable 
    // reference.
    let s = "Hello, world!";

    //--------------------------------

    let a = [1, 2, 3, 4, 5];

    // This slice has the type &[i32]
    let slice = &a[1..3];
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {       // gives_ownership will move its
                                       // return value into the function
                                       // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                        // some_string is returned and
                                       // moves out to the calling
                                       // function
}

// This function takes a String and returns a String.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // Here, s goes out of scope. But because s does not have ownership of what
  // it refers to, the String is not dropped.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

/*
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // ❌ Here, s goes out of scope and is dropped, so its memory goes away.
*/

// `s: &str`: If we have a string slice, we can pass that directly. If we have a String, 
// we can pass a slice of the String or a reference to the String

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
