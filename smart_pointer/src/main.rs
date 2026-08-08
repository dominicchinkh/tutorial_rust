use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};

// ❌ We’ve defined List with a variant that is recursive: It holds another 
// value of itself directly. As a result, Rust can’t figure out how much 
// space it needs to store a List value

// enum List {
//     Cons(i32, List),
//     Nil,
// }

// A Box<T> is a pointer, Rust always knows how much space a Box<T> needs: 
// A pointer’s size doesn’t change based on the amount of data it’s pointing
// to

enum BoxList {
    Cons(i32, Box<BoxList>),
    Nil,
}

enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}

// A Rc<T> that holds a RefCell<T>, you can get a value that can have multiple 
// owners and that you can mutate!

#[derive(Debug)]
enum RcRefCellList {
    Cons(Rc<RefCell<i32>>, Rc<RcRefCellList>),
    Nil,
}

#[derive(Debug)]
enum RefCellList {
    Cons(i32, RefCell<Rc<RefCellList>>),
    Nil,
}

impl RefCellList {
    fn tail(&self) -> Option<&RefCell<Rc<RefCellList>>> {
        match self {
            RefCellList::Cons(_, item) => Some(item),
            RefCellList::Nil => None,
        }
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    // The reason the deref method returns a reference to a value:
    //   If the deref method returned the value directly instead of a
    //   reference to the value, the value would be moved out of self.
    //   We don’t want to take ownership of the inner value inside 
    //   MyBox<T> in this case or in most cases where we use the 
    //   dereference operator

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {

    //-------------------------------------------
    // Using Box<T> to Point to Data on the Heap
    //-------------------------------------------

    // Boxes allow you to store data on the heap rather than the stack

    // You’ll use them most often in these situations
    // 1. When you have a type whose size can’t be known at compile time,
    //    and you want to use a value of that type in a context that 
    //    requires an exact size
    // 2. When you have a large amount of data, and you want to transfer 
    //    ownership but ensure that the data won’t be copied when you do 
    //    so
    // 3. When you want to own a value, and you care only that it’s a type
    //    that implements a particular trait rather than being of a specific
    //    type

    let b = Box::new(5);
    println!("b = {b}");

    let list = BoxList::Cons(1, 
                Box::new(BoxList::Cons(2, 
                    Box::new(BoxList::Cons(3, 
                        Box::new(BoxList::Nil))))));

    //-------------------------------------------------
    // Treating Smart Pointers Like Regular References
    //-------------------------------------------------

    // Deref trait allows you to customize the behavior of the dereference 
    // operator *

    let x = 5;

    // The main difference with `let y = &x;` is that here we set y to be an
    // instance of a box pointing to a copied value of x rather than a 
    // reference pointing to the value of x

    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);

    // Behind the scenes Rust actually ran this code: `*(y.deref())`
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));

    // 1. Call the hello function with the argument &m
    // 2. Rust turns `&MyBox<String>` into `&String` by calling deref
    // 3. The standard library provides an implementation of Deref on String
    //    that returns a string slice. Rust calls deref again to turn the 
    //    &String into &str

    hello(&m);

    // You can use the DerefMut trait to override the * operator on mutable 
    // references

    // Rust will also coerce a mutable reference to an immutable one. But 
    // the reverse is not possible: Immutable references will never coerce 
    // to mutable references

    //   From &mut T to &U when T: Deref<Target=U>
    
    //---------------------------------------------
    // Running Code on Cleanup with the Drop Trait
    //---------------------------------------------

    {
        let c = CustomSmartPointer {
            data: String::from("my stuff"),
        };

        let d = CustomSmartPointer {
            data: String::from("other stuff"),
        };

        println!("CustomSmartPointers created");

        // Variables are dropped in the reverse order of their creation
    }

    {
        let e = CustomSmartPointer {
            data: String::from("some data"),
        };
        println!("CustomSmartPointer created");

        // We call `std::mem::drop` by passing as an argument the value we want 
        // to force-drop
        drop(e);

        println!("CustomSmartPointer dropped before the end of main");
    }

    //--------------------------------------------
    // Rc<T>, the Reference-Counted Smart Pointer
    //--------------------------------------------

    {
        let a = Rc::new(RcList::Cons(5, 
                    Rc::new(RcList::Cons(10, 
                        Rc::new(RcList::Nil)))));
        
        // Rust’s convention is to use Rc::clone in this case. The call to 
        // Rc::clone only increments the reference count

        let b = RcList::Cons(3, Rc::clone(&a));
        let c = RcList::Cons(4, Rc::clone(&a));
    }

    {
        let a = Rc::new(RcList::Cons(5, 
                    Rc::new(RcList::Cons(10, 
                        Rc::new(RcList::Nil)))));

        println!("count after creating a = {}", Rc::strong_count(&a));

        {
            let b = RcList::Cons(3, Rc::clone(&a));
            println!("count after creating b = {}", Rc::strong_count(&a));

            {
                let c = RcList::Cons(4, Rc::clone(&a));
                println!("count after creating c = {}", Rc::strong_count(&a));
            }
            println!("count after c goes out of scope = {}", Rc::strong_count(&a));
        }

        println!("count after b goes out of scope = {}", Rc::strong_count(&a));
    }

    //------------------------------------------------
    // RefCell<T> and the Interior Mutability Pattern
    //------------------------------------------------

    // Refer to lib.rs

    // 1. RefCell<T> have single owners
    // 2. RefCell<T> allows immutable or mutable borrows checked at runtime
    // 3. You can mutate the value inside the RefCell<T> even when the 
    //    RefCell<T> is immutable

    // The RefCell<T> keeps track of how many Ref<T> and RefMut<T> smart pointers are 
    // currently active. Every time we call borrow, the RefCell<T> increases its count 
    // of how many immutable borrows are active. When a Ref<T> value goes out of scope, 
    // the count of immutable borrows goes down by 1

    // RefCell<T> lets us have many immutable borrows or one mutable borrow at any point 
    // in time

    //------------------------------------------
    // Allowing Multiple Owners of Mutable Data

    {
        let value = Rc::new(
            RefCell::new(5)
        );

        let a = Rc::new(
            RcRefCellList::Cons(
                Rc::clone(&value), 
                Rc::new(RcRefCellList::Nil)
            )
        );

        let b = RcRefCellList::Cons(
            Rc::new(RefCell::new(3)), 
            Rc::clone(&a)
        );
        let c = RcRefCellList::Cons(
            Rc::new(RefCell::new(4)), 
            Rc::clone(&a)
        );

        *value.borrow_mut() += 10;

        println!("a after = {a:?}");
        println!("b after = {b:?}");
        println!("c after = {c:?}");
    }

    //----------------------------------
    // Reference Cycles Can Leak Memory
    //----------------------------------

    // Rust’s memory safety guarantees make it difficult, but not impossible, to 
    // accidentally create memory that is never cleaned up (known as a memory leak)

    //----------------------------
    // Creating a Reference Cycle

    {
        // 1. a -> (5, Nil)
        let a = Rc::new(
            RefCellList::Cons(
                5, 
                RefCell::new(Rc::new(RefCellList::Nil))
            )
        );

        println!("a initial rc count = {}", Rc::strong_count(&a));
        println!("a next item = {:?}", a.tail());

        // 2. b -> (10, a)
        let b = Rc::new(
            RefCellList::Cons(
                10, 
                RefCell::new(Rc::clone(&a))
            )
        );

        println!("a rc count after b creation = {}", Rc::strong_count(&a));
        println!("b initial rc count = {}", Rc::strong_count(&b));
        println!("b next item = {:?}", b.tail());

        // 3. a -> (5, b)
        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }

        println!("b rc count after changing a = {}", Rc::strong_count(&b));
        println!("a rc count after changing a = {}", Rc::strong_count(&a));

        // The reference count of the Rc<List> instances in both a and b is 2 after 
        // we change the list in a to point to b. At the end of main, Rust drops the 
        // variable b, which decreases the reference count of the b Rc<List> instance 
        // from 2 to 1. The memory that Rc<List> has on the heap won’t be dropped at 
        // this point because its reference count is 1, not 0. Then, Rust drops a, 
        // which decreases the reference count of the a Rc<List> instance from 2 to 1 
        // as well. This instance’s memory can’t be dropped either, because the other 
        // Rc<List> instance still refers to it

        // ❌ Uncomment the next line to see that we have a cycle;
        // it will overflow the stack.
        // println!("a next item = {:?}", a.tail());
    }

    //-------------------------------------------
    // Preventing Reference Cycles Using Weak<T>

    // Create a weak reference to the value within an Rc<T> instance by calling 
    // Rc::downgrade and passing a reference to the Rc<T>

    // Instead of increasing the strong_count in the Rc<T> instance by 1, calling 
    // Rc::downgrade increases the weak_count by 1

    // Weak references count doesn’t affect when an Rc<T> instance is cleaned up. They
    // won’t cause a reference cycle, because any cycle involving some weak references 
    // will be broken once the strong reference count of values involved is 0

    // To do anything with the value that a Weak<T> is pointing to you must make sure 
    // the value still exists. Do this by calling the upgrade method on a Weak<T> instance, 
    // which will return an Option<Rc<T>>

    {
        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );

        {
            let branch = Rc::new(Node {
                value: 5,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf)]),
            });

            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

            println!(
                "branch strong = {}, weak = {}",
                Rc::strong_count(&branch),
                Rc::weak_count(&branch),
            );

            println!(
                "leaf strong = {}, weak = {}",
                Rc::strong_count(&leaf),
                Rc::weak_count(&leaf),
            );
        }

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}
