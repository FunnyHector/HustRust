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

    println!(
        "The results are: {0}, {1}, {2}, {3}, {4}, {5}",
        sum, difference, product, quotient, remainder, true
    );
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

fn struct_test() {
    #[derive(Debug)] // this thing will give us the nice format when printing
    struct Person {
        name: String,
        age: u32,
        skills: Vec<String>,
    }

    // Object-oriented. This is a method on Person
    impl Person {
        // this is a instance method, the first arg is always &self
        fn with_skills(&self) -> String {
            self.name.clone() + &"(".to_string() + &self.skills.join(", ") + &")".to_string()
        }

        // this is a class method (static method in java)
        fn clone_hector(skills: Vec<String>) -> Person {
            Person {
                name: "Hector".to_string(),
                age: 33,
                skills,
            }
        }
    }

    struct Team {
        name: String,
        team_lead: Person,
        developers: Vec<Person>,
        testers: Vec<Person>,
    }

    fn person_with_skills(person: &Person) -> String {
        person.name.clone() + &"(".to_string() + &person.skills.join(", ") + &")".to_string()
    }

    let hector = Person {
        name: "Hector".to_string(),
        age: 33,
        skills: vec!["Ruby".to_string(), "Rails".to_string(), "Rust".to_string()],
    };

    let andrew = Person {
        name: "Andrew".to_string(),
        age: 35,
        skills: vec![
            "Ruby".to_string(),
            "Rails".to_string(),
            "Jenkins".to_string(),
        ],
    };

    let beck = Person {
        name: "Beck".to_string(),
        age: 35,
        skills: vec!["Management".to_string()],
    };

    let tian = Person {
        name: "Tian".to_string(),
        age: 35,
        skills: vec!["test".to_string()],
    };

    let pt_1 = Team {
        name: "Platform Team 1".to_string(),
        team_lead: beck,
        developers: vec![hector, andrew],
        testers: vec![tian],
    };

    let devs: Vec<String> = pt_1.developers.iter().map(person_with_skills).collect(); // in map we provide a fn name
    let testers: Vec<String> = pt_1
        .testers
        .iter()
        .map(|person| person.with_skills())
        .collect(); // in map we call the method on person

    println!(
        "Team name: {} | Team lead name: {} | Devs are: {:?} | Testers are: {:?}",
        pt_1.name,
        person_with_skills(&pt_1.team_lead),
        devs,
        testers
    );

    // #[derive(Debug)] will give us some good format
    println!("{:?}", pt_1.developers[0]);
    println!("{:#?}", pt_1.developers[0]);

    let hector_clone = Person::clone_hector(
        vec!["Ruby", "Rails", "Rust", "Jenkins"]
            .iter()
            .map(|skill| skill.to_string())
            .collect(),
    );
    println!("{:#?}", hector_clone);
}

fn tuple_struct_test() {
    struct Point(u32, u32);

    let origin = Point(0, 0);
    let point_1 = Point(12, 8);

    struct Foo(u32, String);
    let bar = Foo(5, "bar".to_string());

    println!(
        "{}, {}, {}, {}, {}, {}",
        origin.0, origin.1, point_1.0, point_1.1, bar.0, bar.1
    );
}

fn enum_test() {
    #[derive(Debug)]
    enum Fruit {
        Apple {
            weight_gram: f64,
        },
        Orange {
            number_of_slices: u8,
            country_of_origin: String,
        },
        Banana {
            colour: String,
        },
        Kiwifruit {
            is_golden: bool,
        },
        Cherry,
    }

    let apple = Fruit::Apple { weight_gram: 60.40 };
    let orange = Fruit::Orange {
        number_of_slices: 10,
        country_of_origin: String::from("US"),
    };
    let banana = Fruit::Banana {
        colour: String::from("green"),
    };
    let kiwifruit = Fruit::Kiwifruit { is_golden: true };
    let cherry = Fruit::Cherry;

    println!(
        "Apple: {:?} | Orange: {:?} | Banana: {:?} | Kiwifruit: {:?} | Cherry: {:?}",
        apple, orange, banana, kiwifruit, cherry
    );

    // pattern matching
    // the patterns have to be exhaustive, like in Haskell
    match orange {
        Fruit::Apple { weight_gram } => {
            println!(
                "10 of this apple would weight {} grams",
                weight_gram * 10 as f64
            ) // Using "as f64" to safely cast to f64
        }
        Fruit::Orange {
            number_of_slices,
            country_of_origin,
        } => println!(
            "This orange is from {} and has {} slices",
            country_of_origin, number_of_slices
        ),
        _ => {} // matches everything and does nothing.
    }
}

