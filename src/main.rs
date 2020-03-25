//use std::convert::From;
//use std::env;
mod binary;

//use binary::Number;
use binary::AsBinary;
//use binary::to_binary;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let value: i32 = args[1].parse::<i32>().unwrap();
    //let bin: String = to_binary(value);
    //let num: Number = Number{value: value};
    //println!("{}", num.as_binary());
    println!("{}", value.as_binary());
}
