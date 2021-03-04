use std::env;
use serde::{Deserialize};
use std::string::String;


#[derive(Debug, Deserialize)]
struct InputType {
    a: i64,
    b: i64,
    c: i64,
}

fn main() {
    println!("Hello, world!");
    
    let args: Vec<String> = env::args().collect();

    println!("args are: {:?}", args);

    let data: InputType = serde_json::from_str(&args[1]).unwrap();
    println!("data is: {:?}", data);
    
    let total = data.a + data.b + data.c;
    println!("total is: {:?}", total);
}
