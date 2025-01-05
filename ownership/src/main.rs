#![allow(unused_variables)]

fn read(y: bool) {
    if y {
        let result = y;
        println!("result is {}!", result);
    } // Beyond this point, the variable `result` goes out of scope and is dropped.
}

fn main() {
    // ---- Ownership rules----
    // 1. Each value in Rust has a variable that’s called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.
    let x = true;
    read(x);

    let s1 = String::from("hello"); // Since String can grow or shrink, it is stored on the heap.
                                    // Here s1 is not copied or s2 pointer is not pointing to the heap location of s2.
                                    // Instead, s1 is moved to s2, and s1 is no longer valid (i.e. dropped).
    let s2 = s1;

    // println!("s1 is {}", s1); // This will give an error because s1 is no longer valid.
    //                      ^^ value borrowed here after move
    println!("s2 is {}", s2);

    // We can clone the value if we still want to use the value of s1.
    let s3 = s2.clone(); // This will create a deep copy of s2 and won't drop s2.
    println!("s3 is {}", s3);

    // For integers, booleans & char the copy trait is implemented, so the value is copied instead of moved.
    let a = 5;
    let b = a;
    println!("a is {}, b is {}", a, b); // This will work because a is copied instead of moved.

    let c = true;
    let d = c;
    println!("c is {}, d is {}", c, d); // This will work because c is copied instead of moved.

    let e = 'a';
    let f = e;
    println!("e is {}, f is {}", e, f); // This will work because e is copied instead of moved.

    // Ownership and functions
    let s4 = String::from("hello");
    takes_ownership(s4); // s4 is moved to the function and is no longer valid.
                         // println!("s4 is {}", s4); // This will give an error because s4 is no longer valid, because it is moved to the function.

    let x: i32 = 5;
    makes_copy(x); // x is copied to the function and is still valid.
    println!("x is still {} because it was copied", x); // This will work because x is copied instead of moved.

    let s5 = gives_ownership(); // The ownership of the value is moved to s5.
    println!("s5 is {}", s5);

    // --- References
    // The Rules of References:
    // 1. At any given time, you can have either one mutable reference or any number of immutable references.
    // 2. References must always be valid.
    reference_example();
    mutable_reference_example();
    let mut s6 = String::from("hello");
    let r1 = &s6; // `r1` stores the reference of `s6`.
    let r2 = &s6; // `r2` stores the reference of `s6`.
    println!("r1 is {}, r2 is {}", r1, r2);  // Scope of r1 and r2 ends here, so we can re-define a mutable reference to s6 below this point.
    let r3 = &mut s6; // `r3` stores the mutable reference of `s6`.
    println!("r3 is {}", r3);

    // --- Slices
    // Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.

    let s7 = String::from("hello world");
    let hello = &s7[..5]; // This will create a slice of the string from index 0 to 5.
    let world = &s7[6..]; // This will create a slice of the string from index 6 to 11.
    let entire_string = &s7[..]; // This will create a slice of the entire string.

    let list: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &list[1..3]; // This will create a slice of the array from index 1 to 3.
    println!("Slice is {:?}", slice);
}

fn takes_ownership(s: String) {
    println!(
        "value of s inside the `takes_ownership` function is \"{}\"",
        s
    );
} // Here s goes out of scope and is dropped.

fn makes_copy(x: i32) {
    println!("value of x inside the `makes_copy` function is {}", x);
} // Here x goes out of scope and is dropped.

fn gives_ownership() -> String {
    let s = String::from("hello");
    s // The ownership of the value is moved to the calling function.
} // Here s goes out of scope and is dropped.

fn reference_example() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // We are passing the reference of s1 to the function.
    println!("The length of '{}' is {}.", s1, len); // Here we can still use s1 because we passed the reference to the function.
}

// s is a reference to a String. This is called borrowing (i.e. we are borrowing the value of s1 but not taking the ownership of it).
fn calculate_length(s: &String) -> usize {
    // NOTE: s.push_str(" world"); // This will give an error because references are immutable by default.
    s.len()
} // Here s goes out of scope but since it is a reference, it is not dropped.


fn mutable_reference_example() {
    let mut s = String::from("hello");
    let r1 = &mut s; // `r1` stores the mutable reference of `s`.
    // let r2 = &mut s; // This will give an error because we can't have multiple mutable references to the same value.
    change(r1); // We are passing the mutable reference of s to the function.
    println!("The value of s after calling the `change` function is \"{}\"", s);
}

fn change(s: &mut String) {
    s.push_str(" world"); // This will work because we are passing the mutable reference of s.
} // Here s goes out of scope but since it is a mutable reference, it is not dropped.
