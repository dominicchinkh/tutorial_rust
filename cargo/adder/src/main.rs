// cargo run -p adder

use rand;

fn main() {
    let num = 10;
    println!("Hello, world! {num} plus one is {}!", unsafe { add_one::add_one(&num) }.unwrap_or_else(|_| 0));
}