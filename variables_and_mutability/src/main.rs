// Constants aren’t just immutable by default. They’re always immutable.
const ONE_DAY_IN_SECONDS: u32 = 24 * 60 * 60;

fn main() {
    // x is immutable by default
    let x = 5;
    println!("The value of x is: {x}");
    // x = 6;
    // ^^^^^ cannot assign twice to immutable variable

    // using `mut` keyword makes x mutable
    let mut x = 5;
    println!("The initial value of x is: {x}");
    // we can now assign a new value to x
    x = 6;
    println!("The new value of x is: {x}");

    println!("The value of ONE_DAY_IN_SECONDS is: {ONE_DAY_IN_SECONDS}");

    /*
    Shadowing
    */
    let y = 100;
    let y = y + 1;
    {
        // these braces are used to create a block
        // Whatever happens in this block stays in this block
        let y = y * 2;
        println!("The value of y inside the block is: {y}");
    }
    println!("The value of y outside the block is: {y}");

    // We can also change the type while variable name shadowing
    let spaces: &str = "   ";
    let spaces: usize = spaces.len();
    println!("The value of spaces is: {spaces}");

    // but we can not do this
    // let mut spaces: &str = "    ";
    // spaces = spaces.len(); // error: we can not change the type of variable
}
