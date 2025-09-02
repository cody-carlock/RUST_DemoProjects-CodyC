/*
Submodule written by Cody C.
 */

use std::io::{self, Write}; // Imports `io` module and brings the trait `Write` into scope.
use std::str::FromStr; // Imports the trait `FromStr` from the module `str`.

pub fn prompt<T, G>(prompt: &str, validators: &[(G, &str)]) -> T
where
    T: FromStr,
    G: Fn(&T) -> bool,
    <T as FromStr>::Err: std::fmt::Debug,
{
    loop {
        // Print the prompt and flush immediately
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        // Read the line of input from stdin, then trim whitespace
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        // Try parsing into `T`:
        match input.parse::<T>() {
            Ok(value) => {
                for (check, err) in validators {
                    // Check if the value passes the validator
                    if !check(&value) {
                        return value; // return valid input
                    } else {
                        println!("{}", err); // validation failed; print error message
                    }
                }
            }
            Err(_) => {
                println!("Incorrect input type. Expected input: {}", std::any::type_name::<T>()); // parsing failed
            }
        }
    }
}