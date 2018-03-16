extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please enter a number");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Fail to read line");

        println!("you enter number is {}", guess); 

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            } 
        }
    }
    
}