/*
   Rust does not *at all* allow null.
   Option is used to handle the possible null value.
   This is like Haskell, the Maybe monad: Maybe a = Just a | Nothing

        enum Option<T> {
            None,
            Some(T),
        }
*/
fn option_enum() {
    // fn(Option<u32>) -> String
    fn function_that_takes_optional_int(num: Option<u32>) -> String {
        match num {
            // Option is included by default, so Option::Some(..) can be simplified as Some(..), and
            // and so is None.
            // (need to figure out the term for import/include/require)
            Some(100) => "!!!100!!!".to_string(),
            Some(number) => number.to_string(),
            None => {
                // TODO: this is obviously not ideal. Revisit this when I know more of Rust.
                String::from("Some string for the error case")
            }
        }
    }

    let num = Some(100);
    // let num = Some(66);
    // let num = None;
    let result = function_that_takes_optional_int(num);

    println!("The result is: {:?}", result)
}

/*
  This looks like just a syntax sugar of pattern matching.
  There is a similar thing: while let pattern = expression { do_something() }

  The syntax is:

        if let pattern_1 = expression {
            do_something()
        } else if let pattern_2 = expression {
            do_something_else()
        } else {
            all_other_cases()
        }

    This reads as if `let` destructures `expression` into `pattern_1`, evaluate the block

    `if let` doesn't have to be exhaustive, unlike `match`.
*/
fn if_let() {
    // This is the equivalent function to function_that_takes_optional_int in option_enum
    // This function uses `if let` syntax
    fn function_that_takes_optional_int(num: Option<u32>) -> String {
        // because else if and else are both optional, so if let is often used to simplify the
        // pattern matching that has two cases.
        if let Some(100) = num {
            "!!!100!!!".to_string()
        } else if let Some(number) = num {
            // else if is optional
            number.to_string()
        } else {
            // else is optional
            String::from("Some string for the error case")
        }
    }

    // let num = Some(100);
    let num = Some(66);
    // let num = None;
    let result = function_that_takes_optional_int(num);

    println!("The result is: {:?}", result)
}

// mod declaration will automatically look for a `movies.rs` or `movies/mod.rs` in the same folder,
// and will insert its contents inside a module named `movies` under this scope.
// In this case, `movies.rs` is found and used
// `movies.rs` doesn't have to define `mod movies {...}`, see `movies.rs` for details.
mod movies;

// And in this case `games/mod.rs` is found and used.
mod games;

fn mod_demo() {
    movies::play("Interstellar".to_string());
    games::run_game_from_games_itself();
    games::hangman::run_hangman();
    games::guess_number::run_guess_numbers();
}

// the keyword `use` can simplify calling a method that is deep inside nested modules.
// It's like import method in java.
fn use_demo() {
    use games::guess_number::run_guess_numbers;
    run_guess_numbers();

    // `as` aliases the function name
    use games::guess_number::run_guess_numbers as foobar;
    foobar();

    // `use` can also be used with `pub` to make it publicly available to external.
    // pub use games::guess_number::run_guess_numbers;
}

fn unrecoverable_error_handling() {
    panic!("Something is wrong!");

    // This is unreachable, because panic! stops the execution.
    // println!("Something is not wrong");
}

