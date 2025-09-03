//I know there's already a library for this, I just thought this would be a fun thing to toy around with myself :)

use std::{io::{self, Write}, time::Duration};
use std::thread::sleep;

use crossterm::{
    queue,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
};

#[doc = include_str!("../docs/macros/mdprint.md")]
#[macro_export]
macro_rules! mdprint {
    ($fmt:literal $(, $args:expr)* $(,)?) => {{
        $crate::util::markdown::render_markdown(
            &::std::format!($fmt $(, $args)*),
            false,
            0,
        )
    }};

    ($ms:expr; $fmt:literal $(, $args:expr)* $(,)?) => {{
        $crate::util::markdown::render_markdown(
            &::std::format!($fmt $(, $args)*),
            false,
            $ms,
        )
    }};
}

#[doc = include_str!("../docs/macros/mdprintln.md")]
#[macro_export]
macro_rules! mdprintln {
    ($fmt:literal $(, $args:expr)* $(,)?) => {{
        $crate::util::markdown::render_markdown(
            &::std::format!($fmt $(, $args)*),
            true,
            0,
        )
    }};

    ($ms:expr; $fmt:literal $(, $args:expr)* $(,)?) => {{
        $crate::util::markdown::render_markdown(
            &::std::format!($fmt $(, $args)*),
            true,
            $ms,
        )
    }};
}


pub(crate) fn render_markdown(input: &str, newline: bool, delay: u64) {
    let parser = Parser::new(input);
    let runs = parser.parse();
    render_runs(&runs).expect("Markdown failed to render.");
    if newline { println!() }
    if delay > 0 { sleep(Duration::from_millis(delay)) }
}

#[derive(Clone, Copy, Debug, Default)]
struct StyleState {
    bold: bool,
    underline: bool,
    color: Option<Color>,
}

#[derive(Clone, Debug)]
struct Run {
    text: String,
    style: StyleState,
}

#[derive(Debug)]
struct Parser<'a> {
    input: &'a str,
    pos: usize,
    stack: Vec<FrameKind>,
    current: StyleState,
    runs: Vec<Run>,
}

#[derive(Debug)]
enum FrameKind {
    Bold(bool),
    Underline(bool),
    Color(Option<Color>),
}

#[derive(Debug, Clone, Copy)]
enum TagToken {
    Open(TagOpen),
    Close(TagClose),
}

#[derive(Debug, Clone, Copy)]
enum TagOpen {
    Bold,
    Underline,
    Color(Color),
}

