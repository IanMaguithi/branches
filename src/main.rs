// Simple if else condition
use std::io;

fn main() {
    println!("Enter a number: ");
    let mut number = String::new();

    // Read input from user
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    // Convert string to number using shadowing and trim() to remove whitespace
    let number: i32 = number.trim().parse().expect("Please type a number!");
    check_number(number);
}

// handling if condition
fn check_number(number: i32) {
    if number < 5 {
        println!("Condition is true");
    } else {
        println!("Condition is false");
    }
}
