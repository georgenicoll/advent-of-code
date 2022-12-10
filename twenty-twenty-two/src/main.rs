#![feature(get_many_mut)]
mod sandbox;
mod utils;
mod one;
mod two;
mod three;
mod four;
mod five;
mod six;
mod seven;
mod eight;
mod nine;

pub fn old() {
    sandbox::entry_point();
    println!("1a Result is: {}", one::_1a().unwrap());
    println!("1b Result is: {}", one::_1b().unwrap());
    println!("2a Result is: {}", two::_2a().unwrap());
    println!("2b Result is: {}", two::_2b().unwrap());
    println!("3a Result is: {}", three::_3a().unwrap());
    println!("3b Result is: {}", three::_3b().unwrap());
    println!("4a Result is: {}", four::_4a().unwrap());
    println!("4b Result is: {}", four::_4b().unwrap());
    println!("5a Result is: {}", five::_5a().unwrap());
    println!("5b Result is: {}", five::_5b().unwrap());
    println!("6a Result is: {}", six::_6a().unwrap());
    println!("6b Result is: {}", six::_6b().unwrap());
    println!("7a Result is: {}", seven::_7a().unwrap());
    println!("7b Result is: {}", seven::_7b().unwrap());
    println!("8a Result is: {}", eight::_8a().unwrap());
    println!("8a Result is: {}", eight::_8b().unwrap());
}

fn main() {
    println!("9a Result is: {}", nine::_9a().unwrap());
    println!("9b Result is: {}", nine::_9b().unwrap());
}


