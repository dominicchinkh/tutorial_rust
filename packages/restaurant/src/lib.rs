mod front_of_house;

mod back_of_house;

//--------------------------------
// Creating Idiomatic `use` Paths

// Specifying the parent module when calling the function makes it clear that the function isn't 
// locally defined while still minimizing repetition of the full path

// On the other hand, when bringing in structs, enums, and other items with use, it’s idiomatic to 
// specify the full path

use crate::front_of_house::hosting;

// Note that `use` only creates the shortcut for the particular scope in which the use occurs
mod customer {
    pub fn eat_at_restaurant() {
        // ❌ The `eat_at_restaurant` function is in a new child module named `customer`, which is then a different scope 
        //     than the 1use1 statement
        // hosting::add_to_waitlist();
    }
}

//---------------------------------
// Re-exporting Names with pub use

// When we bring a name into scope with the use keyword, the name is private to the scope into which 
// we imported it. To enable code outside that scope to refer to that name as if it had been defined 
// in that scope, we can combine pub and use

// External code can use the path `restaurant::hosting::add_to_waitlist()`

// pub use crate::front_of_house::hosting;

//-----------------------------------------
// Providing New Names with the as Keyword

// After the path, we can specify as and a new local name, or alias, for the type

// use std::fmt::Result;
// use std::io::Result as IoResult;

//-------------
// Nested path

// We can use nested paths to bring the same items into scope in one line

// use std::{cmp::Ordering, io};

// Same as:
//   use std::cmp::Ordering;
//   use std::io;

// use std::io::{self, Write};

// Same as:
//   use std::io;
//   use std::io::Write;

//----------------------------------------
// Importing Items with the Glob Operator

// use std::collections::*;

//-------------------------------

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Bring path into scope with `use`
    hosting::add_to_waitlist();

    // struct
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // Change our mind about what bread we'd like.
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal.
    // meal.seasonal_fruit = String::from("blueberries");
}

fn deliver_order() {}
