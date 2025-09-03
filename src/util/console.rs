/*
Submodule written by Cody C.
 */

use std::{
    io::{stdin, stdout, Write},
    thread::{self, sleep},
    time::Duration,
    str::FromStr,
    fmt,
};
use crossterm::{
    style::{Color, PrintStyledContent, Stylize},
    ExecutableCommand,
};

const DEFAULT_PRINT_MS: u64 = 300;

#[derive(Debug, Clone, Copy)]
struct Style {
    color: Option<Color>,
    bold: bool,
    underline: bool,
    delay_ms: Option<u64>,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            color: None,
            bold: false,
            underline: false,
            delay_ms: None,
        }
    }
}

fn maybe_sleep(ms: u64) {
    if ms > 0 {
        sleep(Duration::from_millis(ms));
    }
}

fn parse_tag(tag: &str, current: &Style) -> Style {
    let mut style = *current;

    if tag.eq_ignore_ascii_case("bold") {
        style.bold = true;
    } else if tag.eq_ignore_ascii_case("underline") {
        style.underline = true;
    } else if tag.starts_with("delay=") {
        if let Ok(ms) = tag[6..].parse::<u64>() {
            style.delay_ms = Some(ms);
        }
    } else if tag.starts_with("color=") {
        let c = &tag[6..];
        if c.starts_with('#') && c.len() == 7 {
            if let (Ok(r), Ok(g), Ok(b)) = (
                u8::from_str_radix(&c[1..3], 16),
                u8::from_str_radix(&c[3..5], 16),
                u8::from_str_radix(&c[5..7], 16),
            ) {
                style.color = Some(Color::Rgb { r, g, b });
            }
        } else {
            style.color = match c.to_lowercase().as_str() {
                "red" => Some(Color::Red),
                "green" => Some(Color::Green),
                "blue" => Some(Color::Blue),
                "yellow" => Some(Color::Yellow),
                "magenta" => Some(Color::Magenta),
                "cyan" => Some(Color::Cyan),
                "white" => Some(Color::White),
                _ => None,
            };
        }
    }
    style
}

pub(crate) fn print_markup(input: &str, newline: bool) {
    let mut stdout = stdout();
    let mut current_style = Style::default();
    let mut rest = input;

    while let Some(start_idx) = rest.find('[') {
        // print text before the tag
        if start_idx > 0 {
            let before = &rest[..start_idx];
            let mut styled = before.to_string().with(current_style.color.unwrap_or(Color::White));
            if current_style.bold {
                styled = styled.bold();
            }
            if current_style.underline {
                styled = styled.underlined();
            }
            stdout.execute(PrintStyledContent(styled)).unwrap();

            if let Some(ms) = current_style.delay_ms {
                sleep(Duration::from_millis(ms));
            }

        }

        rest = &rest[start_idx..];

        if let Some(end_idx) = rest.find(']') {
            let tag = &rest[1..end_idx];
            current_style = parse_tag(tag, &current_style);
            rest = &rest[end_idx + 1..];
        } else {
            break;
        }
    }

    if !rest.is_empty() {
        let mut styled = rest.with(current_style.color.unwrap_or(Color::White));
        if current_style.bold {
            styled = styled.bold();
        }
        if current_style.underline {
            styled = styled.underlined();
        }
        stdout.execute(PrintStyledContent(styled)).unwrap();

        if let Some(ms) = current_style.delay_ms {
            sleep(Duration::from_millis(ms));
        }
    }

    if newline { println!(); }
}

#[macro_export]
macro_rules! mkprint {
    ($delay:expr, $fmt:literal $(, $($arg:tt)+)?) => {{
        let s = format!($fmt $(, $($arg)+)?);
        $crate::util::console::print_markup(&s, false);
        sleep($delay);
    }};
    ($fmt:literal $(, $($arg:tt)+)?) => {{
        let s = format!($fmt $(, $($arg)+)?);
        $crate::util::console::print_markup(&s, false);
    }};
}

#[macro_export]
macro_rules! mkprintln {
    ($delay:expr, $fmt:literal $(, $($arg:tt)+)?) => {{
        let s = format!($fmt $(, $($arg)+)?);
        $crate::util::console::print_markup(&s, true);
        sleep($delay);
    }};
    ($fmt:literal $(, $($arg:tt)+)?) => {{
        let s = format!($fmt $(, $($arg)+)?);
        $crate::util::console::print_markup(&s, true);
    }};
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
    <T as FromStr>::Err: fmt::Debug,
{
    loop {
        // Print the prompt and flush immediately
        print!("{}", prompt);
        stdout().flush().unwrap();

        // Read the line of input from stdin, then trim whitespace
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
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