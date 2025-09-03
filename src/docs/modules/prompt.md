# Prompt Module

## Overview
The `prompt` module provides utilities for creating interactive command-line prompts that collect and validate user input. This module forms the foundation for all user interaction in the application.

## Core Function

### `create_prompt`

```rust
pub(crate) fn create_prompt<T, G>(prompt: &str, validators: &mut [(G, &str)], delay_ms: u64) -> T
where
    T: FromStr,
    G: FnMut(&T) -> bool,
    <T as FromStr>::Err: fmt::Debug
```

Creates a prompt that reads a value of type `T` from stdin and validates it according to the provided checks.

#### Parameters
- `prompt`: Text to display as the prompt
- `validators`: A mutable reference to an array of tuples, each containing:
  - A function that takes a reference to the parsed value and returns a boolean
  - An error message string to display if validation fails
- `delay_ms`: Milliseconds to delay after returning (if > 0)

#### Type Parameters
- `T`: The type to parse the input into (must implement `FromStr`)
- `G`: The type of the validator function

#### Returns
A value of type `T` after successful validation

## Usage

Typically used through the `prompt!` macro rather than called directly. The module handles:

- Displaying the prompt text
- Reading user input
- Parsing the input into the specified type
- Validating the parsed value
- Displaying error messages for failed validation
- Repeating the prompt until valid input is received

## Error Handling

The module handles two types of errors:
1. Parse errors: When input cannot be converted to type `T`
2. Validation errors: When input fails to meet the specified validation criteria