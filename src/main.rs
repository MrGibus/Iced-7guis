mod counter;
mod temperature;
mod flights;
mod timer;
mod crud;

// fn main() -> iced::Result {
//     // counter::main()
//     // temperature::main()
//     // flights::main()
//     crud::main()
// }

use std::io;

/// Entry point into program
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Enter example 1 - 7");
    println!("1: Counter");
    println!("2: Temperature Converter");
    println!("3: Flight Booker");
    println!("4: Timer");
    println!("5: CRUD (create, read, update, delete)");
    println!("6: Circle Drawing");
    println!("7: Cells");

    let mut input: String = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("{}", input);
        }
        Err(error) => println!("Error: {}", error)
    };

    let v = input.trim().parse::<u32>().expect("please enter a number from 1- 7");
    match v {
        1 => {
            println!("Counter Selected");
            counter::main()?
        },
        2 => {
            println!("Temperature Converter Selected");
            temperature::main()?
        },
        3 => {
            println!("Flight Booker Selected");
            flights::main()?
        },
        4 => {
            println!("Timer Selected");
            timer::main()?
        },
        5 => {
            println!("CRUD Selected");
            crud::main()?
        },
        6 => {
            println!("Circle drawer Selected");
            println!("NOT YET IMPLEMENTED");
        },
        7 => {
            println!("Cells Selected");
            println!("NOT YET IMPLEMENTED");
        },
        _ => ()
    }
    Ok(())
}
