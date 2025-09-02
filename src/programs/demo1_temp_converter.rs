/*
Program written by Cody C.

RUST Practice Project #1 - Temperature Conversion Tool
This tool will allow you to convert a measurement from one temperature scale to another.
 */

/*
!! NOTE: !!
This program in particular, including the custom modules and submodules this program uses, is the first time I have ever written code in RUST.
Additionally, RUST is the first low-level programming language I have used; prior to creating this program, I had only used languages like LUA, Java, Python, and C#.
I used RUST documentation, online forums, and my experience in programming in order to write this program. No AI-generated code was used.
 */

use crate::{dprintln, prompt};

// Create a struct for simplified usage of temperature scales
#[derive(Debug, Clone, Copy)]
struct TempScale {
    name: &'static str,
    symbol: &'static str,
    to_c: fn(f64) -> f64,
    from_c: fn(f64) -> f64,
}

// Create a static slice of TempScale structs that contains data for scale formatting and conversions
static SCALES: &[TempScale] = &[
    TempScale { name: "Celsius", symbol: "°C", to_c: |x| x, from_c: |x| x, },
    TempScale { name: "Fahrenheit", symbol: "°F", to_c: |f| (f - 32.0) * 5.0 / 9.0, from_c: |c| c * 9.0 / 5.0 + 32.0, },
    TempScale { name: "Kelvin", symbol: "K", to_c: |k| k - 273.15, from_c: |c| c + 273.15, },
    TempScale { name: "Rankine", symbol: "°R", to_c: |r| (r - 491.67) * 5.0 / 9.0, from_c: |c| (c + 273.15) * 9.0 / 5.0, },
    TempScale { name: "Réaumur", symbol: "°Ré", to_c: |re| re * 5.0 / 4.0, from_c: |c| c * 4.0 / 5.0, },
];

// Dynamically generate a selection of prompt choices that can be appended to the prompt
fn format_scales(scales: &Vec<TempScale>) -> String {
    scales
        .iter()
        .enumerate()
        .map(|(i,s)| format!("{}: {}", i + 1, s.name))
        .collect::<Vec<String>>()
        .join("\n")
}

// Run program
pub fn convert_temp() {
    println!("Running temperature converter!");

    // Create a mutable vector that contains remaining scale choices
    let mut available: Vec<TempScale> = SCALES.to_vec();

    // Closure that tests to see if a string input either matches the name or index of an item in available
    let validate_scale = |input: &str| -> Option<TempScale> {
        let s = input.trim().to_lowercase();

        if let Ok(n) = s.parse::<usize>() {
            if (1..=available.len()).contains(&n) {
                return Some(available[n - 1]);
            }
        }

        available
            .iter()
            .position(|scale| scale.name.eq_ignore_ascii_case(&s))
            .map(|i| available[i])
    };

    // Cache the validated TempScale exactly once inside the validator closure.
    let mut chosen: Option<TempScale> = None;

    // Note: validator returns true => invalid, false => valid.
    let mut validator = [(
        |s: &String| {
            chosen = validate_scale(s); // run once, cache result
            chosen.is_none()            // invalid if None
        },
        "Your selection is invalid.",   // custom error text
    )];

    let msg_from = format!("Please select a scale to convert from: \n{}\nSelection: ", format_scales(&available)); // build the message that lists available scales
    let _input: String = prompt!(String, 300, &msg_from, &mut validator); // run prompt with the mutable validator
    let from_scale: TempScale = chosen.unwrap(); // safe; unwraps the chosen input

    // Remove the selected TempScale from `available`
    if let Some(idx) = available.iter().position(|sc| sc.name == from_scale.name) {
        available.remove(idx);
    }
    dprintln!("Available: \n{}", format_scales(&available));
    dprintln!("Selected scale: {}", from_scale.name);
    dprintln!("Remaining: \n{}", format_scales(&available));
}