fn recoverable_error_handling() {
    use std::fs::File;

    // `result` here will be a Result<File, Error> type.
    // This is a enum with generic types in definition.
    //
    //     enum Result<T, E> {
    //         Ok(T),
    //         Err(E),
    //     }
    let result = File::open("hello.txt");

    // Use `match`
    // match result {
    //     Ok(file) => println!("File opened: {:?}", file),
    //     Err(err) => println!("Failed to open the file: {}", err),
    // }

    // Use `if let`
    if let Ok(file) = result {
        println!("File opened: {:?}", file)
    }
}

// Sometimes we want to throw the recoverable errors
fn throw_recoverable_error() {
    use std::fs::File;

    // read the source to see how the error got thrown out
    let result_1 = File::open("not_exist.txt").expect("Failed to Open");
    let result_2 = File::open("not_exist.txt").unwrap();
}

// Question mark sign `?` looks like a syntax sugar. It retrieves the `Ok` value out.
// and if it's not an `Ok` value but an `Err` value, return the `Err` instantly.
//
// Because of how `?` works, after `?` is used, the value is guaranteed to be an `Ok`
fn question_mark_sign_example() {
    // say we have a function that takes a int, and returns a `Result`
    fn fn_1(number: i32) -> Result<i32, bool> {
        if number >= 0 {
            Ok(number)
        } else {
            Err(false)
        }
    }

    // and then we have another function, that uses the first function, and returns
    // a `Result`.
    fn fn_2(number: i32) -> Result<i32, bool> {
        // this is the way without using `?`

        // let result = fn_1(number);
        //
        // if let Ok(positive_number) = result {
        //     Ok(positive_number)
        // } else {
        //     Err(false)
        // }

        // this is the way using `?`

        // The possible `Err<T>` that fn_1 returns must match the return type of fn_2
        // i.e. fn_1 -> Result<T1, E1>, fn_2 -> Result<T2, E2>, here E1 and E2 must match
        let result = fn_1(number)?;

        Ok(result)
    }

    let result = fn_2(-50);

    println!("result is {:?}", result);
}

// kind() can be used to find out the error type
// This is also an example of how to achieve `try {...} catch(Error) {...}` in Rust
// Looks like this has to be done in separate functions.
fn error_type_example() {
    use std::fs::File;
    use std::io;
    use std::io::Read;

    // This function read the content from a file into a String
    // There are two places that can return error.
    fn read_text_from_file(path: &str) -> Result<String, io::Error> {
        let mut file = File::open(path)?;
        let mut content = String::new();

        file.read_to_string(&mut content)?;

        Ok(content)
    }

    let result = read_text_from_file("not_exist.txt");

    match result {
        Ok(content) => println!("Content is: {}", content),
        Err(error) => match error.kind() {
            // This is a little like we catch a specific error type and do something
            io::ErrorKind::NotFound => println!("No such file!"),
            _ => println!("Unknown error!"),
        },
    }
}

fn generic_example() {
    // a simple non-generic function to find the max of an array
    fn max(array: &[i32]) -> i32 {
        let mut index_of_max: usize = 0;
        for index in 1..array.len() {
            if array[index_of_max] < array[index] {
                index_of_max = index;
            }
        }

        array[index_of_max]
    }

    // rewrite this function in a generic way
    // <T> is the type param.
    // T also has restrictions: it has to implement PartialOrd and Clone traits,
    // because the `<` operator and `clone` function are used on T.
    fn max_generic<T: std::cmp::PartialOrd + std::clone::Clone>(array: &[T]) -> T {
        let mut index_of_max: usize = 0;
        for index in 1..array.len() {
            if array[index_of_max] < array[index] {
                index_of_max = index;
            }
        }

        array[index_of_max].clone()
    }

    let array = [1, 2, 3, 4, 5, 6, 7, 8, 1, 1, 0, 0, -3];

    println!(
        "max & max_generic: {} & {}",
        max(&array),
        max_generic(&array),
    );
}

