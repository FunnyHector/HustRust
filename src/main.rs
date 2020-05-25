fn main() {
    // variables();
    //
    // function_1("Hector");
    // println!("{:?}", function_2());
    //
    // for_loops();

    let foo = loop_loop();
    println!("loop_loop() returned: {}", foo);
}

// variables
fn variables() {
    // default variables
    const A: i32 = 3;
    let b = 100.8;
    let c = 300;
    const FOO:&str = "A const";

    println!("Hello, world! {0}, {1}, {2}, {3}", A, b, c, FOO);

    // mutable variables
    let mut x:i32 = 5;
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
    }
}
