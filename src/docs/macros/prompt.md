Interactive typed input method with input validation. 
# `prompt!` Macro

## Overview
The `prompt!` macro provides a convenient way to create interactive command-line prompts that collect and validate user input.

## Usage

```rust
// Basic usage - collect a value of specified type with validation
prompt!(type, prompt_string, validators)

// With delay - adds a delay after returning output
prompt!(type, delay_ms, prompt_string, validators)
```

## Parameters

- `type`: The Rust type to parse the input into (e.g., `String`, `i32`, `f64`)
- `prompt_string`: Text to display as the prompt
- `validators`: A mutable reference to an array of validator tuples, each containing:
  - A function that takes a reference to the parsed value and returns a boolean
  - An error message string to display if validation fails
- `delay_ms` (optional): Milliseconds to delay after returning the value

## Examples

```rust
// Prompt for a string with no validation
let mut empty_validators: [(fn(&String) -> bool, &str); 1] = [(|_| false, "")];
let name: String = prompt!(String, "Enter your name: ", &mut empty_validators);

// Prompt for an integer with validation and 500ms delay after input
let mut validators = [
    (|&n: &i32| n > 0, "Value must be positive")
];
let age: i32 = prompt!(i32, 500, "Enter your age: ", &mut validators);
```

## Notes
- The validator array must have at least one element, even if no validation is needed
- If validation fails, the prompt will repeat until valid input is received
- Uses `FromStr` trait for type conversion, so custom types must implement this trait
# `prompt!` Macro

## Overview
The `prompt!` macro provides a convenient way to create interactive command-line prompts that collect and validate user input.

## Usage

```rust
// Basic usage - collect a value of specified type with validation
prompt!(type, prompt_string, validators)

// With delay - adds a delay after returning output
prompt!(type, delay_ms, prompt_string, validators)
```

## Parameters

- `type`: The Rust type to parse the input into (e.g., `String`, `i32`, `f64`)
- `prompt_string`: Text to display as the prompt
- `validators`: A mutable reference to an array of validator tuples, each containing:
  - A function that takes a reference to the parsed value and returns a boolean
  - An error message string to display if validation fails
- `delay_ms` (optional): Milliseconds to delay after returning the value

## Examples

```rust
// Prompt for a string with no validation
let mut empty_validators: [(fn(&String) -> bool, &str); 1] = [(|_| false, "")];
let name: String = prompt!(String, "Enter your name: ", &mut empty_validators);

// Prompt for an integer with validation and 500ms delay after input
let mut validators = [
    (|&n: &i32| n > 0, "Value must be positive")
];
let age: i32 = prompt!(i32, 500, "Enter your age: ", &mut validators);
```

## Notes
- The validator array must have at least one element, even if no validation is needed
- If validation fails, the prompt will repeat until valid input is received
- Uses `FromStr` trait for type conversion, so custom types must implement this trait