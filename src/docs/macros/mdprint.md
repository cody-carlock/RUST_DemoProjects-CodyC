Markdown-esque print without a trailing newline.
# `mdprint!` Macro

## Overview
The `mdprint!` macro is designed for rendering Markdown-formatted text to the terminal without automatically adding a newline. It's ideal for in-place updates, prompts, and other cases where newlines should be controlled manually.

## Parameters

- `fmt`: A format string literal that can include Markdown syntax
- `args`: Optional format arguments that will be substituted into the format string
- `ms` (optional): Milliseconds to delay between rendering elements for animated output

## Markdown Support

The macro supports a subset of Markdown syntax including:

- Headers (`# Header`)
- Emphasis (`*italic*`, `**bold**`)
- Lists (ordered and unordered)
- Code blocks and inline code
- Custom color syntax: `[color=red]colored text[/color]`

## Examples

## Examples

```rust
// Basic formatting
mdprint!("Welcome to the application");

// Basic formatting with 100ms delay
mdprint!(100; "Welcome to the application");

// Combining formatting
mdprint!("Status Report\n- Item 1: [color=green]Complete[/color]\n- Item 2: [color=yellow]Pending[/color]");
```

## Notes
- Unlike `mdprintln!`, this macro does not automatically append a newline
- For multi-line output with automatic newlines, use `mdprintln!` instead
- The macro renders directly to stdout and cannot be captured as a string