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
    let condition: bool = check_number(number);
    divisibility_test(number);

    let_if_statement(condition);
}

// handling if condition
fn check_number(number: i32) -> bool {
    if number < 5 {
        println!("Condition is true");
        true
    } else {
        println!("Condition is false");
        false
    }
}

// handling multiple if condition with else if
//? Note using too many else if can be a sign of a bad code, use match instead
fn divisibility_test(number: i32) {
    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3 or 2");
    }
}

//? Using if in a let statement
fn let_if_statement(condition: bool) {
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
}
