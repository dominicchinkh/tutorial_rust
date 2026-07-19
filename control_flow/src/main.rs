fn main() {

    //----
    // if

    let number = 6;

    // ❌ Rust will not automatically try to convert non-Boolean types to a Boolean
    // if number {

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // `if` is an expression

    // Note: the values that have the potential to be results from each arm of the if 
    // must be the same type

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    //------
    // loop

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // Pass the result of that operation out of the loop to the rest of your code
            break counter * 2;
        }
    };

    println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop {
        let mut remaining = 10;

        loop {
            if remaining == 9 {
                break;
            }
            if count == 2 {
                // `break` and `continue` apply to the innermost loop at that point

                // Specify a loop label to specify that `break` applies to the labeled loop instead 
                // of the innermost loop

                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    //-------
    // while

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    //-----
    // for

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    // `1..4` generates all numbers in sequence starting from 1 and ending before 4

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
