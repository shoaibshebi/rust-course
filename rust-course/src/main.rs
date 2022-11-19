//cargo-watch -qc(quit and clear the terminal) -x(execute command) run -x(execute) clippy
//cargo-watch -qc -x run -x clippy

use rand::Rng;
use std::{cmp::Ordering, io};

const MY_NAME: &str = "shoaib";

fn main() {
    /*       LEC 1 (CHAPTER 1,2,3 OF BOOK)                */
    // lec_one();
    // scope();
    // shadowing();
    // conditions()

    /*       LEC 2 (CHAPTER 4 OF BOOK)                */
    // lec_two();

    /*       LEC 3 (CHAPTER 5 OF BOOK)                */
    // lec_three();

    /*       LEC 4 (CHAPTER 6 OF BOOK)                */
    lec_four();

    /*                  RUST OTHERS                    */
    // rust_others();
}

/*       LEC 1 (CHAPTER 1,2,3 OF BOOK)                */
pub fn lec_one() {
    let name = "Rust Habibi";
    // name = "Rust Habibi"; //cant mut
    println!("{}", name);

    let counting = 200u8; //defining data type at end

    let Color = 0xFF0000;

    let body_wight_kg = 5.5;
    {
        println!("{} {}", body_wight_kg, Color); //5.5
        let body_wight_kg = "shoaib";
        println!("{}", body_wight_kg); //shoaib
    }

    println!("{} ", body_wight_kg); //5.5

    // let boyage = 20;
    // const MY_AGE = boyage; //can't do that
    const MY_AGE: i32 = 100_000; //better readability

    let myinfo = (25, "Rust Habbi");
    println!("{} ", myinfo.0);
    let (age, name) = myinfo;
    // conditions_and_loops()
    guessing_number_game();
}
pub fn scope() {
    let body_wight_kg = "5.5";
    println!("{} ", body_wight_kg);
}
pub fn data_types() {
    let a = 98_222; // decimal
    let b = 0xff; //hex
    let c = 0o77; //octal
    let d = 0b1111_0000; //binary
    let e = b'A'; //byte

    let f = 255;

    // flot f64, f32

    // compound types -> tuples, and other
    // arrays/lists of fixed length
    //vectors dynamic size
}
pub fn conditions_and_loops() {
    let condition = true;
    if condition == true {
        println!("true")
    } else {
        println!("false");
    }

    let mut counter = 0;
    let res = loop {
        counter += 1;
        if counter == 9 {
            break counter;
        }
    };
    println!("{}", res);

    // while loop, for loop, loop

    for number in 1..4 {
        println!("{}", number);
    }
}
pub fn guessing_number_game() {
    println!("Guess the no");

    let secret_no = rand::thread_rng().gen_range(1..=100);
    println!("The secrent number is : {secret_no}");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Enter number");
    println!("{guess}");

    match guess.cmp(&secret_no) {
        Ordering::Equal => println!("You win"),
        Ordering::Less => println!("Too less"),
        Ordering::Greater => println!("Too large"),
    }
}
pub fn shadowing() {
    let body_wight_kg = 5;
    println!("{}", body_wight_kg); // 5

    let body_wight_kg = body_wight_kg + 1;
    println!("{}", body_wight_kg); // 6

    {
        let body_wight_kg = "shoaib";
        println!("{}", body_wight_kg); //shoaib
    }

    println!("{} ", body_wight_kg); // 6

    let spaces = "   ";

    println!("{spaces}");

    let n8: u8 = 255;

    let nf1: f32 = 2147483648.0;
    let nf2: f32 = -4294967297.0;

    let out = 2 / 3; //gives zero - automatically floored

    println!("{out}");

    let t1 = (2, "age", 2);
    println!("{}", t1.2);
}