fn generic_struct_example() {
    struct Point<T> {
        x: T,
        y: T,
    }

    let point_1 = Point { x: 1, y: 2 };
    let point_2 = Point { x: 1.0, y: 2.5 };

    // unmatched types are not allowed:
    // let invalid_point = Point { x: 1, y: 2.5 };

    // and when we implement a function for the struct, we need to add the generic type
    // after the keyword `impl` too
    impl<T> Point<T> {
        fn get_x(&self) -> &T {
            &self.x
        }
    }

    // we can implement same function differently based on different types
    impl Point<i32> {
        fn get_y(&self) -> &i32 {
            &self.y
        }
    }

    impl Point<String> {
        fn get_y(&self) -> String {
            "Something Different".to_string()
        }
    }
}

fn trait_example() {
    struct Person {
        name: String,
        age: u32,
        skills: Vec<String>,
    }

    let hector = Person {
        name: "Hector".to_string(),
        age: 33,
        skills: vec![],
    };
    let roxy = Person {
        name: "Roxy".to_string(),
        age: 32,
        skills: vec![],
    };

    // This is a generic trait
    trait Love<T> {
        // this is a function that has implementation.
        // in Java this is called default method in interface
        fn love(&self, other: &T) -> String {
            "Love".to_string()
        }
    }

    // Implement the trait for Person. Syntax:
    //
    //     impl <trait_name> for <struct_or_enum_name>

    // Since it's a generic trait, we need to specify the type for it.
    //
    // If we don't override the default implementation, then it behaves as the default
    // implementation:
    // impl Love<Person> for Person {}

    // If we override the default implementation, then it behaves as the new way.
    impl Love<Person> for Person {
        fn love(&self, other: &Person) -> String {
            self.name.clone() + " loves " + &other.name
        }
    }

    println!("{}", hector.love(&roxy));

    // another trait
    trait Hate<T> {
        // this function is empty.
        // any struct that implement this trait needs to implement this function
        // NOTE: do not need an empty {} after String.
        // i.e. this is wrong: fn hate(&self, other: T) -> String {};
        fn hate(&self, other: &T) -> String;
    }

    // This doesn't compile as Rust knows that there is an empty function that
    // needs a concrete implementation.
    // impl Hate<Person> for Person {}

    impl Hate<Person> for Person {
        fn hate(&self, other: &Person) -> String {
            self.name.clone() + " hates " + &other.name + " (not really :))"
        }
    }

    println!("{}", hector.hate(&roxy));
}

fn trait_on_generic_types() {
    trait Love {
        fn love(&self) -> String;
    }

    // This is a little like Ruby -- we can reopen a bundled core class!
    // But really this isn't anything black-magical like Ruby.
    // We just implement a trait for a struct (yes String is a struct)
    impl Love for String {
        fn love(&self) -> String {
            "Love ".to_string() + self
        }
    }

    // In Java, we can declare the argument type with an interface
    // In Rust this is done with a slightly different syntax -- add `impl` before the trait
    // This method signature means: the function accepts an `object` argument which
    // must implement the trait `Love`
    fn give_love_1(object: impl Love) {
        // inside the function, `object` can only invoke functions defined in `Love`
        // Explained in Ruby:
        // object.methods == Love.methods
        println!("{}", object.love());
    }

    give_love_1("Rust".to_string());

    // Another way using generic:
    fn give_love_2<T: Love>(object: T) {
        println!("{}", object.love());
    }

    give_love_2("Rust".to_string());

    // The generic way is more handy when there are multiple args
    // Otherwise the method def line would be
    //     fn give_love_3(object_1: impl Love, object_2: impl Love, object_3: impl Love)
    fn give_love_3<T: Love>(object_1: T, object_2: T, object_3: T) {
        println!(
            "{}, {}, {}",
            object_1.love(),
            object_2.love(),
            object_3.love()
        );
    }

    give_love_3(
        "Rust".to_string(),
        "Music".to_string(),
        "Fancy Car".to_string(),
    );

    trait Hate {
        fn hate(&self) -> String;
    }

    impl Hate for String {
        fn hate(&self) -> String {
            "Hate ".to_string() + self
        }
    };

    // When we want a type that implements multiple traits,
    // we can use `+` to connect multiple traits
    // Alternatively, this could be:
    //     give_love_4(object: impl Love + Hate)
    fn give_love_4<T: Love + Hate>(object: T) {
        println!("{}, or {}", object.love(), object.hate());
    }

    give_love_4("Rust".to_string());

    // A syntax sugar to make the long line shorter and read nicer
    // use the keyword `where`
    fn give_love_5<T, U>(object_1: T, object_2: U)
    where
        T: Love,
        U: Love + Hate,
    {
        println!("{}, and {}", object_1.love(), object_2.hate());
    }

    give_love_5("Rust".to_string(), "Damp weather".to_string());

    // The keyword `Self` is a special type, it represents the current type
    // or, in other words, the type of `self` (note the case difference)
    trait Respect {
        fn respect(&self, other: &Self) -> String;
    }

    impl Respect for String {
        fn respect(&self, other: &Self) -> String {
            self.clone() + " respects " + &other[..]
        }
    };

    fn give_respect<T: Respect>(from: T, to: T) {
        println!("{}", from.respect(&to));
    }

    give_respect("Hector".to_string(), "Andrew".to_string());
}

