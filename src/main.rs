extern crate ferris_says;
use ferris_says::say;
use std::io::{ stdout, BufWriter };
mod lib;
mod my_random_file;
fn main() {
    //[1]>> Basic Rust app setup with printing Text
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();
    
    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
    //<

    //[2]>> Basic Rust app to print using rustacean using different way of coding.
    //let out = b"Hello fellow Rustaceans!";
    //let width = 24;
    
    //let mut writer = BufWriter::new(stdout());
    //say(out, width, &mut writer).unwrap();
    //<<

    //[3]>> Calc lib usage
    // let mut calc = calculator_module::Calculator::new();
    // calc.add(2.0);
    // calc.subtract(1.0);
    // calc.multiply(3.0);
    // calc.divide(2.0);
    // println!("Result: {}", calc.result);
    //<<

    //[4]>>
    let lib_val  = lib::MY_LIB_CONSTANT + 15;
    let mut random_val :i32 = my_random_file::MY_RANDOM_CONSTANT + 15;

    println!("1. At top in main.rs I added a statement `mod lib`.\n\tFrom lib.rs file I print MY_LIB_CONSTANT + 15 =  {lib_val}");
    println!("2. At top in main.rs I added a statement `mod my_random_file`.\n\tFrom my_random_file.rs file I print MY_RANDOM_CONSTANT + 15 = {random_val}");
    //<<
}