pub fn conditions() {
    let cond = false;
    let num = if cond { 5 } else { 6 }; // won't work, coz rust demands to have both sections type same
    println!("{num}");

    // have implicit return type on loop called unit () but you are returning  int
    // loop {
    //     println!("again");
    //     break 5;
    // }

    println!("here");

    'countingup1: loop {
        println!("outer loop");
        // loop {
        //     println!("inner loop");
        //     break 'countingup1;
        // }
    }

    // let num2 = while true {
    //     5;
    //     break;
    // };

    for x in (1..5).rev() {
        println!("{x}");
    }
}

/*       LEC 2 (CHAPTER 4 OF BOOK)                */
pub fn lec_two() {
    let x = 2;
    let y = x;
    println!("{x} {y} {MY_NAME}");

    let mut s = String::from("hello world");

    let r1 = &s; // no problem
    let r2 = &s; // no problem

    println!("{r1}");

    let r3 = &mut s; // BIG PROBLEM

    println!("{r3}");

    let fword = first_word(&s);
    println!("{fword}")
}
pub fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes(); // bytes contain ascii codes of each latter in s

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            println!("{i} {item} {}", b' ');
            return &s[i..s.len()]; // not dunamic string, but the string literal
        }
    }

    &s[..] // not dunamic string, but the string literal
}

/*       LEC 3 (CHAPTER 5 OF BOOK)                */
pub fn lec_three() {
    // tuple structure

    struct Person {
        name: String,
        age: u8,
    }
    #[derive(Debug)] // debug trait to directly debug(print) the struct/tupple/object value
    struct Point(f64, f64, f64);

    /*

        @describe - detials of the point
        @zero - returns an empty point

    */
    impl Point {
        fn describe(&self) {
            println!("Point x={} y={} z={} ", self.0, self.1, self.2);
        }
        fn zero() -> Point {
            Point(0.0, 0.0, 0.0)
        }
    }

    /*

        @twice - double and return new point
        @make_twice - twice the caller object values

    */
    impl Point {
        fn twice(&self) -> Point {
            Point(self.0 * 2.0, self.1 * 2.0, self.2 * 2.0)
        }
        fn make_twice(&mut self) {
            self.0 *= 2.0;
            self.1 *= 2.0;
            self.2 *= 2.0;
        }
    }

    let mut p = Point(0.0, 1.0, 2.0);
    p.describe();
    p.make_twice();
    println!("{:#?}", p); // debug trait to directly debug(print) the struct/tupple/object value
    dbg!(p); // debug trait to directly debug(print) the struct/tupple/object value

    let person = Person {
        name: "shoaib".to_string(),
        age: 24,
    };

    let person2 = Person {
        // name: person.name, // String dont implement copy trait, so value moved
        name: "ali".to_string(),
        age: person.age, // scaler type implements copy trait
    };

    println!("{}", person.name)
}

/*       LEC 4 (CHAPTER 6 OF BOOK)                */
pub fn lec_four() {
    #[derive(PartialEq)]
    enum Shapes {
        Circle { radius: f64, center: (f64, f64) },
        Rectangle { height: f64, width: f64 },
    }

    let rect = Shapes::Rectangle {
        height: 3.0,
        width: 3.0,
    };
    let rect2 = Shapes::Rectangle {
        height: 3.0,
        width: 3.0,
    };

    // want to check creating variable ahead of "if" then only "=" sign otherwise if two variables then "==" sign
    // if let Shapes::Rectangle { height, width } = rect2 {
    if let Shapes::Rectangle {
        height: 3.0,
        width: 3.0,
    } = rect2
    {
        println!("same rects ")
    };

    match rect {
        Shapes::Rectangle { height, width } => {
            println!(" width  = {} , height = {}", width, height);
        }
        _ => println!("default"),
    }

    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState), //tuple enum
    }

    let coin = Coin::Penny;

    let res = match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}! ", state);
            25
        }
    };
    println!("coin = {res}");

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(item) => Some(item + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // println!("{five} {six}")

    let num = 7;
    match num {
        3 => 3 + 1,
        5 => 5 + 1,
        other => other * 0,
    };
}

pub fn rust_others() {
    let s = String::from("shoaib");
    println!("{s}")
}
