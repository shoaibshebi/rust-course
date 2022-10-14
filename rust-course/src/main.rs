#![deny(clippy::all)]
//cargo-watch -qc -x run -x clippy

fn main() {
    let first_name = "John";
    let first_name = first_name;
    println!("hellow {}", first_name);

    let personal_data = (32, "shoaib");
    let (age, name) = personal_data;
    println!(" {} {}", age, name);

    let firstint = 32;
    let secint = firstint;
    println!("{} {}", firstint, secint);
}
