fn main() {
    // variables();
    //
    // function_1("Hector");
    // println!("{:?}", function_2());
    //
    // for_loops();

    // let foo = loop_loop();
    // println!("loop_loop() returned: {}", foo);

    // ownership_1();

    // ownership_2();

    // reference();

    pass_in_value_vs_pass_in_reference();
}

// variables
fn variables() {
    // default variables
    const A: i32 = 3;
    let b = 100.8;
    let c = 300;
    const FOO: &str = "A const";

    println!("Hello, world! {0}, {1}, {2}, {3}", A, b, c, FOO);

    // mutable variables
    let mut x: i32 = 5;
    x = x - 6;

    println!("The value of x is: {}", x);

    // + - * / %
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    println!("The results are: {0}, {1}, {2}, {3}, {4}, {5}", sum, difference, product, quotient, remainder, true);
}

// define the method signature. parameter has a type
fn function_1(name: &str) {
    println!("Hello, {}!", name);
}

// define the method signature. method has a return type
fn function_2() -> (i32, i32) {
    let x = 5;

    // The last line is the returned value. No semicolon for the last line
    let y = {
        let x = 4;
        x + 1
    };

    // conditional
    if x == y {
        // early return a tuple
        return (x, y);
    }

    // normal return a tuple
    return (7, 8);
}

fn for_loops() {
    // array
    let arr = [10, 20, 30, 40, 50];

    // borrow the value?????
    for i in &arr {
        println!("value: {}", i);
    }

    // iterator
    for i in arr.iter() {
        println!("value: {}", i);
    }

    // exclusive range
    for i in 0..5 {
        println!("value: {}", arr[i]);
    }

    //inclusive range
    for i in 0..=4 {
        println!("value: {}", arr[i]);
    }
}

fn loop_loop() -> char {
    let s = ['R', 'U', 'N', 'O', 'O', 'B'];
    let mut i = 0;

    // the value of loop is from break
    return loop {
        let ch = s[i];

        // if ch == 'O' {
        //     return ch;
        // }

        if ch == 'X' {
            return ch;
        }

        println!("\'{}\'", ch);

        if i == (s.len() - 1) {
            // loop needs a break
            break ch;
        }

        i += 1;
    };
}

// understanding ownership
fn ownership_1() {
    /*
       The following has error:

             let s1 = String::from("hello");
                 -- move occurs because `s1` has type `std::string::String`, which does not implement the `Copy` trait
             let s2 = s1;
                      -- value moved here
             println!("{}, world!", s2);
             println!("{}, world!", s1);
                                    ^^ value borrowed here after move
     */
    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{}, world! {}, world!", s1, s2);

    // If we clone the string, then it's a cloned resource in memory, and it's ok now
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{}, world! {}, world!", s1, s2);

    // If the string is not from String::from(), but a string literal, then it's ok.
    // Note that `String::from("hello")` has the type String, but "hello" has the type `&str`
    let s1 = "hello";
    let s2 = s1;
    println!("{}, world! {}, world!", s1, s2);

    // If we make s2 the reference of s1, then it's also ok.
    // see reference() function.
    let s1 = String::from("hello");
    let s2 = &s1;
    println!("{}, world! {}, world!", s1, s2);
}

// understanding ownership
fn ownership_2() {
    fn takes_ownership(a_string: String) {
        println!("In takes_ownership: {}", a_string);
    }

    let s = String::from("hello");

    takes_ownership(s);

    // after takes_ownership(s), s has been passed into another function, and therefore the value
    // has been moved. Same error here as in ownership_1()
    // println!("After takes_ownership: {}", s);

    fn makes_copy(primitive_value: i32) {
        println!("In makes_copy: {}", primitive_value);
    }

    let x = 5;

    makes_copy(x);

    // But for primitive values like integers, the value is copied not moved. So here x is still
    // available.
    println!("After makes_copy: {}", x);
}

// reference
fn reference() {
    let s1 = String::from("hello");
    let s2 = &s1;

    // s2 is a reference of s1, not moved from s1. Making a reference doesn't make s1 not available
    // because it doesn't change the ownership of the string.
    println!("s1 is {}, s2 is {}", s1, s2);
}

fn pass_in_value_vs_pass_in_reference() {
    // Accepts a reference of string, returns a int
    fn calculate_length_reference(s: &String) -> usize {
        s.len()
    }

    let s1 = String::from("hello");
    let r1 = &s1;
    let len1 = calculate_length_reference(r1);

    // s1 is available. r is a reference of s1, but not the owner. r borrows the ownership of s1.
    println!("Part 1: The length of '{}' is {}", s1, len1);

    // Accepts a string, returns a int
    fn calculate_length_value(s: String) -> usize {
        s.len()
    }

    let s2 = String::from("hello");
    let len2 = calculate_length_value(s2);

    // at this point, s2 is not available as the ownership has been passed into
    // calculate_length_value()
    println!("Part 2: The length is {}", len2);
}
