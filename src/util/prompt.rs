use std::fmt;
use std::io::{stdin, stdout, Write};
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;
use crate::{mdprint, mdprintln};

#[macro_export]
macro_rules! prompt {
    ($ty:ty, $prompt:expr, $validators:expr) => {{
        $crate::util::prompt::create_prompt::<$ty, _>($prompt, $validators, 0)
    }};
    ($ty:ty, $delay_ms:expr, $prompt:expr, $validators:expr) => {{
        $crate::util::prompt::create_prompt::<$ty, _>($prompt, $validators, $delay_ms)
    }};
}

/// Create a prompt that reads a value of type `T` from stdin, validated by provided checks.
/// If `delay_ms` > 0, waits that long after returning output.
pub(crate) fn create_prompt<T, G>(prompt: &str, validators: &mut [(G, &str)], delay_ms: u64) -> T
where
    T: FromStr,
    G: FnMut(&T) -> bool,
    <T as FromStr>::Err: fmt::Debug,
{
    loop {
        // Buffer and flush the prompt so it appears immediately.
        mdprint!("{}", prompt);
        stdout().flush().unwrap();

        // Capture user input and trim trailing whitespace/newlines.
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        // Attempt to parse the raw input into the target type.
        match input.parse::<T>() {
            Ok(value) => {
                for (check, err) in validators.iter_mut() {
                    // Evaluate the validator:
                    // - If it returns false, accept the value immediately.
                    // - If it returns true, print the associated error message and (optionally) pause.
                    if !check(&value) {
                        return value;
                    } else {
                        mdprintln!("{}", err);
                        if delay_ms > 0 { sleep(Duration::from_millis(delay_ms)); }
                    }
                }
            }
            Err(_) => {
                // Parsing failed; inform the user of the expected type and (optionally) pause.
                mdprintln!(
                    "[color=red][bold]Incorrect input type. Expected input type: [/bold]{}",
                    std::any::type_name::<T>()
                );
                if delay_ms > 0 { sleep(Duration::from_millis(delay_ms)); }
            }
        }
    }
}
