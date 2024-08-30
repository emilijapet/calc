include!("bindings.rs");
use std::io;

fn read_integer () -> i32 {
    let mut user_input = String::new();

    loop {
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read user input");

        let Ok(parsed_input) = user_input.trim().parse::<i32>() else {
            println!("Failed to parse integer, enter a new number");
            user_input.clear();
            continue;
        };

        return parsed_input;
    }
}

fn do_math() {
    println!("Enter your favorite number:");
    let a = read_integer();

    println!("Enter your second favorite number:");
    let b = read_integer();

    unsafe {
        println!("\nSum of your favorite numbers: {}", add(a, b));
        println!("Difference of your favorite numbers: {}", subtract(a, b));
        println!("Product of your favorite numbers: {}", multiply(a, b));
        println!("Division of your favorite numbers: {:.5}\n", divide(a, b));
    }
}

fn main() {
    let mut user_input = String::new();

    loop {
        do_math();

        println!("To repeat with new numbers press [Enter]");
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read user input");
    }
}
