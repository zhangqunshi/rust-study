extern crate rand;

use std::io;
use rand:Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_rang(1, 101);
    
    println!("Please enter a number");
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Fail to read line");

    println!("you enter number is {}", guess); 
}