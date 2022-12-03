mod utils;
mod one;
mod two;
mod three;

fn main() {
    println!("1a Result is: {}", one::_1a().unwrap());
    println!("1b Result is: {}", one::_1b().unwrap());
    println!("2a Result is: {}", two::_2a().unwrap());
    println!("2b Result is: {}", two::_2b().unwrap());
    println!("3a Result is: {}", three::_3a().unwrap());
    println!("3b Result is: {}", three::_3b().unwrap());
}
