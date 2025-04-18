//! # Colored Next (CNXT)
//! [![crates.io](https://api.lance.fun/badge/cratesio/cnxt)](https://crates.io/crates/cnxt)
//!
//! An enhanced fork of [colored](https://github.com/colored-rs/colored) offering superior performance and terminal handling.
//!
//! Why CNXT?
//!
//! 1. **Optimized Performance**: Utilizes `Cow` for intelligent memory management [(Inspired by this PR)](https://github.com/colored-rs/colored/pull/135)
//! 2. **Modern Codebase**: Removed legacy code and streamlined implementation
//! 3. **Advanced Terminal Support**: Sophisticated terminal capability detection with automatic color downgrading
//!
//! ## Usage
//! Coloring your terminal made simple. You already know how to do it.
//!
//! ![usage](https://raw.githubusercontent.com/cnlancehu/cnxt/refs/heads/master/assets/usage.png)
//!
//! ### Essential Configuration
//!
//! 1. For **Windows targets**, add this to enable colors in **Windows CMD**:
//!     ```rust
//!     #[cfg(windows)]
//!     cnxt::control::set_virtual_terminal(true);
//!     ```
//!
//!     Comparison showing how Windows CMD displays colors before and after enabling virtual terminal.
//!
//!     ![comparison](https://raw.githubusercontent.com/cnlancehu/cnxt/refs/heads/master/assets/set_virtual_terminal_comparison.png)
//!
//! 2. CNXT dynamically detects terminal color support across three tiers:
//!     - `Ansi16` (16 colors)
//!     - `Ansi256` (256 colors)
//!     - `TrueColor`
//!      
//!     When using colors beyond your terminal's capabilities, CNXT automatically **downgrades** them to the maximum supported level.
//!
//!     Manual control options:
//!     ```rust
//!     use cnxt::control::{set_should_colorize, ShouldColorize};
//!     
//!     // Environment-based detection level (default)
//!     set_should_colorize(ShouldColorize::from_env());
//!
//!     // Explicit configuration
//!     set_should_colorize(ShouldColorize::YesWithTrueColor);  // Force truecolor
//!     set_should_colorize(ShouldColorize::YesWithAnsi256);    // Force 256-color
//!     set_should_colorize(ShouldColorize::No);                // Disable colors
//!     set_should_colorize(ShouldColorize::Yes);               // Enable colors with auto-detect level
//!
//!     // Manual color fallback
//!     use cnxt::Color;
//!
//!     let truecolor = Color::TrueColor { r: 166, g: 227, b: 161 };
//!     let ansi16 = truecolor.fallback_to_ansi16();
//!     let ansi256 = truecolor.fallback_to_ansi256();
//!     ```
//!
//! ### Features
//! 1. **terminal-detection** (Enabled by default):
//!
//!     Automatically detects terminal color support and downgrades colors accordingly.
//!
//!     Defaultly use TrueColor if disable this feature.
//!
//! 2. **conditional-coloring** :
//!    Provide helper functions to colorize strings based on conditions.
//!
//!     ```rust
//!     use cnxt::Colorize as _;
//!     
//!     println!("{}", "red".red_if(true)); // print red color
//!     println!("{}", "red".red_if(false)); // print no color
//!
//!     println!("{}", "green".green().red_if(false)); // print green color
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
/// [`Display`](fmt::Display) and [`ToString`] for those purposes
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
/// colored_text.input = "Blue".into();
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
    pub style: Style,
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

