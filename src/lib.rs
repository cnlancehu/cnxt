//! # Colored Next (CNXT)
//! [![badge](https://api.lance.fun/badge/cratesio/cnxt)](https://crates.io/crates/cnxt)
//!
//! A fork of [colored](https://github.com/colored-rs/colored) which introduces better functionalities.
//!
//! Why CNXT?
//!
//! 1. **Enhanced Performance**: Uses `Cow` to minimize allocations [(Inspired by this PR)](https://github.com/colored-rs/colored/pull/135)
//! 2. **Streamlined Codebase**: Removed outdated and redundant code
//! 3. **Superior Terminal Support**: Improved detection and handling of terminal capabilities
//!
//! ## Usage
//! Coloring your terminal made simple. You already know how to do it.
//!
//! ![usage](https://github.com/cnlancehu/cnxt/blob/master/assets/usage.png?raw=true)
//!
//! Small tips
//!
//! 1. For **Windows targets**, add this to enable colors in **Windows CMD**:
//!     ```rust
//!     #[cfg(windows)]
//!     cnxt::control::set_virtual_terminal(true);
//!     ```
//!     
//!     Comparison of colors with virtual terminal disabled vs enabled.
//!
//!     ![comparison](https://github.com/cnlancehu/cnxt/blob/master/assets/set_virtual_terminal_comparison.png?raw=true)
//!
//! 2. CNXT automatically detects **terminal color support** across **3 levels**:
//!
//!     - `Ansi16`
//!     - `Ansi256`
//!     - `TrueColor`
//!      
//!     When using colors beyond your terminal's capabilities, CNXT automatically downgrades them to the maximum supported level.
//!     ```rust
//!     use cnxt::control::{set_should_colorize, ShouldColorize};
//!
//!     // By default, the support level is detected from environment
//!     ShouldColorize::from_env()
//!
//!     // You can explicitly set the support level:
//!     set_should_colorize(ShouldColorize::YesWithTrueColor);  // Enable colorization with true color support
//!     set_should_colorize(ShouldColorize::YesWithAnsi256);    // Enable colorization with 256 color support
//!
//!     // Simple on/off control:
//!     set_should_colorize(ShouldColorize::No);    // Disable colorization
//!     set_should_colorize(ShouldColorize::Yes);   // Enable colorization
//!
//!     // Reset to environment-based detection:
//!     set_should_colorize(ShouldColorize::from_env());
//!     ```
//!     And for manual color fallback control:
//!
//!     ```rust
//!     use cnxt::Color;
//!     
//!     let color = Color::TrueColor {
//!         r: 166,
//!         g: 227,
//!         b: 161,
//!     };
//!     let ansi16_color = color.fallback_to_ansi16();
//!     # or
//!     let ansi256_color = color.fallback_to_ansi256();
//!     ```

mod color;
pub mod control;
mod style;

pub use self::customcolors::CustomColor;

/// Custom colors support.
pub mod customcolors;

use std::{
    borrow::Cow,
    fmt,
    ops::{Deref, DerefMut},
};

pub use color::*;
pub use style::{Style, Styles};

/// A string that may have color and/or style applied to it.
///
/// Commonly created via calling the methods of [`Colorize`] on a &str.
/// All methods of [`Colorize`] either create a new `ColoredString` from
/// the type called on or modify a callee `ColoredString`. See
/// [`Colorize`] for more.
///
/// The primary usage of `ColoredString`'s is as a way to take text,
/// apply colors and miscillaneous styling to it (such as bold or
/// underline), and then use it to create formatted strings that print
/// to the console with the special styling applied.
///
/// ## Usage
///
/// As stated, `ColoredString`'s, once created, can be printed to the
/// console with their colors and style or turned into a string
/// containing special console codes that has the same effect.
/// This is made easy via `ColoredString`'s implementations of
/// [`Display`](std::fmt::Display) and [`ToString`] for those purposes
/// respectively.
///
/// Printing a `ColoredString` with its style is as easy as:
///
/// ```
/// # use cnxt::*;
/// let cstring: ColoredString = "Bold and Red!".bold().red();
/// println!("{}", cstring);
/// ```
///
/// ## Manipulating the coloring/style of a `ColoredString`
///
/// Getting or changing the foreground color, background color, and or
/// style of a `cnxtString` is as easy as manually reading / modifying
/// the fields of `ColoredString`.
///
/// ```
/// # use cnxt::*;
/// let mut red_text = "Red".red();
/// // Changing color using re-assignment and [`Colorize`]:
/// red_text = red_text.blue();
/// // Manipulating fields of `ColoredString` in-place:
/// red_text.fgcolor = Some(Color::Blue);
///
/// let styled_text1 = "Bold".bold();
/// let styled_text2 = "Italic".italic();
/// let mut styled_text3 = ColoredString::from("Bold and Italic");
/// styled_text3.style = styled_text1.style | styled_text2.style;
/// ```
///
/// ## Modifying the text of a `ColoredString`
///
/// Modifying the text is as easy as modifying the `input` field of
/// `ColoredString`...
///
/// ```
/// # use cnxt::*;
/// let mut colored_text = "Magenta".magenta();
/// colored_text = colored_text.blue();
/// colored_text.input = "Blue".to_string();
/// // Note: The above is inefficient and `colored_text.input.replace_range(.., "Blue")` would
/// // be more proper. This is just for example.
///
/// assert_eq!(&*colored_text, "Blue");
/// ```
///
/// Notice how this process preserves the coloring and style.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct ColoredString<'a> {
    /// The plain text that will have color and style applied to it.
    pub input: Cow<'a, str>,
    /// The color of the text as it will be printed.
    pub fgcolor: Option<Color>,
    /// The background color (if any). None means that the text will be printed
    /// without a special background.
    pub bgcolor: Option<Color>,
    /// Any special styling to be applied to the text (see Styles for a list of
    /// available options).
    pub style: style::Style,
}

