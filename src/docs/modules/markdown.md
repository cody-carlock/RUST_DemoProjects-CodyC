
# Markdown Module

## Overview
The `markdown` module provides utilities for rendering and displaying Markdown-formatted text in the console. This module enables rich text formatting within terminal applications, allowing for more visually appealing and organized output.

## Features

- Renders Markdown syntax as formatted terminal text
- Supports basic Markdown formatting like headers, emphasis, links, and lists
- Handles color syntax for terminal output
- Configurable delay options for animated text display

## Functions

### `render_markdown`

Renders a Markdown string to the terminal with formatting.

```rust
pub fn render_markdown(text: &str, add_newline: bool, delay_ms: u64)
```

#### Parameters
- `text`: The Markdown-formatted string to render
- `add_newline`: Whether to add a newline after rendering
- `delay_ms`: Delay in milliseconds between rendering elements (0 for no delay)

## Custom Syntax

In addition to standard Markdown, this module supports custom syntax for terminal-specific formatting:

- `[color=name]text[/color]` - Displays text in the specified color
- `[bold]text[/bold]` - Displays bolded text
- `[underline]text[/underline]` - Displays text with an underline

## Usage

Typically used through the `mdprintln!` macro rather than called directly. See the documentation for that macro for more detailed usage examples.