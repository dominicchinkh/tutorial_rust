struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Associated functions
impl Rectangle {

    // The `&self` is actually short for `self: &Self`. Within an `impl` block, the type `Self` is an alias for 
    // the type that the `impl` block is for.

    // Note that we still need to use the `&` in front of the `self` shorthand to indicate that this method borrows
    // the `Self` instance. Methods can take ownership of `self`, borrow `self` immutably, or borrow `self` mutably

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    // Method with parameter
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // We can define associated functions that don’t have self as their first parameter

    // The `Self` keywords in the return type and in the body of the function are aliases for the type that appears
    // after the `impl` keyword

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {

    // Note that the entire instance must be mutable; Rust doesn’t allow us to mark only certain 
    // fields as mutable.
    let mut user1 = build_user(String::from("dominic@example.com"), String::from("dominic"));
    
    user1.email = String::from("anotheremail@example.com");

    println!("{0} {1} {2} {3}", user1.active, user1.username, user1.email, user1.sign_in_count);

    //----------------------------------------------
    // Creating Instances with Struct Update Syntax

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // We can no longer use `user1` after creating `user2` because the String in the username 
    // field of `user1` was moved into `user2`. If we had given `user2` new String values for both 
    // `email` and `username`, and thus only used the `active` and `sign_in_count` values from `user1`, 
    // then `user1` would still be valid after creating `user2`.

    // We can also still use `user1.email` in this example, because its value was not moved out of `user1`.

    println!("{0} {1} {2}", user1.active, /*user1.username,*/ user1.email, user1.sign_in_count);

    //---------------
    // Tuple structs

    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);

    println!("{0} {1} {2}", {black.0}, {black.1}, {black.2});

    // Unlike tuples, tuple structs require you to name the type of the struct when you destructure them
    let Color(r, g, b) = black;

    //-------------------
    // Unit-Like Structs

    // Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data 
    // that you want to store in the type itself

    struct AlwaysEqual;
    let subject = AlwaysEqual;

    //---------
    // Example

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // When you call a method with `object.something()`, Rust automatically adds in `&`, `&mut`, or `*` so that 
    // object matches the signature of the method

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    // Putting the specifier `:?` inside the curly brackets tells println! we want to use an output format called 
    // `Debug`. The Debug trait enables us to print our struct in a way that is useful for developers so that we 
    // can see its value while we’re debugging our code.

    // We have to explicitly opt in to make that functionality available for our struct. To do that, we add the 
    // outer attribute `#[derive(Debug)]` just before the struct definition

    // When we have larger structs, it’s useful to have output that’s a bit easier to read; in those cases, we can 
    // use `{:#?}`

    println!("rect1 is {rect1:#?}");

    // `dbg!` macro takes ownership of an expression (as opposed to `println!`, which takes a reference), prints 
    // the file and line number of where that `dbg!` macro call occurs in your code along with the resultant value
    // of that expression, and returns ownership of the value.

    dbg!(&rect1);

    //--------
    // Method

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let rect4 = Rectangle::square(30);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,              // Because the `email` field and the `email` parameter have the same name, 
                            // we only need to write `email` rather than `email: email`
        sign_in_count: 1,
    }
}