// Define macros to generate color methods
macro_rules! impl_basic_fg_colors {
        ($(($method:ident, $color:ident)),*) => {
            $(
                fn $method(self) -> ColoredString<'a>
                where
                    Self: Sized,
                {
                    self.color(Color::$color)
                }
            )*
        }
    }

macro_rules! impl_basic_bg_colors {
        ($(($method:ident, $color:ident)),*) => {
            $(
                fn $method(self) -> ColoredString<'a>
                where
                    Self: Sized,
                {
                    self.on_color(Color::$color)
                }
            )*
        }
    }

/// The trait that enables something to be given color.
///
/// You can use `colored` effectively simply by importing this trait
/// and then using its methods on `String` and `&str`.
#[allow(missing_docs)]
pub trait Colorize<'a> {
    // Generate standard foreground colors
    impl_basic_fg_colors! {
        (black, Black),
        (red, Red),
        (green, Green),
        (yellow, Yellow),
        (blue, Blue),
        (magenta, Magenta),
        (purple, Magenta), // Alias for magenta
        (cyan, Cyan),
        (white, White),
        (bright_black, BrightBlack),
        (bright_red, BrightRed),
        (bright_green, BrightGreen),
        (bright_yellow, BrightYellow),
        (bright_blue, BrightBlue),
        (bright_magenta, BrightMagenta),
        (bright_purple, BrightMagenta), // Alias for bright magenta
        (bright_cyan, BrightCyan),
        (bright_white, BrightWhite)
    }
    fn ansi256color(self, idx: u8) -> ColoredString<'a>
    where
        Self: Sized,
    {
        self.color(Color::Ansi256 { idx })
    }
    fn truecolor(self, r: u8, g: u8, b: u8) -> ColoredString<'a>
    where
        Self: Sized,
    {
        self.color(Color::TrueColor { r, g, b })
    }
    /// The following `#` prefix is optional.
    ///
    /// This function will **panic** if the hex string is invalid.
    ///
    /// Use [`Self::try_hexcolor`] for a non-panicking alternative.
    fn hexcolor<S>(self, hex: S) -> ColoredString<'a>
    where
        Self: Sized,
        S: AsRef<str>,
    {
        self.color(parse_hex(hex.as_ref()).unwrap())
    }
    /// The following `#` prefix is optional.
    ///
    /// This function will return `None` if the hex string is invalid
    fn try_hexcolor<S>(self, hex: S) -> Option<ColoredString<'a>>
    where
        Self: Sized,
        S: AsRef<str>,
    {
        parse_hex(hex.as_ref()).map(|color| self.color(color))
    }
    fn custom_color<T>(self, color: T) -> ColoredString<'a>
    where
        Self: Sized,
        T: Into<CustomColor>,
    {
        let color = color.into();

        self.color(Color::TrueColor {
            r: color.r,
            g: color.g,
            b: color.b,
        })
    }
    fn color<S: Into<Color>>(self, color: S) -> ColoredString<'a>;

    // Generate background colors
    impl_basic_bg_colors! {
        (on_black, Black),
        (on_red, Red),
        (on_green, Green),
        (on_yellow, Yellow),
        (on_blue, Blue),
        (on_magenta, Magenta),
        (on_purple, Magenta), // Alias for magenta
        (on_cyan, Cyan),
        (on_white, White),
        (on_bright_black, BrightBlack),
        (on_bright_red, BrightRed),
        (on_bright_green, BrightGreen),
        (on_bright_yellow, BrightYellow),
        (on_bright_blue, BrightBlue),
        (on_bright_magenta, BrightMagenta),
        (on_bright_purple, BrightMagenta), // Alias for bright magenta
        (on_bright_cyan, BrightCyan),
        (on_bright_white, BrightWhite)
    }
    fn on_ansi256color(self, idx: u8) -> ColoredString<'a>
    where
        Self: Sized,
    {
        self.on_color(Color::Ansi256 { idx })
    }
    fn on_truecolor(self, r: u8, g: u8, b: u8) -> ColoredString<'a>
    where
        Self: Sized,
    {
        self.on_color(Color::TrueColor { r, g, b })
    }
    /// The following `#` prefix is optional.
    ///
    /// This function will **panic** if the hex string is invalid.
    ///
    /// Use [`Self::try_on_hexcolor`] for a non-panicking alternative.
    fn on_hexcolor<S>(self, hex: S) -> ColoredString<'a>
    where
        Self: Sized,
        S: AsRef<str>,
    {
        self.on_color(parse_hex(hex.as_ref()).unwrap())
    }
    /// The following `#` prefix is optional.
    ///
    /// This function will return `None` if the hex string is invalid
    fn try_on_hexcolor<S>(self, hex: S) -> Option<ColoredString<'a>>
    where
        Self: Sized,
        S: AsRef<str>,
    {
        parse_hex(hex.as_ref()).map(|color| self.on_color(color))
    }
    fn on_custom_color<T>(self, color: T) -> ColoredString<'a>
    where
        Self: Sized,
        T: Into<CustomColor>,
    {
        let color = color.into();

        self.on_color(Color::TrueColor {
            r: color.r,
            g: color.g,
            b: color.b,
        })
    }
    fn on_color<S: Into<Color>>(self, color: S) -> ColoredString<'a>;

    // Styles
    fn clear(self) -> ColoredString<'a>;
    fn normal(self) -> ColoredString<'a>;
    fn bold(self) -> ColoredString<'a>;
    fn dimmed(self) -> ColoredString<'a>;
    fn italic(self) -> ColoredString<'a>;
    fn underline(self) -> ColoredString<'a>;
    fn blink(self) -> ColoredString<'a>;
    fn reversed(self) -> ColoredString<'a>;
    fn hidden(self) -> ColoredString<'a>;
    fn strikethrough(self) -> ColoredString<'a>;
}