#[derive(Debug, Clone, Copy)]
enum TagClose {
    Bold,
    Underline,
    Color,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input,
            pos: 0,
            stack: Vec::new(),
            current: StyleState::default(),
            runs: Vec::new(),
        }
    }

    fn parse(mut self) -> Vec<Run> {
        let bytes = self.input.as_bytes();
        while self.pos < self.input.len() {
            if bytes[self.pos] == b'[' {
                if let Some(tok) = self.try_parse_tag() {
                    self.apply_tag(tok);
                    continue;
                } else {
                    // Not a tag; treat literal '['
                    self.push_text("[");
                    self.pos += 1;
                    continue;
                }
            }

            // Accumulate a chunk of plain text until next '[' or end
            let start = self.pos;
            while self.pos < self.input.len() && self.input.as_bytes()[self.pos] != b'[' {
                self.pos += 1;
            }
            if self.pos > start {
                self.push_text(&self.input[start..self.pos]);
            }
        }
        self.runs
    }

    fn push_text(&mut self, s: &str) {
        if s.is_empty() {
            return;
        }
        self.runs.push(Run {
            text: s.to_string(),
            style: self.current,
        });
    }

    fn try_parse_tag(&mut self) -> Option<TagToken> {
        debug_assert!(self.pos < self.input.len());
        let start = self.pos;
        if self.input.as_bytes()[start] != b'[' {
            return None;
        }

        // Find the closing ']'
        let rest = &self.input[start + 1..];
        let close_idx = rest.find(']')?;
        let inner = &rest[..close_idx];

        // Advance cursor past the entire tag
        self.pos = start + 1 + close_idx + 1;

        if inner.starts_with('/') {
            // Closer
            let name = &inner[1..].trim();
            let kind = match name.to_ascii_lowercase().as_str() {
                "bold" | "b" => TagClose::Bold,
                "underline" | "ul" => TagClose::Underline,
                "color" | "colour" | "c" => TagClose::Color,
                _ => {
                    // Not a recognized tag -> treat literally
                    // Rewind and return None
                    self.pos = start;
                    return None;
                }
            };
            Some(TagToken::Close(kind))
        } else {
            // Opener
            let inner = inner.trim();
            // Forms:
            //   bold
            //   underline
            //   color=red
            let (name, value) = if let Some(eq) = inner.find('=') {
                (inner[..eq].trim(), Some(inner[eq + 1..].trim()))
            } else {
                (inner, None)
            };

            let token = match name.to_ascii_lowercase().as_str() {
                "bold" => Some(TagToken::Open(TagOpen::Bold)),
                "underline" | "ul" => Some(TagToken::Open(TagOpen::Underline)),
                "color" | "colour" | "fg" => {
                    let v = value?;
                    let color = resolve_color(v)?;
                    Some(TagToken::Open(TagOpen::Color(color)))
                }
                _ => None,
            };

            if token.is_none() {
                // Not a recognized tag
                self.pos = start;
            }
            token
        }
    }

    fn apply_tag(&mut self, tok: TagToken) {
        match tok {
            TagToken::Open(open) => {
                match open {
                    TagOpen::Bold => {
                        let prev = self.current.bold;
                        self.current.bold = true;
                        self.stack.push(FrameKind::Bold(prev));
                    }
                    TagOpen::Underline => {
                        let prev = self.current.underline;
                        self.current.underline = true;
                        self.stack.push(FrameKind::Underline(prev));
                    }
                    TagOpen::Color(color) => {
                        let prev = self.current.color;
                        self.current.color = Some(color);
                        self.stack.push(FrameKind::Color(prev));
                    }
                }
            }
            TagToken::Close(kind) => {
                // Find nearest matching opener in the stack (supports crossed closures)
                let idx_opt = self.stack.iter().rposition(|f| match kind {
                    TagClose::Bold => matches!(f, FrameKind::Bold(_)),
                    TagClose::Underline => matches!(f, FrameKind::Underline(_)),
                    TagClose::Color => matches!(f, FrameKind::Color(_)),
                });

                // No matching opener -> swallow stray closer
                let Some(idx) = idx_opt else {
                    return;
                };

                // Temporarily remove frames above the target, remembering how to re-open them
                let mut to_reopen: Vec<TagOpen> = Vec::new();
                while self.stack.len() > idx + 1 {
                    let f = self.stack.pop().unwrap();
                    match f {
                        FrameKind::Bold(prev) => {
                            let was = self.current.bold;
                            self.current.bold = prev;
                            if was {
                                to_reopen.push(TagOpen::Bold);
                            }
                        }
                        FrameKind::Underline(prev) => {
                            let was = self.current.underline;
                            self.current.underline = prev;
                            if was {
                                to_reopen.push(TagOpen::Underline);
                            }
                        }
                        FrameKind::Color(prev) => {
                            let was = self.current.color;
                            self.current.color = prev;
                            if let Some(c) = was {
                                to_reopen.push(TagOpen::Color(c));
                            }
                        }
                    }
                }

                // Pop and close the matching target
                if let Some(target) = self.stack.pop() {
                    match (kind, target) {
                        (TagClose::Bold, FrameKind::Bold(prev)) => self.current.bold = prev,
                        (TagClose::Underline, FrameKind::Underline(prev)) => {
                            self.current.underline = prev
                        }
                        (TagClose::Color, FrameKind::Color(prev)) => self.current.color = prev,
                        _ => {}
                    }
                }

                // Re-open the frames we temporarily removed (in original order)
                for open in to_reopen.into_iter().rev() {
                    self.apply_tag(TagToken::Open(open));
                }
            }
        }
    }
}

