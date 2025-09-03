/*
Program written by Cody C.

RUST Practice Project #1 - Temperature Conversion Tool
This tool will allow you to convert a measurement from one temperature scale to another.
 */

/*
!! NOTE: !!
This program in particular, including the custom modules and submodules this program uses, is the first time I have ever written code in RUST.
Additionally, RUST is the first low-level programming language I have used; prior to creating this program, I had only used languages like LUA, Java, Python, and C#.
I used RUST documentation, online forums, and my experience in programming in order to write this program.
No AI-generated code was directly used; however, AI was used as a learning/debugging tool.
 */

use crate::{mkprint, mkprintln, prompt};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TempScale { name: &'static str, symbol: &'static str, to_c: fn(f64) -> f64, from_c: fn(f64) -> f64, }

// Create a static slice of TempScale structs that contains data for scale formatting and conversions
static SCALES: &[TempScale] = &[
    TempScale { name: "Celsius", symbol: "°C", to_c: |x| x, from_c: |x| x, },
    TempScale { name: "Fahrenheit", symbol: "°F", to_c: |f| (f - 32.0) * 5.0 / 9.0, from_c: |c| c * 9.0 / 5.0 + 32.0, },
    TempScale { name: "Kelvin", symbol: "K", to_c: |k| k - 273.15, from_c: |c| c + 273.15, },
    TempScale { name: "Rankine", symbol: "°R", to_c: |r| (r - 491.67) * 5.0 / 9.0, from_c: |c| (c + 273.15) * 9.0 / 5.0, },
    TempScale { name: "Réaumur", symbol: "°Ré", to_c: |re| re * 5.0 / 4.0, from_c: |c| c * 4.0 / 5.0, },
];

fn normalize(s: &str) -> String {
    let mut out = s.trim().to_lowercase();
    out = out.replace('°', "");
    out = out.replace('é', "e");
    out
}

fn resolve_scale(input: &str) -> Option<TempScale> {
    let raw = input.trim();
    if raw.is_empty() {
        return None;
    }

    if let Ok(idx) = raw.parse::<usize>() {
        if (1..=SCALES.len()).contains(&idx) {
            return Some(SCALES[idx - 1]);
        }
    }

    let norm = normalize(raw);
    for scale in SCALES.iter() {
        if normalize(scale.name) == norm || normalize(scale.symbol) == norm {
            return Some(*scale);
        }
    }

    None
}

fn choose_scale(label: &str, disallow: Option<TempScale>) -> TempScale {
    let mut accept_string: [(fn(&String) -> bool, &str); 1] = [(|_: &String| false, "")];

    loop {
        let input: String = prompt!(String, label, &mut accept_string);

        match resolve_scale(&input) {
            None => {
                mkprintln!("[color=red]Please enter a [color=white]valid scale[/color] (number, name, or symbol).[/color]");
                continue;
            }
            Some(s) => {
                if Some(s) == disallow {
                    mkprintln!("Invalid; cannot repeat selection.");
                    continue;
                }
                return s;
            }
        }
    }
}

pub fn run() {
    let scales = SCALES.to_vec();

    println!("Available scales:");

    (1..).zip(scales).for_each(|(i, s)| mkprintln!("{i}: {} ({})", s.name, s.symbol));

    let in_scale = choose_scale("Select input scale: ", None);
    let out_scale = choose_scale(
        "Select output scale: ",
        Some(in_scale),
    );

    let mut f64_validators: [(fn(&f64) -> bool, &str); 1] = [(|_: &f64| false, "")];
    let prompt_value = format!("Enter temperature in {} ({}): ", in_scale.name, in_scale.symbol);
    let value: f64 = prompt!(f64, &prompt_value, &mut f64_validators);

    let c = (in_scale.to_c)(value);
    let converted = (out_scale.from_c)(c);

    mkprintln!("{:.3}{} is equal to {:.3}{}.", value, in_scale.symbol, converted, out_scale.symbol);
}