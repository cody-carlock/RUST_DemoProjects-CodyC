use crate::{mdprintln, prompt};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

pub fn run() {
    let scales = SCALES.to_vec();

    mdprintln!(200; "[color=#8FCFFF]Available scales:");
    (1..).zip(scales).for_each(|(i, s)| mdprintln!(200; "[color=darkblue]{i}: [bold][color=blue]{}[/color][/bold] ([bold][color=blue]{}[/color][/bold])[/color]", s.name, s.symbol));

    let in_scale = choose_scale("[color=#8FCFFF]Select input scale: ", None);
    let out_scale = choose_scale("[color=#8FCFFF]Select output scale: ", Some(in_scale));

    let mut f64_validators: [(fn(&f64) -> bool, &str); 1] = [(|_: &f64| false, "")];
    let prompt_value = format!("[color=#8FCFFF]Enter temperature in [bold][color=blue]{} ({})[/color][/bold]: ", in_scale.name, in_scale.symbol);
    let value: f64 = prompt!(f64, &prompt_value, &mut f64_validators);

    let c = (in_scale.to_c)(value);
    let converted = (out_scale.from_c)(c);

    mdprintln!(400; "[color=#8FCFFF][bold][color=blue]{:.2}{} ({})[/color][/bold] is equal to [bold][color=blue]{:.2}{} ({})[/color][/bold].",
        value, in_scale.symbol, in_scale.name, converted, out_scale.symbol, out_scale.name);
}

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
                mdprintln!(400; "[color=red][bold]Please enter a valid scale (number, name, or symbol).");
                continue;
            }
            Some(s) => {
                if Some(s) == disallow {
                    mdprintln!(400; "[color=red][bold]Invalid input; cannot repeat selection.");
                    continue;
                }
                return s;
            }
        }
    }
}