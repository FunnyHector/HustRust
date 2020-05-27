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

// `&` is the reference sign, it allows us to use (read-only) but not own the value. In other words,
// the ownership is not transferred.
fn reference() {
    let s1 = String::from("hello");
    let s2 = &s1;

    // s2 is a reference of s1, not moved from s1. Making a reference doesn't make s1 not available
    // because it doesn't change the ownership of the string.
    println!("s1 is {}, s2 is {}", s1, s2);
}

// `*` is the dereference sign. Opposite of reference.
fn dereference() {
    let s1 = String::from("hello");
    let s2 = &s1;

    // s2 is a reference of s1, not moved from s1. Making a reference doesn't make s1 not available
    // because it doesn't change the ownership of the string.
    println!("True? {}", (*s2 == s1) == true);
}

fn borrow_ownership() {
    let s1 = String::from("hello");
    let s2 = &s1;

    println!("s1 is {}, s2 is {}", s1, s2);

    let s3 = s1;

    // this will explode, as in previous step, we have transferred the ownership of "hello" from s1
    // to s3. s1 is not no one's owner, and s2 is trying to borrow s1's ownership, hence not valid.
    // println!("s3 is {}, s2 is {}", s3, s2);

    // if we re-borrow the ownership from a valid owner, then it's a valid move.
    let s2 = &s3;
    println!("s3 is {}, s2 is {}", s3, s2);
}

// value vs reference as parameters
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

// when you have a borrowed ownership (reference), you can use (read) the value, but you can't
// mutate it.
// But things get more powerful with mutable reference.
fn mutable_reference() {
    let mut s1 = String::from("hello");
    let s2 = &s1;

    // it's ok to read the value of s1.
    // read s2 == read the referee of s2 == read the value of s1
    println!("{}", s2);

    // this will explode. It's not ok to mutate the value
    // s2.push_str(" -- Ops!");

    // However, we can set s2 as a mutable reference.

    let mut s1 = String::from("hello");
    // notice the difference here. Also note that s1 must be a mutable type if we define s2 as a
    // mutable reference.
    let s2 = &mut s1;

    // it's ok to read the value of s1.
    // read s2 == read the referee of s2 == read the value of s1
    println!("{}", s2);

    // it's ok to mutate the value
    s2.push_str(" -- OK!");

    println!("{}", s2);
}

fn only_one_active_mutable_reference() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    // Boom.
    // Rust doesn't allow multiple mutable references to the same value in the same scope. This is
    // for concurrency safety.
    // println!("{}, {}", r1, r2);

    // how about one mutable one immutable?
    let r1 = &s;
    let r2 = &mut s;

    // or the other way?
    // let r1 = &mut s;
    // let r2 = &s;

    // Boom again.
    // Rust doesn't allow mutable reference as long as there is another immutable reference in the
    // same scope.
    // "cannot borrow `s` as mutable because it is also borrowed as immutable"
    // println!("{}, {}", r1, r2);
}

// Won't compile.
// This function will return a reference to s, i.e. the value "hello". However, when the
// function finishes, s is freed, there is no value for &s to reference.
//   > help: this function's return type contains a borrowed value, but there is no value for it to
//     be borrowed from
//   > help: consider using the `'static` lifetime
//
// fn no_dangling_reference() -> &String {
//     let s = String::from("hello");
//
//     return &s;
// }

// Not so sure why yet. Should come back and ponder again.
fn a_strange_case() {
    // won't compile:
    //   > Use of moved value for v1[0] and v2[0]
    // fn foo_1(v1: Vec<i32>, v2: Vec<i32>) -> (Vec<i32>, Vec<i32>, i32) {
    //     (v1, v2, v1[0] + v2[0])
    // }

    // this is ok
    fn foo_2(v1: Vec<i32>, v2: Vec<i32>) -> (i32, Vec<i32>, Vec<i32>) {
        (v1[0] + v2[0], v1, v2)
    }

    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];

    let (v1, v2, answer) = foo_2(v1, v2);

    println!("{:?}, {:?}, {:?}", v1, v2, answer);
}

fn string_slicing() {
    let mut s = String::from("provocative");

    let slice_1 = &s[0..5];
    let slice_2 = &s[5..];

    // Boom. Once the string is sliced, it's borrowed as immutable. Even it's a mutable variable,
    // We can't borrow it as a mutable now.
    // Error:
    //   > cannot borrow `s` as mutable because it is also borrowed as immutable
    //s.push_str("wrong!");

    println!("slice 1 is {}, and slice 2 is {}", slice_1, slice_2);
}

fn all_of_the_strings() {
    let s1 = String::from("hello"); // :String
    let s2 = "hello".to_string(); // :String. Should eq s1
    let s3 = &s1; // :&String
    // let s4 = *s1; // :str. This cannot compile: "doesn't have a size known at compile-time"
    let s5 = &*s1; // :&str
    let s6 = &s1[..]; // :&str. Should eq s5.

    println!("from s1 to s6: {}, {}, {}, {}, {}", s1, s2, s3, s5, s6);
}

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

    // dereference();

    // pass_in_value_vs_pass_in_reference();

    // borrow_ownership();

    // mutable_reference();

    // only_one_active_mutable_reference();

    // no_dangling_reference();

    // a_strange_case();

    // string_slicing();

    all_of_the_strings();
}
