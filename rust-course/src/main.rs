#![deny(clippy::all)]
#![allow(unused_variables)]
#![allow(dead_code)]
//cargo-watch -qc -x run -x clippy

//readonly reference
fn greet(name: &String) {
    println!("Hello, {}", name);
}
//mutable reference
fn greet2(fruit: &mut String) {
    println!("Hello, {}", fruit);
}
//this empty braces says this fn has no return type
fn say_hello_world() {
    let message = String::from("hellow");
    println!("{} ", message)
}

fn process_name(name: &str, callback: fn(&str) -> ()) {
    callback(name);
}

//STRUCTS

struct Person {
    name: String,
    age: u8,
}
fn person_print(name: String, age: u8) {
    let person = Person { name, age };
}

struct Point(f64, f64, f64);

enum AnimalType {
    Dog,
    Cat,
    Rabbit,
}

enum Shapes {
    Circle { radius: f64, center: (f64, f64) },
    Rectangle { width: f64, height: f64 },
}

fn main() {
    // let mut first_name = String::from("John");
    // // let second_name = first_name; //this is not good
    // let mut second_name = &first_name; // this is good
    //                                    //try to change data
    //                                    // second_name = String::from("John mean"); //will not do that
    // println!("hellow {}", first_name);

    // let personal_data = (32, "shoaib");
    // let (age, name) = personal_data;
    // println!(" {} {}", age, name);

    // let mut firstint = 32;
    // let mut secint = firstint;
    // secint += 1;
    // println!("{} {}", firstint, secint);

    let fruitname = String::from("Johb");
    //this is good , when you call
    greet(&fruitname);
    println!("fruitname {} ", fruitname);

    // // this is not good
    // greet(fruitname);
    // println!("fruitname {} ", fruitname); //you will not get value here, coz it's deallocated at the end of greet func

    let mut mango = String::from("Mango>");
    greet2(&mut mango);

    say_hello_world(); // this returns uint

    //inline functions
    let say_somthing = || String::from("hellow man");
    let say_somthing_out = say_somthing();
    println!("{} ", say_somthing_out);

    //first class functions

    //structs
    // let structOut = person_print("shoaib".to_string(), 24);

    //tupples
    let point: Point = Point(0.0, 0.1, 0.2);
    println!("{}", point.1);

    // match/switch
    let fluffy = AnimalType::Dog;
    match fluffy {
        AnimalType::Dog => println!("Woof"),
        AnimalType::Cat => println!("Meow"),
        AnimalType::Rabbit => println!("Hoot"),
        _ => println!("Soemthign default"),
    }

    let rectangle = Shapes::Rectangle {
        width: 3.2,
        height: 3.2,
    };
    if let Shapes::Rectangle { width, height } = rectangle {
        println!("width={}, height={} ", width, height);
    }
}
