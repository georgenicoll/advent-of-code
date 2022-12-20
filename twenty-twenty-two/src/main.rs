#![feature(get_many_mut)]
#![feature(once_cell)]
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
mod ten;
mod eleven;
mod twelve;
mod thirteen;
mod fourteen;
mod fifteen;
mod sixteen;
mod seventeen;
mod eighteen;
mod nineteen;
mod twenty;

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
    println!("9a Result is: {}", nine::_9a().unwrap());
    println!("9b Result is: {}", nine::_9b().unwrap());
    println!("10a Result is: {}", ten::_10a_and_10b().unwrap());
    println!("11a Result is: {}", eleven::_11a().unwrap());
    println!("11b Result is: {}", eleven::_11b().unwrap());
    println!("12a Result is: {}", twelve::_12a().unwrap());
    println!("12b Result is: {}", twelve::_12b().unwrap());
    println!("13a Result is: {}", thirteen::_13a().unwrap());
    println!("13b Result is: {}", thirteen::_13b().unwrap());
    println!("14a Result is: {}", fourteen::_14a().unwrap());
    println!("14b Result is: {}", fourteen::_14b().unwrap());
    println!("15a Result is: {}", fifteen::_15a().unwrap());
    println!("15b Result is: {}", fifteen::_15b().unwrap());
    println!("16a Result is: {}", sixteen::_16a().unwrap());
    println!("16b Result is: {}", sixteen::_16b().unwrap());
    println!("17a Result is: {}", seventeen::_17a().unwrap());
    println!("17b Result is: {}", seventeen::_17b().unwrap());
    println!("18a Result is: {}", eighteen::_18a().unwrap());
    println!("18b Result is: {}", eighteen::_18b().unwrap());
    println!("19a Result is: {}", nineteen::_19a().unwrap());
    println!("19b Result is: {}", nineteen::_19b().unwrap());
}

fn main() {
    println!("20a Result is: {}", twenty::_20a().unwrap());
    println!("20b Result is: {}", twenty::_20b().unwrap());
}
