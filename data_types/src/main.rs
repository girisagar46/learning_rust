#![allow(unused_variables)]

fn main() {
    /*
    Rust has four primary scalar types:
    1. integers,
    2. floating-point numbers,
    3. Booleans, and
    4. characters.
     */

    /* Integres */

    /*
    Length	Signed	Unsigned
    8-bit	i8	    u8
    16-bit	i16	    u16
    32-bit	i32	    u32
    64-bit	i64	    u64
    128-bit	i128	u128
    arch	isize	usize (arch means depends on the kind of computer your program is running on: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.)
     */

    // if we do not specify the type `u32`, it won't compile
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{}", guess);

    // Each signed variant can store numbers from [-2^(n-1) to 2^(n-1)-1] inclusive
    // i8 can store numbers from [-(2^7) to (2^7)-1], which equals -128 to 127.

    /* Floating-Point Numbers */
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    /* Operations */

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    /* boolean types */
    let t = true;

    let f: bool = false; // with explicit type annotation

    /* Char types */
    // char types are defined with single quotes
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    /* Compound types */
    // Rust has two primitive compound types: tuples and arrays.

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // we can also destructure the tuple
    let (x, y, z) = tup;

    // we can access the tuple elements using the dot notation
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // for mutable tuple, we can modify the values
    let mut x: (i32, i32) = (1, 2);
    x.0 = 0;
    x.1 += 5;
    println!("{:?}", x); // Output: (0, 7)

    // Array
    let a = [1, 2, 3, 4, 5];
    // Arrays are more useful when you know the number of elements.
    // Arrays can't grow or shrink in size.
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // we can also define size while declaring an array
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let b = [3; 5]; // This will create an array of size 5 with all elements as 3

    // Accessing array elements

    let first = a[0];
    let second = a[1];

    // Accessing an invalid index will result in a panic
    // let elem = a[10]; // This will panic
    // ^^^^^ index out of bounds: the length is 5 but the index is 10
}