// Hex parsing support for [color=#RRGGBB], [color=#RGB], and alpha variants (alpha ignored).
fn parse_hex_color(s: &str) -> Option<Color> {
    let hex = s.strip_prefix('#').unwrap_or(s).trim();

    fn hex_val(b: u8) -> Option<u8> {
        match b {
            b'0'..=b'9' => Some(b - b'0'),
            b'a'..=b'f' => Some(10 + (b - b'a')),
            b'A'..=b'F' => Some(10 + (b - b'A')),
            _ => None,
        }
    }

    fn byte_from_pair(hi: u8, lo: u8) -> Option<u8> {
        Some((hex_val(hi)? << 4) | hex_val(lo)?)
    }

    match hex.len() {
        3 => {
            // #RGB -> expand to #RRGGBB
            let b = hex.as_bytes();
            let r = byte_from_pair(b[0], b[0])?;
            let g = byte_from_pair(b[1], b[1])?;
            let bl = byte_from_pair(b[2], b[2])?;
            Some(Color::Rgb { r, g, b: bl })
        }
        4 => {
            // #RGBA -> ignore alpha
            let b = hex.as_bytes();
            let r = byte_from_pair(b[0], b[0])?;
            let g = byte_from_pair(b[1], b[1])?;
            let bl = byte_from_pair(b[2], b[2])?;
            Some(Color::Rgb { r, g, b: bl })
        }
        6 => {
            // #RRGGBB
            let b = hex.as_bytes();
            let r = byte_from_pair(b[0], b[1])?;
            let g = byte_from_pair(b[2], b[3])?;
            let bl = byte_from_pair(b[4], b[5])?;
            Some(Color::Rgb { r, g, b: bl })
        }
        8 => {
            // #RRGGBBAA -> ignore alpha
            let b = hex.as_bytes();
            let r = byte_from_pair(b[0], b[1])?;
            let g = byte_from_pair(b[2], b[3])?;
            let bl = byte_from_pair(b[4], b[5])?;
            Some(Color::Rgb { r, g, b: bl })
        }
        _ => None,
    }
}

// Unified color resolver: supports hex (#RRGGBB / #RGB), 0-255 ANSI index, and common names.
fn resolve_color(spec: &str) -> Option<Color> {
    let s = spec.trim();

    if s.starts_with('#') {
        return parse_hex_color(s);
    }

    if let Ok(idx) = s.parse::<u8>() {
        return Some(Color::AnsiValue(idx));
    }

    match spec.to_ascii_lowercase().as_str() {
        "black" => Some(Color::Black),
        "red" => Some(Color::Red),
        "green" => Some(Color::Green),
        "yellow" => Some(Color::Yellow),
        "blue" => Some(Color::Blue),
        "magenta" | "purple" => Some(Color::Magenta),
        "cyan" => Some(Color::Cyan),
        "white" => Some(Color::White),
        "grey" | "gray" => Some(Color::Grey),
        "darkgrey" | "darkgray" => Some(Color::DarkGrey),
        "darkred" => Some(Color::DarkRed),
        "darkgreen" => Some(Color::DarkGreen),
        "darkyellow" => Some(Color::DarkYellow),
        "darkblue" => Some(Color::DarkBlue),
        "darkmagenta" | "darkpurple" => Some(Color::DarkMagenta),
        "darkcyan" => Some(Color::DarkCyan),
        _ => None,
    }
}

fn render_runs(runs: &[Run]) -> Result<(), Box<dyn std::error::Error>> {
    let mut out = io::stdout();

    for run in runs {
        // Reset everything for each run, then apply the run's style
        queue!(out, SetAttribute(Attribute::Reset), ResetColor)?;

        if run.style.bold {
            queue!(out, SetAttribute(Attribute::Bold))?;
        }
        if run.style.underline {
            queue!(out, SetAttribute(Attribute::Underlined))?;
        }
        if let Some(c) = run.style.color {
            queue!(out, SetForegroundColor(c))?;
        }

        queue!(out, Print(&run.text))?;

        out.flush()?;
    }

    // Final reset
    queue!(out, SetAttribute(Attribute::Reset), ResetColor)?;
    out.flush()?;
    Ok(())
}