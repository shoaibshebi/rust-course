#![deny(clippy::all)]
//cargo-watch -qc(quit and clear the terminal) -x(execute command) run -x(execute) clippy
//cargo-watch -qc -x run -x clippy

use intutils::addition::add;
use intutils::subtraction::sub;

use rand::{Rng, Error};
use core::panic;
use std::{cmp::Ordering, io::{self, ErrorKind, Read}, collections::HashMap, mem::size_of_val, fs::File, path::StripPrefixError};

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
    // lec_four();

    /*       LEC 5 (CHAPTER 7 OF BOOK)                */
    // lec_five();

    /*       LEC 6 (CHAPTER 8 OF BOOK)                */
    // lec_six();

    /*       LEC 7 (CHAPTER 9 OF BOOK)                */
    // lec_seven();

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

pub fn lec_five(){
    let add_res = add(1,2);
    println!("{add_res}");
}

pub fn lec_six(){
    fn get_values() -> (String, String, i32) {
        ("Shoaib".to_string(), "Ali".to_string(), 24)
    }

    let (name,_,_) = get_values();
    println!("{:?}",name);

    let mut values: [&str; 2]= ["foo", "bar"];
    println!("{}", &values[0]);


    let mut values1 = vec![1,2,3];
    let mut values2 = vec![1,2,3];
    values1.append(&mut values2); // moved

    if values1.contains(&3){
        println!("yes");
    }else{
        println!("no");
    }

    println!("{:?} {:?}", values1,values2);

    let mut values = HashMap::new();
    values.insert("foo", "bar");  

    let mapval = values.get("foo");
    println!("{:?}", mapval);

    // if mapval == Option("bar"){
    //     println!("yes")
    // }

    match  mapval {
        Some(val)=> println!("{val}"),
        None=> println!("none")
    }

    let entry = values.entry("foo");
    // match entry {
    //     std::collections::hash_map::Entry(value) => {
    //         println!("{}", value.get());
    //     },
    //     _=> println!("not found")
    // }

    let values = vec![1,2,3,4];
    let iter = values.iter();
    // let sum1: i32 = iter.sum();
    // let sum2: i32 = iter.sum(); // iter have consumed all the values via moving so cant use again 
    let doubledVals = iter.map(|x|  x*2);
    // let doubledVals = iter.map(|x|  x*2);

    for name in values.into_iter() { //moved values to name
        println!("{}", name);
    }
    // println!("{}", values[0]);

    let mut values = vec![1,2,3,4];
    let firstval = &values[2];
    println!("{firstval}");
    values.push(5);

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(3.2),
        SpreadsheetCell::Text(String::from("shoaib"))
    ];

    println!("---{}", size_of_val(&*row));


    let mut s0  =  "foo";
    let mut s1 = String::from("Habibi");
    let s2 = "bar";
    // s1.push_str(s2); // this method take ownership of s2, so s2 will not be available ahead to use
    s1.push('b'); // this method takes single char to push at end
    println!("{s2} {s1}");

    let s3 = String::from(" Wallah");
    let s4 = s1 + s2;

    println!("{s4}");

    fn my_name(a:&String, b:&String){
        println!("{a} {b}");
    }
    let first_name = String::from("muh");
    let last_name = String::from("shoaib");
    my_name(&first_name, &last_name);

    println!("{first_name}");

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace()  {
        let count = map.entry(word).or_insert(0);
        println!("ref {}", count);
        *count +=1;
    }
    println!("{:?}", map);


}

pub fn lec_seven(){
    let value:Result<&str, ()> = Ok("some");
    match value {
        Ok(val)=>println!("{}",val),
        Err(_)=>println!("")
    }

    fn my_name() -> Result<String, ()> {
        Ok("shoaib".to_string())
    }

    let name = my_name().expect("expecting anme");
    println!("{name}");

    let greetings_file = File::open("hello.txt");
    // let greetings_file = match greetings_file {
    //     Ok(file) => file,
    //     Err(err)=> match err.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt"){
    //             Ok(fc)=>fc,
    //             Err(e)=> panic!("Problem creating the file {:?}",{e})
    //         },
    //         // File::create("hello.txt").unwrap_or_else(|err| {
    //         //     panic!("error creating file {:?}",err)
    //         // })
    //         other_error =>{
    //             panic!("problem opening the file {:?}",other_error)
    //         }
    //     }
    // };

    let greetings_file = File::open("hello.txt");

 let mut username_file = match greetings_file {
        Ok(file) => file,
        Err(_) =>  panic!("not found"),
    };
    
    println!("file objecct {:?}",username_file);

    let mut username = String::new();
    // match username_file.read_to_string(&mut username) {
    //     Ok(username) => username,
    //     Err(err)=> panic!("not read")
    // }


        let file_content = read_username_from_file();
        print!("file_content {:?}", file_content);

        let last_cchar = last_char_of_first_line("my name shoaib");
        println!("last char {:?}",last_cchar);
        
        options_call();

}

fn read_username_from_file() -> Result<String, io::Error>{
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
fn last_char_of_first_line(text:&str)-> Option<char>{
    text.lines().next()?.chars().last()
}
fn options_call(){
    let val = Some(10);
    let name:Option<&str> = None;

    println!("Name is {}", val.unwrap());

    let mut age = Some(20);
    match age.as_mut() {
        Some(age)=> *age +=10,
        None=>{}
    }

    let newname = Some("john doe");
    let unwraped = name.unwrap_or_else(|| {
        "jane doe"
    });

    println!("newname {:?}",newname);
}

pub fn rust_others() {
    let s = String::from("shoaib");
    println!("{s}")
}
