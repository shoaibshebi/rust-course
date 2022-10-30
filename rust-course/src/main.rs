#![deny(clippy::all)]
#![allow(unused_variables)]
#![allow(dead_code)]
//cargo-watch -qc -x run -x clippy

use std::{collections::HashMap, vec};

fn get_values() -> (String, String, i32) {
    ("Hellow".to_string(), "World".to_string(), 30)
}

fn get_user_name() -> Result<String, ()> {
    Ok("Shoaib ".to_string())
}

fn main() {
    let (hello, _, _) = get_values();

    let values: [i32; 2] = [2, 3];
    //dynamc length size vector
    let mut values2 = vec![2, 3];
    println!("values vector length {}", values.len());
    values2.push(5);

    let mut hashValues = HashMap::new();
    hashValues.insert("foo", "bar");

    if hashValues.contains_key("name") {
        println!("Key is present");
    }

    let entrySingle = hashValues.entry("foo");

    match entrySingle {
        std::collections::hash_map::Entry::Occupied(value) => {
            println!("key  {}, value {}", value.key(), value.get())
        }
        _ => println!("Not found"),
    }

    //iterators
    let values = vec![1, 2, 3, 4];
    // let foo = values.iter();
    // println!("values {:?}", foo);
    let updated_foo = values.iter().map(|x| x * 2);
    println!("{:?}", updated_foo);

    for value in values.iter() {
        println!("{}", value)
    }

    let name = None;
    let unwrapped = name.unwrap_or("Shaoib");
    println!("name is {} ", unwrapped);

    // Result<T, E> is the type used for returning and propagating errors. It is an enum with the variants, Ok(T),
    // representing success and containing a value, and Err(E), representing error and containing an error value.
    let valueErr: Result<&str, Box<dyn std::error::Error>> = Ok("Hello");
    match valueErr {
        Ok(value) => println!("value is {} ", value),
        Err(err) => println!("Err is {} ", err),
    }

    get_user_name().expect("hellow there");

    let is_ok = get_user_name().is_ok();
    let is_err = get_user_name().is_err();
}
