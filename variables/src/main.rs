fn main() {

    //----------------------------------------------------------------------------
    // ❌ By default, variables are immutable. When a variable is immutable, once 
    //     a value is bound to a name, you can’t change that value.
    // let x = 5;
    // x = 6;

    //----------------------------------------------------------------------------
    // You can make variables mutable by adding mut in front of the variable name
    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

    //-------------------------------------------------------------------------------
    // `constants` are values that are bound to a name and are not allowed to change
    // 1. You aren’t allowed to use mut with constants
    // 2. Constants can be declared in any scope, including the global scope
    // 3. Constants may be set only to a constant expression, not the result of a value 
    //    that could only be computed at runtime
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");

    //-------------------------------------------------------------------------------
    // You can declare a new variable with the same name as a previous variable. the 
    // first variable is shadowed by the second, which means that the second variable is 
    // what the compiler will see when you use the name of the variable, taking any uses 
    // of the variable name to itself until either it itself is shadowed or the scope ends
    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of x in the inner scope is: {y}");
    }

    println!("The value of x is: {y}");
}
