use std::cell::RefCell;
use std::fmt::Display;
use std::marker;

// You can use as many generic type parameters in a definition as you want, but using more 
// than a few makes your code hard to read
struct Point<T, U> {
    x: T,
    y: U,
}

// Note that we have to declare T just after impl so that we can use T to specify that we're 
// implementing methods on the type Point<T>
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

// We could implement methods only on Point<f32>
impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Note: Generic type parameters in a struct definition aren’t always the same as those you 
//       use in that same struct’s method signatures
impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

pub trait Summary {
    fn summarize(&self) -> String;

    //-------------------------------------------
    // Alternatively, the default implementation

    // Default implementations can call other methods in the same trait, even if those other 
    // methods don’t have a default implementation

    // fn summarize_author(&self) -> String;

    // fn summarize(&self) -> String {
    //     format!("(Read more from {}...)", self.summarize_author())
    // }
}

//--------------------------------
// Implementing a trait on a type

// One restriction to note is that we can implement a trait on a type only if either the 
// trait or the type, or both, are local to our crate

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// Alternatively, use the default implementation
// impl Summary for NewsArticle {}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

//-------------------------------------------------------
// Using Trait Bounds to Conditionally Implement Methods

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

//-------------------------------
// Lifetime in struct definition

// This struct has the single field part that holds a string slice, which is a reference. This
// annotation means an instance of ImportantExcerpt can’t outlive the reference it holds in 
// its part field.

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {

    // Because of the first elision rule, we’re not required to annotate the lifetime of the 
    // reference to `self`

    // The first lifetime elision rule in Rust states: "Each parameter that is a reference gets 
    // its own lifetime parameter."

    // When the Rust compiler parses a function or method signature, it automatically assigns a 
    // distinct, unique lifetime to every input variable that is a reference

    fn level(&self) -> i32 {
        3
    }

    // 1. Rust applies the first lifetime elision rule and gives both `&self` and `announcement` 
    //    their own lifetimes. 
    // 2. Then, because one of the parameters is `&self`, the return type gets the lifetime of 
    //    `&self`
    
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

//--------------------------------
// Higher-Rank Trait Bound (HRTB)

// for<'a> can be read as “for all choices of 'a”, and basically produces an infinite list of 
// trait bounds that F must satisfy

struct Closure<F> {
    data: (u8, u16),
    func: F,
}

impl<F> Closure<F>
    where for<'a> F: Fn(&'a (u8, u16)) -> &'a u8,
{
    fn call(&self) -> &u8 {
        (self.func)(&self.data)
    }
}

fn do_it(data: &(u8, u16)) -> &u8 { &data.0 }

fn main() {

    //--------------------
    // Generic data types
    //--------------------

    //---------------------
    // Function definition

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");

    //---------------------
    // Struct definition

    let integer = Point { x: 5,   y: 10.7 };
    let float   = Point { x: 1.0, y: 4.0 };

    //-----------------
    // Enum definition

    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    //-------------------
    // Method definition

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    //--------
    // Traits
    //--------

    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    };

    println!("1 new post: {}", post.summarize());

    notify(&post);

    //-----------
    // Lifetimes
    //-----------

    // Lifetime Annotation Syntax

    // Annotations are meant to tell Rust how generic lifetime parameters of multiple 
    // references relate to each other

    // &i32        // a reference
    // &'a i32     // a reference with an explicit lifetime
    // &'a mut i32 // a mutable reference with an explicit lifetime

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");

    // ❌ Attempting to use result after string2 has gone out of scope

    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {result}");

    //------------------
    // Lifetime ellison

    // The compiler uses three rules to figure out the lifetimes of the references when there
    // aren’t explicit annotations.

    // 1. The compiler assigns a lifetime parameter to each parameter that’s a reference
    // 2. If there is exactly one input lifetime parameter, that lifetime is assigned to all 
    //    output lifetime parameters
    // 3. If there are multiple input lifetime parameters, but one of them is `&self` or 
    //    `&mut self` because this is a method, the lifetime of `self` is assigned to all output 
    //    lifetime parameters

    //-------------------------
    // Higher-Rank Trait Bound

    let clo = Closure { data: (0, 1), func: do_it };
    println!("{}", clo.call());

    //---------
    // Subtype

    let hello: &'static str = "hello";
    {
        let world = String::from("world");
        let world = &world;     // 'world has a shorter lifetime than 'static
        debug(hello, world);    // hello silently downgrades from `&'static str` into `&'world str`
    }

    //----------
    // Variance

    {
        let mut hello: &'static str = "hello";
        {
            let world = String::from("world");

            // ❌ We cannot assume &'static str can still be downgraded into &'world str
            // to satisfy T, once it’s behind a &mut reference. This means that &mut &'static str 
            // cannot be a subtype of &mut &'world str, even if 'static is a subtype of 'world

            // assign(&mut hello, &world);
        }
        println!("{hello}"); // use after free 😿
    }

    {
        let hello: &'static str = "hello";
        
        demo("hello", store); // "hello" is 'static. Can call `store` fine

        {
            let smuggle = String::from("smuggle");

            // ❌ `&smuggle` is not static. If we were to call `store` with `&smuggle`,
            // we would have pushed an invalid lifetime into the `STATIC_VECTOR`.
            // Therefore, `fn(&'static str)` cannot be a subtype of `fn(&'a str)`

            // `store` cannot accept a string reference that lives for any duration less 
            // than 'static

            // demo(&smuggle, store);
        }

        // use after free 😿
        STATIC_VECTOR.with_borrow(|v| println!("{v:?}"));
    }

    //------------
    // Drop check

    // For a generic type to soundly implement drop, its generics arguments must 
    // strictly outlive it

    {
        let mut world = World {
            inspector: None,
            days: Box::new(1),
        };
        world.inspector = Some(Inspector(&world.days));

        // Let's say `days` happens to get dropped first.
        // Then when Inspector is dropped, it will try to read free'd memory!
    }

    //------------------
    // Splitting borrow

    // The borrow checker (a.k.a. borrowck) does understand structs sufficiently to 
    // know that it’s possible to borrow disjoint fields of a struct simultaneously

    {
        let mut x = Foo {a: 0, b: 0, c: 0};
        let a = &mut x.a;
        let b = &mut x.b;
        let c = &x.c;
        *b += 1;
        let c2 = &x.c;
        *a += 10;
        println!("{} {} {} {}", a, b, c, c2);
    }
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