// We want to define a function that returns a type that implements a trait
// syntax:
//     fn function_name(arg: type) -> impl TraitName { ... }
fn trait_as_return_type() {
    trait Love {
        fn love(&self) -> String {
            "Love".to_string()
        }
    }

    impl Love for String {};

    // This function returns something that implements `Love`.
    // The implementation returns a string, which implements `Love`,
    // so it's valid.
    fn function_that_returns_trait_1() -> impl Love {
        "Any String".to_string()
    }

    impl Love for Option<String> {};

    // Also valid since `Option<String>` has implemented `Love`
    fn function_that_returns_trait_2() -> impl Love {
        Some("Any String".to_string())
    }

    // The print function won't work because it expects string as arguments.
    // But the compiler can only be sure that these two functions returns
    // a "impl Love" type, but that is not necessarily String.
    // println!(
    //     "{:?} and {:?}",
    //     function_that_returns_trait_1(),
    //     function_that_returns_trait_2()
    // );
}

fn lifetime_example() {
    // This function can't pass compilation, because the return value is one of the arguments,
    // whose lifetime could have been ended. See the code below this function for example.
    // fn longer_string(s1: &str, s2: &str) -> &str {
    //     if s2.len() > s1.len() {
    //         s2
    //     } else {
    //         s1
    //     }
    // }
    //
    // let result;
    // {
    //     let s1 = "short";
    //     let s2 = "long";
    //     result = longer_string(s1, s2);
    // } // outside this code block, s1 and s2 has ended their lifetime
    // so we are using `result` illegally.
    // println!("{} is longer", result);

    // With lifetime parameter, this function compiles and works as expected.
    fn longer_string<'a>(s1: &'a str, s2: &'a str) -> &'a str {
        if s2.len() > s1.len() {
            s2
        } else {
            s1
        }
    }

    println!("The longer string: {}", longer_string("Short", "Long"));

    // The following example comes from the rust bible. Includes generics, traits, and lifetime.

    use std::fmt::Display;

    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, announcement: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", announcement);

        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    println!(
        "The longer string: {}",
        longest_with_an_announcement("Short", "Long", "Announcement")
    );
}

// a crate must have a main() function. This is like the main function in Java.
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

    // all_of_the_strings();

    // struct_test();

    // tuple_struct_test();

    // enum_test();

    // option_enum();

    // if_let();

    // mod_demo();

    // use_demo();

    // unrecoverable_error_handling();

    // recoverable_error_handling();

    // throw_recoverable_error();

    // question_mark_sign_example();

    // error_type_example();

    // generic_example();

    // generic_struct_example();

    // trait_example();

    // trait_on_generic_types();

    // trait_as_return_type();

    lifetime_example();
}
