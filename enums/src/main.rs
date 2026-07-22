#[allow(dead_code)]
enum IpAddr {
    // You can put any kind of data inside an enum variant: strings, numeric types, or structs, for example. You 
    // can even include another enum
    V4(u8, u8, u8, u8),
    V6(String),
}

#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 }, // Note: Has named fields
    Write(String),
    ChangeColor(i32, i32, i32),
}

// We are able to define methods on enums
impl Message {
    fn call(&self) {
        match self {
            Message::Quit => {
                println!("Quit: Shutting down.");
            }
            Message::Move { x, y } => {
                println!("Move: Moving to coordinates x: {}, y: {}", x, y);
            }
            Message::Write(text) => {
                println!("Write: {}", text);
            }
            Message::ChangeColor(r, g, b) => {
                println!("ChangeColor: Setting color to RGB({}, {}, {})", r, g, b);
            }
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
enum UsState {
    Alabama,
    Alaska
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska  => year >= 1959,
        }
    }
}

#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let _home = IpAddr::V4(127, 0, 0, 1);
    let _loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    //-------------
    // Option enum

    // Rust can infer these types because we’ve specified a value inside the `Some` variant. For `absent_number`, 
    // Rust requires us to annotate the overall Option type

    let _some_number = Some(5);
    let _some_char = Some('e');

    let _absent_number: Option<i32> = None;

    //------------------------------
    // Patterns That Bind to Values

    // When a Coin::Quarter matches, the `state` variable will bind to the value of that quarter’s `state`
    let _coin = value_in_cents(Coin::Quarter(UsState::Alaska));

    //-----------------------------
    // The Option<T> match Pattern

    let five = Some(5);
    let six  = plus_one(five);
    let none = plus_one(None);

    println!("{:?} {:?} {:?}", five, six, none);

    //------------------------------------------
    // Catch-All Patterns and the _ Placeholder

    let dice_roll = 5;

    match dice_roll {
        3 => println!("Found 3!"),

        // For the last arm that covers every other possible value, the pattern is the variable we've chosen 
        // to name `other`
        other => println!("Other number {other}"),

        // We can use `_` when we want a catch-all but don't want to use the value in the catch-all pattern
        // We can use `()` when we don’t want to run any code in this case

        // _ => (),
    }

    //-----------------------
    // if let and let...else

    // Using if let means less typing, less indentation, and less boilerplate code. However, you lose the 
    // exhaustive checking match enforces that ensures that you aren’t forgetting to handle any cases.

    let config_max = Some(3u8);

    // Same as:
    //     match config_max {
    //         Some(max) => println!("The maximum is configured to be {max}"),
    //         _ => (),
    //     }

    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    let mut _count = 0;
    let coin = Coin::Quarter(UsState::Alaska);

    // Same as:
    //     match coin {
    //         Coin::Quarter(state) => println!("State quarter from {state:?}!"),
    //         _ => _count += 1,
    //     }

    if let Coin::Quarter(ref state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        _count += 1;
    }

    //------------
    // let...else

    println!("{:?}", describe_state_quarter(coin));
}

fn value_in_cents(coin: Coin) -> u8 {

    // Note: The arms' patterns must cover all possibilities
    match coin {
        Coin::Penny  => 1,
        Coin::Nickel => 5,
        Coin::Dime   => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {

    // Note: The arms' patterns must cover all possibilities
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {

    // The `let...else` syntax takes a pattern on the left side and an expression on the right, very similar 
    // to `if let`, but it does not have an if branch, only an else branch. If the pattern matches, it will 
    // bind the value from the pattern in the outer scope. If the pattern does not match, the program will 
    // flow into the `else` arm.
    
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}