//--------------------
// Trait bound syntax

// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

//-----------------------------------------
// Multiple Trait Bounds with the + Syntax

// pub fn notify(item: &(impl Summary + Display)) {
// pub fn notify<T: Summary + Display>(item: &T) {

//-----------------------------------------
// Clearer Trait Bounds with where Clauses

// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {

//---------------------------------------
// Returning Types That Implement Traits

// Note: you can only use impl Trait if you’re returning a single type

fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    }
}

// We want the signature to express the following constraint: The returned reference will be 
// valid as long as both of the parameters are valid. This is the relationship between 
// lifetimes of the parameters and the return value. We’ll name the lifetime 'a and then add 
// it to each reference

// The borrow checker should reject any values that don’t adhere to these constraints

// The generic lifetime 'a will get the concrete lifetime that is equal to the smaller of the
// lifetimes of x and y. Because we’ve annotated the returned reference with the same lifetime
// parameter 'a, the returned reference will also be valid for the length of the smaller of 
// the lifetimes of x and y.

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// Note: We’ve specified a lifetime parameter 'a for the parameter x and the return type, but 
// not for the parameter y, because the lifetime of y does not have any relationship with the 
// lifetime of x or the return value.

// fn longest<'a>(x: &'a str, y: &str) -> &'a str {
//     x
// }

fn debug<'a>(a: &'a str, b: &'a str) {
    println!("a = {a:?} b = {b:?}");
}

fn assign<T>(input: &mut T, val: T) {
    *input = val;
}

// thread_local! defines a static variable that is instantiated per OS thread, rather than being 
// shared across the entire program
thread_local! {
    pub static STATIC_VECTOR: RefCell<Vec<&'static str>> = RefCell::new(Vec::new());
}

/// Save the input given into a thread local `Vec<&'static str>`
fn store(input: &'static str) {
    STATIC_VECTOR.with_borrow_mut(|v| v.push(input));
}

/// Call the function with it's input (must have the same lifetime!)
fn demo<'a>(input: &'a str, f: fn(&'a str)) {
    f(input);
}

struct Inspector<'a>(&'a u8);

// ❌ If we add a destructor, the program will no longer compile!

// impl<'a> Drop for Inspector<'a> {
//     fn drop(&mut self) {
//         println!("I was only {} days from retirement!", self.0);
//     }
// }

struct World<'a> {
    inspector: Option<Inspector<'a>>,
    days: Box<u8>,
}

// 'a is unused within the struct’s body, it’s unbounded. Because of the troubles 
// this has historically caused, unbounded lifetimes and types are forbidden in 
// struct definitions. Therefore we must somehow refer to these types in the body. 
// Correctly doing this is necessary to have correct variance and drop checking

// We do this using PhantomData, which is a special marker type. PhantomData consumes
// no space, but simulates a field of the given type for the purpose of static analysis

struct Iter<'a, T: 'a> {
    ptr: *const T,
    end: *const T,
    _marker: marker::PhantomData<&'a T>,
}

struct Foo {
    a: i32,
    b: i32,
    c: i32,
}
