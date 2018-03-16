

use std::io;


fn main() {

    println!("Please enter a number");
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Fail to read line");

    println!("you enter number is {}", guess); 
}