impl ColoredString<'_> {
    /// Clears foreground coloring on this `ColoredString`, meaning that it
    /// will be printed with the default terminal text color.
    pub fn clear_fgcolor(&mut self) {
        self.fgcolor = None;
    }

    /// Gets rid of this `ColoredString`'s background.
    pub fn clear_bgcolor(&mut self) {
        self.bgcolor = None;
    }

    /// Clears any special styling and sets it back to the default (plain,
    /// maybe colored, text).
    pub fn clear_style(&mut self) {
        self.style = Style::default();
    }

    /// Checks if the colored string has no color or styling.
    ///
    /// ```rust
    /// # use cnxt::*;
    /// let cstr = "".red();
    /// assert_eq!(cstr.is_plain(), false);
    /// let cstr = cstr.clear();
    /// assert_eq!(cstr.is_plain(), true);
    /// ```
    #[must_use]
    pub fn is_plain(&self) -> bool {
        self.bgcolor.is_none()
            && self.fgcolor.is_none()
            && self.style == style::CLEAR
    }

    fn has_colors() -> bool {
        control::get_current_color_level() != control::ColorLevel::None
    }

    fn compute_style(&self) -> String {
        if !Self::has_colors() || self.is_plain() {
            return String::new();
        }

        let mut res = String::from("\x1B[");
        let mut has_wrote = if self.style == style::CLEAR {
            false
        } else {
            res.push_str(&self.style.to_str());
            true
        };

        if let Some(ref bgcolor) = self.bgcolor {
            if has_wrote {
                res.push(';');
            }

            res.push_str(&bgcolor.to_bg_str());
            has_wrote = true;
        }

        if let Some(ref fgcolor) = self.fgcolor {
            if has_wrote {
                res.push(';');
            }

            res.push_str(&fgcolor.to_fg_str());
        }

        res.push('m');
        res
    }

    fn escape_inner_reset_sequences(&self) -> Cow<str> {
        if !Self::has_colors() || self.is_plain() {
            return Cow::Borrowed(self.input.as_ref());
        }

        let reset = "\x1B[0m";
        let style = self.compute_style();
        let matches: Vec<_> = self.input.match_indices(reset).collect();

        if matches.is_empty() {
            return Cow::Borrowed(self.input.as_ref());
        }

        let additional_space = matches.len() * style.len();
        let mut result =
            String::with_capacity(self.input.len() + additional_space);

        let mut last_end = 0;
        for (idx, _) in matches {
            result.push_str(&self.input[last_end..idx]);
            result.push_str(reset);
            result.push_str(&style);

            last_end = idx + reset.len();
        }

        if last_end < self.input.len() {
            result.push_str(&self.input[last_end..]);
        }

        Cow::Owned(result)
    }
}

