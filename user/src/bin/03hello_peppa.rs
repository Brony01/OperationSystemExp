#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("Hello peppa! Have a nice day!");
    println!("Hello peppa! Good morning!");
    println!("Hello peppa! Good afternoon!");
    println!("Hello peppa! Good night!");
    println!("Hello peppa! Good to see you again!");
    println!("Hello peppa! I'll always be with you!");
    println!("Hello peppa! Hope we could share the rest of our life with each other!");
    0
}