#[cfg(feature = "conditional-coloring")]
macro_rules! impl_basic_fg_colors_conditional {
        ($(($method:ident, $color:ident)),*) => {
            $(
                /// If `cond` is true, color the string with the specified color.
                ///
                /// Otherwise, return the string unchanged.
                fn $method(self, cond: bool) -> ColoredString<'a>
                where
                    Self: Sized,
                {
                    if cond {
                        self.color(Color::$color)
                    } else {
                        self.do_nothing()
                    }
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

#[cfg(feature = "conditional-coloring")]
macro_rules! impl_basic_bg_colors_conditional {
        ($(($method:ident, $color:ident)),*) => {
            $(
                /// If `cond` is true, color the string with the specified background color.
                ///
                /// Otherwise, return the string unchanged.
                fn $method(self, cond: bool) -> ColoredString<'a>
                where
                    Self: Sized,
                {
                    if cond {
                        self.on_color(Color::$color)
                    } else {
                        self.do_nothing()
                    }
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
    #[cfg(feature = "conditional-coloring")]
    fn do_nothing(self) -> ColoredString<'a>
    where
        Self: Sized,
    {
        self.do_nothing()
    }

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

    #[cfg(feature = "conditional-coloring")]
    impl_basic_fg_colors_conditional! {
        (black_if, Black),
        (red_if, Red),
        (green_if, Green),
        (yellow_if, Yellow),
        (blue_if, Blue),
        (magenta_if, Magenta),
        (purple_if, Magenta), // Alias for magenta
        (cyan_if, Cyan),
        (white_if, White),
        (bright_black_if, BrightBlack),
        (bright_red_if, BrightRed),
        (bright_green_if, BrightGreen),
        (bright_yellow_if, BrightYellow),
        (bright_blue_if, BrightBlue),
        (bright_magenta_if, BrightMagenta),
        (bright_purple_if, BrightMagenta), // Alias for bright magenta
        (bright_cyan_if, BrightCyan),
        (bright_white_if, BrightWhite)
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

    #[cfg(feature = "conditional-coloring")]
    impl_basic_bg_colors_conditional! {
        (on_black_if, Black),
        (on_red_if, Red),
        (on_green_if, Green),
        (on_yellow_if, Yellow),
        (on_blue_if, Blue),
        (on_magenta_if, Magenta),
        (on_purple_if, Magenta), // Alias for magenta
        (on_cyan_if, Cyan),
        (on_white_if, White),
        (on_bright_black_if, BrightBlack),
        (on_bright_red_if, BrightRed),
        (on_bright_green_if, BrightGreen),
        (on_bright_yellow_if, BrightYellow),
        (on_bright_blue_if, BrightBlue),
        (on_bright_magenta_if, BrightMagenta),
        (on_bright_purple_if, BrightMagenta), // Alias for bright magenta
        (on_bright_cyan_if, BrightCyan),
        (on_bright_white_if, BrightWhite)
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
        if !self.input.contains(reset) {
            return Cow::Borrowed(self.input.as_ref());
        }

        let style = self.compute_style();
        let matches: Vec<_> = self.input.match_indices(reset).collect();

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
    #[cfg(feature = "conditional-coloring")]
    fn do_nothing(self) -> ColoredString<'a> {
        self
    }

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
        (bold, Styles::Bold),
        (dimmed, Styles::Dimmed),
        (italic, Styles::Italic),
        (underline, Styles::Underline),
        (blink, Styles::Blink),
        (reversed, Styles::Reversed),
        (hidden, Styles::Hidden),
        (strikethrough, Styles::Strikethrough)
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
    #[cfg(feature = "conditional-coloring")]
    fn do_nothing(self) -> ColoredString<'a> {
        ColoredString::from(self)
    }

    fn color<S: Into<Color>>(self, color: S) -> ColoredString<'a> {
        ColoredString {
            fgcolor: Some(color.into()),
            input: Cow::Borrowed(self),
            ..ColoredString::default()
        }
    }

    fn on_color<S: Into<Color>>(self, color: S) -> ColoredString<'a> {
        ColoredString {
            bgcolor: Some(color.into()),
            input: Cow::Borrowed(self),
            ..ColoredString::default()
        }
    }

    fn clear(self) -> ColoredString<'a> {
        ColoredString {
            input: Cow::Borrowed(self),
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