impl Deref for ColoredString<'_> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.input.as_ref()
    }
}

impl DerefMut for ColoredString<'_> {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        self.input.to_mut()
    }
}

impl<'a, T: Into<Cow<'a, str>>> From<T> for ColoredString<'a> {
    fn from(s: T) -> Self {
        ColoredString {
            input: s.into(),
            ..ColoredString::default()
        }
    }
}

macro_rules! impl_coloredstring_style_methods {
    ($(($method:ident, $style:expr)),*) => {
        $(
            fn $method(mut self) -> ColoredString<'a> {
                self.style.add($style);
                self
            }
        )*
    }
}

impl<'a> Colorize<'a> for ColoredString<'a> {
    fn color<S: Into<Color>>(mut self, color: S) -> ColoredString<'a> {
        self.fgcolor = Some(color.into());
        self
    }
    fn on_color<S: Into<Color>>(mut self, color: S) -> ColoredString<'a> {
        self.bgcolor = Some(color.into());
        self
    }

    fn clear(self) -> ColoredString<'a> {
        Self {
            input: self.input,
            ..Self::default()
        }
    }
    fn normal(self) -> ColoredString<'a> {
        self.clear()
    }

    impl_coloredstring_style_methods! {
        (bold, style::Styles::Bold),
        (dimmed, style::Styles::Dimmed),
        (italic, style::Styles::Italic),
        (underline, style::Styles::Underline),
        (blink, style::Styles::Blink),
        (reversed, style::Styles::Reversed),
        (hidden, style::Styles::Hidden),
        (strikethrough, style::Styles::Strikethrough)
    }
}

macro_rules! impl_str_style_methods {
    ($(($method:ident)),*) => {
        $(
            fn $method(self) -> ColoredString<'a> {
                ColoredString::from(self).$method()
            }
        )*
    }
}

impl<'a> Colorize<'a> for &'a str {
    fn color<S: Into<Color>>(self, color: S) -> ColoredString<'a> {
        ColoredString {
            fgcolor: Some(color.into()),
            input: Cow::Owned(String::from(self)),
            ..ColoredString::default()
        }
    }

    fn on_color<S: Into<Color>>(self, color: S) -> ColoredString<'a> {
        ColoredString {
            bgcolor: Some(color.into()),
            input: Cow::Owned(String::from(self)),
            ..ColoredString::default()
        }
    }

    fn clear(self) -> ColoredString<'a> {
        ColoredString {
            input: Cow::Owned(String::from(self)),
            style: style::CLEAR,
            ..ColoredString::default()
        }
    }
    fn normal(self) -> ColoredString<'a> {
        self.clear()
    }

    impl_str_style_methods! {
        (bold),
        (dimmed),
        (italic),
        (underline),
        (blink),
        (reversed),
        (hidden),
        (strikethrough)
    }
}

impl fmt::Display for ColoredString<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !Self::has_colors() || self.is_plain() {
            return write!(f, "{}", self.input);
        }

        let escaped_input = self.escape_inner_reset_sequences();

        f.write_str(&self.compute_style())?;
        write!(f, "{}", escaped_input)?;
        f.write_str("\x1B[0m")?;
        Ok(())
    }
}

fn parse_hex(s: &str) -> Option<Color> {
    // Remove leading # if present
    let s = s.strip_prefix('#').unwrap_or(s);

    match s.len() {
        6 => {
            let r = u8::from_str_radix(&s[0..2], 16).ok()?;
            let g = u8::from_str_radix(&s[2..4], 16).ok()?;
            let b = u8::from_str_radix(&s[4..6], 16).ok()?;
            Some(Color::TrueColor { r, g, b })
        }
        3 => {
            // Convert hex shorthand (e.g. "f09") to full form by duplicating each digit
            let r = u8::from_str_radix(&s[0..1], 16).ok()?;
            let g = u8::from_str_radix(&s[1..2], 16).ok()?;
            let b = u8::from_str_radix(&s[2..3], 16).ok()?;
            Some(Color::TrueColor {
                r: r * 17, // Same as (r << 4) | r but more readable
                g: g * 17,
                b: b * 17,
            })
        }
        _ => None,
    }
}
