# Temperature Converter Module

## Overview
The `temp_converter` module provides a simple command-line interface for converting temperatures between Celsius, Fahrenheit, and Kelvin scales. The module demonstrates proper user input validation and formatted output display.

## Features

- Converts between three temperature scales (Celsius, Fahrenheit, Kelvin)
- Input validation with helpful error messages
- Clean terminal interface with formatted output
- User-friendly prompts and instructions

## Main Function

### `run`

```rust
pub fn run()
```

Executes the temperature converter program, handling the entire conversion flow from user input to result display.

## Temperature Scales

The module supports the following temperature scales:

1. **Celsius** - The metric temperature scale where water freezes at 0° and boils at 100°
2. **Fahrenheit** - The imperial temperature scale where water freezes at 32° and boils at 212°
3. **Kelvin** - The scientific temperature scale where absolute zero is 0K (equivalent to -273.15°C)

## Conversion Formulas

- **Celsius to Fahrenheit**: `°F = (°C × 9/5) + 32`
- **Celsius to Kelvin**: `K = °C + 273.15`
- **Fahrenheit to Celsius**: `°C = (°F - 32) × 5/9`
- **Fahrenheit to Kelvin**: `K = (°F - 32) × 5/9 + 273.15`
- **Kelvin to Celsius**: `°C = K - 273.15`
- **Kelvin to Fahrenheit**: `°F = (K - 273.15) × 9/5 + 32`

## Usage

When run, the program will:

1. Prompt the user to select a source temperature scale
2. Ask for the temperature value to convert
3. Prompt the user to select a target temperature scale
4. Display the converted temperature with appropriate formatting

## Input Validation

The module validates:

- Menu selection inputs (must be valid numbers corresponding to available options)
- Temperature inputs (must be valid numbers within physically possible ranges)

## Example

```
Select source temperature scale:
1. Celsius
2. Fahrenheit
3. Kelvin

Enter your choice: 1
Enter temperature in Celsius: 100

Select target temperature scale:
1. Celsius
2. Fahrenheit
3. Kelvin

Enter your choice: 2

100.0°C = 212.0°F
```

## Implementation Notes

The module uses the `prompt!` macro for collecting and validating user input and the `mdprintln!` macro for formatted output display.
