mod exercise;

fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number1 = 3;
    // if conditions must be of type bool
    // if number1 {
    //     ^^^^^^^ expected `bool`, found integer
    //     println!("number was three");
    // }
    if number1 != 0 {
        println!("number was something other than zero");
    }

    // comprehensive if-else
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
    // where as this will give error
    // let number = if condition { 5 } else { "six" };  // expected integer, found &str

    /* Repetitions with loops */
    // ⚠️️ This is an infinite loop
    // loop {
    //     println!("again!");
    // }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // `break` is very similar to `return`
        }
    };

    println!("The result is {result}");

    // Loop Labels to Disambiguate Between Multiple Loops
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                println!("Breaking the outer loop");
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // Conditional loop
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }
    println!("LIFTOFF!!!");

    // looping over a collection
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    // Chapter exercise
    println!("32°F is {}°C", exercise::tempurature_conversion(32.0, "f"));
    println!("0°C is {}°F", exercise::tempurature_conversion(0.0, "c"));
    println!("5th fibonacci number is {}", exercise::nth_fibonacci(5));
}
