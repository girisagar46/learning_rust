fn main() {
    println!("Hello, world!");
    another_function(5);

    let y = {
        let x = 3;
        x + 1 // Adding a semicolon here would turn this into a statement and won't assign return value to y
    };

    println!("The value of y is: {y}");

    println!("The value of five() is: {}", five());
    println!("The value of plus_one(5) is: {}", plus_one(5));
}

fn another_function(x: i32) {
    println!("Another function.");
    println!("The value of x is: {}", x);
}

// function with return value
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1 // This is implicit return. Notice there is no semicolon at the end of the line

    // If we want to do explicit return, we can do it like this
    // return x + 1;
}
