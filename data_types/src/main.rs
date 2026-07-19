fn main() {
    //-------------
    // Scalar type

    // Integer types

    // | Length	                | Signed | Unsigned
    // |------------------------|--------|---------
    // | 8-bit	                | i8	 | u8
    // | 16-bit	                | i16	 | u16
    // | 32-bit	                | i32	 | u32
    // | 64-bit	                | i64	 | u64
    // | 128-bit	            | i128   | u128
    // | Architecture-dependent | isize  | usize

    // Integer literals
    let _decimal = 98_222;
    let _hex     = 0xff;
    let _octal   = 0o77;
    let _binary  = 0b1111_0000;
    let _byte    = b'A';

    // Floating point types
    let _f32: f32 = 2.0;
    let _f64: f64 = 3.145;

    // Boolean type
    let _t       = true;
    let _f: bool = false;       // with explicit type annotation

    // Character type

    // Specify char literals with single quotation marks, as opposed to string literals, 
    // which use double quotation marks

    let _lowercase_z       = 'z';
    let _uppercase_z: char = 'Z';          // with explicit type annotation

    // Rust’s char type is 4 bytes in size and represents a Unicode scalar value
    let _heart_eyed_cat = '😻';

    //---------------
    // Compound type

    // Tuple type

    // A tuple is a general way of grouping together a number of values with a variety 
    // of types into one compound type. Tuples have a fixed length: Once declared, they 
    // cannot grow or shrink in size.

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Destructuring 
    let (_x, _y, _z) = tup;

    // We can also access a tuple element directly by using a period (.) followed by the 
    // index of the value we want to access
    let _five_hundred   = tup.0;
    let _six_point_four = tup.1;
    let _one            = tup.2;

    // Array type

    // 1. Every element of an array must have the same type. 
    // 2. Arrays in Rust have a fixed length.
    let _a = [1, 2, 3, 4, 5];

    // Write an array’s type using square brackets with the type of each element, a 
    // semicolon, and then the number of elements in the array
    let _a: [i32; 5] = [1, 2, 3, 4, 5];

    // Initialize an array to contain the same value for each element by specifying the 
    // initial value, followed by a semicolon, and then the length of the array in square 
    // brackets
    let a = [3; 5];

    // Access elements of an array using indexing
    let _first  = a[0];
    let _second = a[1];
}
