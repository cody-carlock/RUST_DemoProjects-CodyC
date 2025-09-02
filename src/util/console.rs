/*
Submodule written by Cody C.
 */

use std::{ io::{self, Write}, thread, time::Duration, str::FromStr, fmt, };

const DEFAULT_PRINT_MS: u64 = 300;

fn maybe_sleep(ms: u64) {
    if ms > 0 {
        thread::sleep(Duration::from_millis(ms));
    }
}

pub(crate) fn delay_write(args: fmt::Arguments<'_>, newline: bool, ms: Option<u64>) {
    let mut out = io::stdout();
    out.write_fmt(args).ok();
    if newline {
        out.write_all(b"\n").ok();
    }
    out.flush().ok();

    let delay = ms.unwrap_or(DEFAULT_PRINT_MS);
    thread::sleep(Duration::from_millis(delay));
}

#[macro_export]
macro_rules! dprint {
    ($fmt:literal $(, $($arg:tt)+)?) => {
        $crate::util::console::delay_write(format_args!($fmt $(, $($arg)+)?), false, None)
    };
    ($delay:expr, $fmt:literal $(, $($arg:tt)+)?) => {
        $crate::util::console::delay_write(format_args!($fmt $(, $($arg)+)?), false, Some($delay))
    };
}

#[macro_export]
macro_rules! dprintln {
    ($fmt:literal $(, $($arg:tt)+)?) => {
        $crate::util::console::delay_write(format_args!($fmt $(, $($arg)+)?), true, None)
    };
    ($delay:expr, $fmt:literal $(, $($arg:tt)+)?) => {
        $crate::util::console::delay_write(format_args!($fmt $(, $($arg)+)?), true, Some($delay))
    };
}

#[macro_export]
macro_rules! prompt {
    ($ty:ty, $prompt:expr, $validators:expr) => {{
        $crate::util::console::do_prompt::<$ty, _>($prompt, $validators, 0)
    }};
    ($ty:ty, $delay_ms:expr, $prompt:expr, $validators:expr) => {{
        $crate::util::console::do_prompt::<$ty, _>($prompt, $validators, $delay_ms)
    }};
}

pub(crate) fn do_prompt<T, G>(prompt: &str, validators: &mut [(G, &str)], delay_ms: u64) -> T
where
    T: FromStr,
    G: FnMut(&T) -> bool,
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
                for (check, err) in validators.iter_mut() {
                    // Check if the value passes the validator
                    if !check(&value) {
                        return value; // return valid input
                    } else {
                        println!("{}", err); // validation failed; print error message
                        maybe_sleep(delay_ms);
                    }
                }
            }
            Err(_) => {
                println!("Incorrect input type. Expected input: {}", std::any::type_name::<T>()); // parsing failed
                maybe_sleep(delay_ms);
            }
        }
    }
}