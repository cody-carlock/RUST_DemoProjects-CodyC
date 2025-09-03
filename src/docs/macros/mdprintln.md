Markdown-esque print followed by a newline.
# `mdprintln!` Macro

## Overview
The `mdprintln!` macro renders Markdown-formatted text to the terminal with automatic newline appending. It provides a convenient way to display rich, formatted text in console applications.

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

```rust
// Basic formatting
mdprintln!("Welcome to the application");

// Combining formatting
mdprintln!("Status Report\n- Item 1: [color=green]Complete[/color]\n- Item 2: [color=yellow]Pending[/color]");
```

## Implementation

The macro wraps a call to the `render_markdown` function in the `markdown` module with the `add_newline` parameter set to `true`.

```rust
render_markdown(formatted_string, true, delay_ms)
```

## See Also
- `mdprint!` - Similar functionality but without automatic newline