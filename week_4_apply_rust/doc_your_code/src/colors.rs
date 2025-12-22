//! Colorized output utilities for the terminal using ANSI escape codes.
//!
//! This module provides small helpers to wrap strings with ANSI escape codes so
//! they render in color (or bold) in terminals that support ANSI coloring.
//!
//! # Examples
//! ```
//! use cli_utils::colors::*;
//! println!("{} {} {}", red("Red"), green("Green"), blue("Blue"));
//! ```

/// Returns the input string wrapped in the ANSI escape code for red.
///
/// # Examples
/// ```
/// use cli_utils::colors::red;
/// assert_eq!(red("x"), "\x1b[31mx\x1b[0m");
/// ```
pub fn red(s: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", s)
}

/// Returns the input string wrapped in the ANSI escape code for green.
///
/// # Examples
/// ```
/// use cli_utils::colors::green;
/// assert_eq!(green("x"), "\x1b[32mx\x1b[0m");
/// ```
pub fn green(s: &str) -> String {
    format!("\x1b[32m{}\x1b[0m", s)
}

/// Returns the input string wrapped in the ANSI escape code for blue.
///
/// # Examples
/// ```
/// use cli_utils::colors::blue;
/// assert_eq!(blue("x"), "\x1b[34mx\x1b[0m");
/// ```
pub fn blue(s: &str) -> String {
    format!("\x1b[34m{}\x1b[0m", s)
}

/// Returns the input string wrapped in the ANSI code for bold text.
///
/// # Examples
/// ```
/// use cli_utils::colors::bold;
/// assert_eq!(bold("x"), "\x1b[1mx\x1b[0m");
/// ```
pub fn bold(s: &str) -> String {
    format!("\x1b[1m{}\x1b[0m", s)
}

/// Resets any styling and returns the input string unchanged (no color).
///
/// This helper is mainly provided for symmetry with the other helpers.
pub fn reset(s: &str) -> String {
    format!("\x1b[0m{}\x1b[0m", s)
}

/// A color or style that can be applied to a string.
///
/// Use this enum together with `ColorString` to represent the desired styling.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Color {
    /// Red text
    Red,
    /// Green text
    Green,
    /// Blue text
    Blue,
    /// Bold text
    Bold,
}

/// A string with an associated color and a cached colorized representation.
///
/// # Fields
/// - `color` — the `Color` to apply to `string`
/// - `string` — the original, non-colored text
/// - `colorized` — the generated, colorized string (updated by `paint`)
///
/// # Examples
/// ```
/// use cli_utils::colors::{Color, ColorString};
/// let mut s = ColorString::new(Color::Red, "hello");
/// s.paint();
/// assert!(s.colorized().contains("hello"));
/// ```
pub struct ColorString {
    /// The color or style to apply
    pub color: Color,
    /// The original string content
    pub string: String,
    /// Cached colorized representation (empty until painted)
    pub colorized: String,
}

impl ColorString {
    /// Create a new `ColorString` for the given `color` and `string`.
    ///
    /// The returned instance has an empty `colorized` field until `paint` or
    /// `into_colorized` is called.
    pub fn new(color: Color, string: impl Into<String>) -> Self {
        ColorString { color, string: string.into(), colorized: String::new() }
    }

    /// Paint the internal `string` using the configured `color` and store the
    /// result in `colorized`.
    ///
    /// # Examples
    /// ```
    /// use cli_utils::colors::{Color, ColorString};
    /// let mut s = ColorString::new(Color::Green, "ok");
    /// s.paint();
    /// assert!(s.colorized().contains("ok"));
    /// ```
    pub fn paint(&mut self) {
        match self.color {
            Color::Red => self.colorized = red(&self.string),
            Color::Green => self.colorized = green(&self.string),
            Color::Blue => self.colorized = blue(&self.string),
            Color::Bold => self.colorized = bold(&self.string),
        };
    }

    /// Reset the `colorized` representation to an unstyled version of the
    /// original string.
    pub fn reset(&mut self) {
        self.colorized = reset(&self.string);
    }

    /// Consume `self`, ensure the `colorized` value is generated, and return
    /// it as an owned `String`.
    ///
    /// This is convenient when you want the final colored string and do not
    /// need to keep the `ColorString` value afterwards.
    
    pub fn into_colorized(mut self) -> String {
        if self.colorized.is_empty() {
            self.paint();
        }
        self.colorized
    }

    /// Borrow the current cached colorized string.
    ///
    /// If `paint` has not been called, this will be an empty string slice.
    
    pub fn colorized(&self) -> &str {
        &self.colorized
    }
}
