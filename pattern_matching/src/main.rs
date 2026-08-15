struct Point2D {
    x: i32,
    y: i32,
}

struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn main() {

    //-------------------------------------
    // All the Places Patterns Can Be Used
    //-------------------------------------

    //------------
    // match arms
    
    {
        let x = Some(5);

        let y = match x {
            Some(i) => Some(i + 1),
            None => None
        };

        println!("y is {}", y.unwrap());
    }

    // Matching Literals

    {
        let x = 1;

        match x {
            1 => println!("one"),
            2 => println!("two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }

    // Matching Named Variables

    {
        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("Got 50"),

            // Because we’re in a new scope inside the match expression, this is a new y
            // variable, not the y we declared at the beginning with the value 10

            Some(y) => println!("Matched, y = {y}"),
            _ => println!("Default case, x = {x:?}"),
        }

        println!("at the end: x = {x:?}, y = {y}");
    }

    // Matching Multiple Patterns

    {
        let x = 1;

        match x {
            1 | 2 => println!("one or two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }

    // Matching Ranges of Values with ..=

    {
        // Note: ranges are only allowed with numeric or char values

        let x = 5;

        match x {
            1..=5 => println!("one through five"),
            _ => println!("something else"),
        }

        let x = 'c';

        match x {
            'a'..='j' => println!("early ASCII letter"),
            'k'..='z' => println!("late ASCII letter"),
            _ => println!("something else"),
        }
    }

    // struct

    {
        let p = Point2D { x: 0, y: 7 };

        // We can destructure with literal values as part of the struct pattern rather
        // than creating variables for all the fields

        match p {
            Point2D { x, y: 0 } => println!("On the x axis at {x}"),  // on the x axis
            Point2D { x: 0, y } => println!("On the y axis at {y}"),  // on the y axis
            Point2D { x, y } => {                                     // neither
                println!("On neither axis: ({x}, {y})");
            }
        }
    }

    // enum

    {
        // let msg = Message::Quit;
        // let msg = Message::Move { x: 4, y: 7 };
        // let msg = Message::Write(String::from("Hello world!"));
        let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

        match msg {

            // For enum variants without any data, like Message::Quit, we can’t destructure
            // the value any further. We can only match on the literal Message::Quit value, 
            // and no variables are in that pattern

            Message::Quit => {
                println!("The Quit variant has no data to destructure.");
            }
            Message::Move { x, y } => {
                println!("Move in the x direction {x} and in the y direction {y}");
            }
            Message::Write(text) => {
                println!("Text message: {text}");
            }

            // Nested structs and enums:

            Message::ChangeColor(Color::Rgb(r, g, b)) => {
                println!("Change color to red {r}, green {g}, and blue {b}");
            }
            Message::ChangeColor(Color::Hsv(h, s, v)) => {
                println!("Change color to hue {h}, saturation {s}, value {v}");
            }
        }
    }

    // Parts of a Value with a Nested _

    {
        let mut setting_value = Some(5);
        let new_setting_value = Some(10); // None;

        match (setting_value, new_setting_value) {

            // We test for the case when setting_value and new_setting_value are the
            // Some variant `Some`

            (Some(_), Some(_)) => {
                println!("Can't overwrite an existing customized value");
            }

            // If either setting_value or new_setting_value is None

            _ => {
                setting_value = new_setting_value;
            }
        }

        println!("setting is {setting_value:?}");
    }

    {
        let numbers = (2, 4, 8, 16, 32);

        match numbers {
            (first, _, third, _, fifth) => {
                println!("Some numbers: {first}, {third}, {fifth}");
            }
        }
    }

    // Remaining Parts of a Value with ..

    {
        // The .. pattern ignores any parts of a value that we haven’t explicitly matched
        // in the rest of the pattern. `..` must be unambiguous

        let origin = Point3D { x: 0, y: 0, z: 0 };

        match origin {
            Point3D { x, .. } => println!("x is {x}"),
        }

        let numbers = (2, 4, 8, 16, 32);

        match numbers {
            (first, .., last) => {
                println!("Some numbers: {first}, {last}");
            }
        }
    }

    // Adding Conditionals with Match Guards

    {
        // A match guard is an additional if condition, specified after the pattern 
        // in a match arm, that must also match for that arm to be chosen

        // The downside of this additional expressiveness is that the compiler doesn’t 
        // try to check for exhaustiveness when match guard expressions are involved

        let num = Some(4);

        match num {
            Some(x) if x % 2 == 0 => println!("The number {x} is even"),
            Some(x) => println!("The number {x} is odd"),
            None => (),
        }
    }

    {
        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("Got 50"),

            // The pattern in the second match arm doesn’t introduce a new variable y 
            // that would shadow the outer y, meaning we can use the outer y in the 
            // match guard

            Some(n) if n == y => println!("Matched, n = {n}"),
            _ => println!("Default case, x = {x:?}"),
        }

        println!("at the end: x = {x:?}, y = {y}");
    }

    {
        let x = 4;
        let y = false;

        match x {

            // The match condition states that the arm only matches if the value of x is 
            // equal to 4, 5, or 6 and if y is true, i.e. (4 | 5 | 6) if y => ...

            4 | 5 | 6 if y => println!("yes"),
            _ => println!("no"),
        }
    }

    // Using @ Bindings

    // The at operator @ lets us create a variable that holds a value at the same time
    // we’re testing that value for a pattern match

    {
        enum Message {
            Hello { id: i32 },
        }

        let msg = Message::Hello { id: 5 };

        match msg {
            Message::Hello { id: id @ 3..=7 } => {
                println!("Found an id in range: {id}")
            }
            Message::Hello { id: 10..=12 } => {
                println!("Found an id in another range")
            }
            Message::Hello { id } => println!("Found some other id: {id}"),
        }
    }

    //----------------
    // let Statements

    {
        let (x, y, z) = (1, 2, 3);
    }

    // Structs

    {
        let p = Point2D { x: 0, y: 7 };

        // Note that the names of the variables in the pattern don’t have to match the
        // field names of the struct. However, it’s common to match the variable names 
        // to the field names to make it easier to remember which variables came from 
        // which fields

        let Point2D { x: a, y: b } = p;
        assert_eq!(0, a);
        assert_eq!(7, b);

        // Rust has a shorthand for patterns that match struct fields: You only need to 
        // list the name of the struct field, and the variables created from the pattern 
        // will have the same names

        let Point2D { x, y } = p;
        assert_eq!(0, x);
        assert_eq!(7, y);
    }

    // Structs and tuple

    {
        // We can mix, match, and nest destructuring patterns in even more complex ways

        let ((feet, inches), Point2D { x, y }) = ((3, 10), Point2D { x: 3, y: -10 });
    }

    //--------------------------------
    // Conditional if let Expressions

    {
        let favorite_color: Option<&str> = None;
        let is_tuesday = false;
        let age: Result<u8, _> = "34".parse();

        // Note: Rust doesn’t require that the conditions in a series of if let, else if,
        // and else if let arms relate to each other

        if let Some(color) = favorite_color {
            println!("Using your favorite color, {color}, as the background");

        } else if is_tuesday {
            println!("Tuesday is green day!");

        // The line introduces a new `age` variable that contains the value inside the Ok
        // variant, shadowing the existing `age` variable

        } else if let Ok(age) = age {

            // The new `age` we want to compare to 30 isn’t valid until the new scope starts 
            // with the curly bracket

            if age > 30 {
                println!("Using purple as the background color");
            } else {
                println!("Using orange as the background color");
            }

        } else {
            println!("Using blue as the background color");
        }

        // The downside of using if let expressions is that the compiler doesn’t check for 
        // exhaustiveness, whereas with match expressions it does
    }

    //-----------------------------
    // while let Conditional Loops

    {
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            for val in [1, 2, 3] {
                tx.send(val).unwrap();
            }
        });

        // The while let conditional loop allows a while loop to run for as long as a
        // pattern continues to match

        // The recv method returns an Ok each time a message arrives, as long as the 
        // sender exists, and then produces an Err once the sender side disconnects

        while let Ok(value) = rx.recv() {
            println!("{value}");
        }
    }

    //-----------
    // for Loops

    {
        let v = vec!['a', 'b', 'c'];

        // The enumerate method produces a value and the index for that value, placed 
        // into a tuple. The first value produced is the tuple (0, 'a'). When this 
        // value is matched to the pattern (index, value), index will be 0 and value 
        // will be 'a'

        for (index, value) in v.iter().enumerate() {
            println!("{value} is at index {index}");
        }
    }

    //---------------------
    // Function Parameters

    {
        // The values &(3, 5) match the pattern &(x, y), so x is the value 3 and y 
        // is the value 5

        let point = (3, 5);
        print_coordinates(&point);
    }

    {
        foo(3, 4);
    }

    //-----------------------------------------------------
    // Refutability: Whether a Pattern Might Fail to Match
    //-----------------------------------------------------

    // Irrefutable: patterns that will match for any possible value passed. Function 
    // parameters, let statements, and for loops can only accept irrefutable patterns

    // Refutable: patterns that can fail to match for some possible value. The `if let` 
    // and `while let` expressions and the `let...else` statement accept refutable and 
    // irrefutable patterns

    {
        let some_option_value = Some(5);

        // ❌ Rust will complain that we’ve tried to use a refutable pattern where an 
        //    irrefutable pattern is required

        // let Some(x) = some_option_value;

        // ✅ If the pattern doesn’t match, the code in the curly brackets will handle 
        // the value

        let Some(x) = some_option_value else {
            return;
        };

        // ❌ Rust complains that it doesn’t make sense to use let...else with an 
        // irrefutable pattern

        // let x = 5 else {
        //     return;
        // };


    }
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({x}, {y})");
}

// Use the underscore as a wildcard pattern that will match any value but not bind to
// the value

fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {y}");
}
