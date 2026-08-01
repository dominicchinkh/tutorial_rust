use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {

        // The standard library didn’t need to know anything about the 
        // `Inventory` or `ShirtColor` types we defined, or the logic we 
        // want to use in this scenario. The closure captures an immutable 
        // reference to the `self` Inventory instance and passes it with 
        // the code we specify to the unwrap_or_else method

        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {

    //----------
    // Closures
    //----------

    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    //----------------------------------------
    // Inferring and Annotating Closure Types

    // These are all valid definitions that will produce the same behavior
    // when they’re called

    // fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    // let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // let add_one_v3 = |x|             { x + 1 };
    // let add_one_v4 = |x|               x + 1  ;

    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));

    // ❌ The first time we call example_closure with the String value, the
    //    compiler infers the type of x and the return type of the closure to
    //    be String. Those types are then locked into the closure, and we get 
    //    a type error when we next try to use a different type with the same 
    //    closure.
    // let n = example_closure(5);

    //------------------------------------------
    // Capturing References or Moving Ownership

    // Borrowing immutably
        let list = vec![1, 2, 3];
        println!("Before defining closure: {list:?}");

        let only_borrows = || println!("From closure: {list:?}");

        // We can have multiple immutable references to list at the same time
        println!("Before calling closure: {list:?}");
        only_borrows();
        println!("After calling closure: {list:?}");

    // Borrowing mutably
        let mut list = vec![1, 2, 3];

        // When borrows_mutably is defined, it captures a mutable reference to list
        println!("Before defining closure: {list:?}");

        let mut borrows_mutably = || list.push(7);

        // An immutable borrow to print isn’t allowed here

        borrows_mutably();

        // We don’t use the closure again after the closure is called, so the 
        // mutable borrow ends
        println!("After calling closure: {list:?}");

    // Taking ownership
        let list = vec![1, 2, 3];
        println!("Before defining closure: {list:?}");

        // Specify that list should be moved into the closure by putting the 
        // `move` keyword at the beginning of the closure definition

        thread::spawn(move || println!("From thread: {list:?}"))
            .join()
            .unwrap();

        // If the main thread maintained ownership of `list` but ended before  
        // the new thread and drops `list`, the immutable reference in the 
        // thread would be invalid. Therefore, the compiler requires that `list`
        // be moved into the closure given to the new thread so that the reference 
        // will be valid.

    //----------------------------------------
    // Moving Captured Values Out of Closures

    // Move a captured value out of the closure

        // A closure that moves captured values out of its body will only implement
        // `FnOnce` and none of the other `Fn` traits because it can only be called
        // once

        // Using `FnOnce` in the trait bound expresses the constraint that 
        // `unwrap_or_else` will not call `f` more than once. In the body of 
        // `unwrap_or_else`, we can see that if the Option is Some, `f` won’t be
        // called. If the Option is None, `f` will be called once.

        // impl<T> Option<T> {
        //     pub fn unwrap_or_else<F>(self, f: F) -> T
        //     where
        //         F: FnOnce() -> T
        //     {
        //         match self {
        //             Some(x) => x,
        //             None => f(),
        //         }
        //     }
        // }

    // Mutate the captured value

        // `FnMut` applies to closures that don’t move captured values out of their
        // body but might mutate the captured values. These closures can be called 
        // more than once

        let mut list = [
            Rectangle { width: 10, height: 1 },
            Rectangle { width: 3, height: 5 },
            Rectangle { width: 7, height: 12 },
        ];

        // `sort_by_key` uses `FnMut` instead of `FnOnce` for the trait bound. It 
        // calls the closure multiple times: once for each item in the slice

        list.sort_by_key(|r| r.width);
        println!("{list:#?}");

    // Neither move nor mutate the value
    // Capture nothing from the environment to begin with

        // `Fn` applies to closures that don’t move captured values out of their 
        // body and don’t mutate captured values, as well as closures that capture 
        // nothing from their environment

    //-----------
    // Iterators
    //-----------

    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    // We don’t need to make `v1_iter` mutable when we used a for loop, because the 
    // loop took ownership of `v1_iter` and made it mutable behind the scenes.

    for val in v1_iter {
        println!("Got: {val}");
    }

    //----------------------------------------
    // The Iterator Trait and the next Method

    // Implementing the Iterator trait requires that you also define an Item type, 
    // and this Item type is used in the return type of the next method

        // pub trait Iterator {
        //     type Item;
        //     fn next(&mut self) -> Option<Self::Item>;
        // }

    let v1 = vec![1, 2, 3];

    // Note that we needed to make v1_iter mutable: Calling the next method on an 
    // iterator changes internal state that the iterator uses to keep track of 
    // where it is in the sequence

    // into_iter: 
    //   create an iterator that takes ownership of v1 and returns owned values

    // iter_mut: 
    //   iterate over mutable references

    let mut v1_iter = v1.iter();

    // Note: the values we get from the calls to next are immutable references to 
    // the values in the vector

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    //-----------------------------------
    // Methods That Consume the Iterator

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    // We aren’t allowed to use v1_iter after the call to sum, because sum takes 
    // ownership of the iterator we call it on

    assert_eq!(total, 6);

    //--------------------------------------
    // Methods That Produce Other Iterators

    // Iterator adapters are methods defined on the Iterator trait that don’t 
    // consume the iterator. Instead, they produce different iterators by changing 
    // some aspect of the original iterator

    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter()
                        .map(|x| x + 1)
                        // Consume the iterator and collects the resultant values 
                        // into a collection data type
                        .collect();

    assert_eq!(v2, vec![2, 3, 4]);

}
