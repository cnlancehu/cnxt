use core::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not,
};

macro_rules! auto_impl_ref_binop_trait {
    (impl $trait_name:ident, $method:ident for $t:ty, $u:ty) => {
        impl $trait_name<&$u> for $t {
            type Output = <$t as $trait_name<$t>>::Output;

            #[inline]
            fn $method(self, rhs: &$u) -> Self::Output {
                $trait_name::$method(self, *rhs)
            }
        }

        impl $trait_name<$u> for &$t {
            type Output = <$t as $trait_name<$t>>::Output;

            #[inline]
            fn $method(self, rhs: $u) -> Self::Output {
                $trait_name::$method(*self, rhs)
            }
        }

        impl $trait_name<&$u> for &$t {
            type Output = <$t as $trait_name<$t>>::Output;

            #[inline]
            fn $method(self, rhs: &$u) -> Self::Output {
                $trait_name::$method(*self, *rhs)
            }
        }
    };
}

macro_rules! impl_assign_op_trait {
    (
        $trait:ident, $method:ident for $t:ty, $u:ty, using $used_trait:ident::$used_method:ident
    ) => {
        impl $trait<$u> for $t {
            #[inline]
            fn $method(&mut self, other: $u) {
                *self = $used_trait::$used_method(*self, other);
            }
        }

        impl $trait<&$u> for $t {
            #[inline]
            fn $method(&mut self, other: &$u) {
                *self = $used_trait::$used_method(*self, *other);
            }
        }
    };
}

const CLEARV: u8 = 0b0000_0000;
const BOLD: u8 = 0b0000_0001;
const UNDERLINE: u8 = 0b0000_0010;
const REVERSED: u8 = 0b0000_0100;
const ITALIC: u8 = 0b0000_1000;
const BLINK: u8 = 0b0001_0000;
const HIDDEN: u8 = 0b0010_0000;
const DIMMED: u8 = 0b0100_0000;
const STRIKETHROUGH: u8 = 0b1000_0000;

static STYLES: [(u8, Styles); 8] = [
    (BOLD, Styles::Bold),
    (DIMMED, Styles::Dimmed),
    (UNDERLINE, Styles::Underline),
    (REVERSED, Styles::Reversed),
    (ITALIC, Styles::Italic),
    (BLINK, Styles::Blink),
    (HIDDEN, Styles::Hidden),
    (STRIKETHROUGH, Styles::Strikethrough),
];

pub static CLEAR: Style = Style(CLEARV);

/// A combinatorial style such as bold, italics, dimmed, etc.
///
/// ## Creation
///
/// `Style::default()` returns a `Style` with no style switches
/// activated and is the default method of creating a plain `Style`.
///
/// ## `Style` from a set of `Styles`s / `Styles` iterator
///
/// `Style` implements `FromIter<Styles>` which means that it is
/// possible to do the following:
///
/// ```rust
/// # use cnxt::*;
/// let style =
///     Style::from_iter([Styles::Bold, Styles::Italic, Styles::Strikethrough]);
/// for styles in [Styles::Bold, Styles::Italic, Styles::Strikethrough] {
///     assert!(style.contains(styles));
/// }
/// ```
///
/// As you can see, this is a good thing to keep in mind, although for
/// most cases, where you're not setting styles dynamically and are
/// simply creating a pre-defined set of styles, using [`Default`] and
/// then using the builder-style methods is likely prettier.
///
/// ```rust
/// # use cnxt::*;
/// let many_styles = Style::default().bold().underline().italic().blink();
/// ```
///
/// ## Implementation of logical bitwise operators
///
/// `Style` implements bitwise logical operations that operate on
/// the held style switches collectively. By far the most common
/// and useful is the bitwise 'or' operator `|` which combines two
/// styles, merging their combined styles into one. Example:
///
/// ```rust
/// # use cnxt::*;
/// let only_bold = Style::from(Styles::Bold);
/// // This line is actually an example of `Styles`'s bitwise logic impls but still.
/// let underline_and_italic = Styles::Underline | Styles::Italic;
/// let all_three = only_bold | underline_and_italic;
///
/// assert!(all_three.contains(Styles::Bold)
///     && all_three.contains(Styles::Underline)
///     && all_three.contains(Styles::Italic));
/// ```
///
/// This functionality also allows for easily turning off styles
/// of one `Styles` using another by combining the `&` and `!`
/// operators.
///
/// ```rust
/// # use cnxt::*;
/// let mut very_loud_style = Style::default()
///     .bold()
///     .underline()
///     .italic()
///     .strikethrough()
///     .hidden();
///
/// // Oops! Some of those should not be in there!
/// // This Style now has all styles _except_ the two we don't want
/// // (hidden and strikethough).
/// let remove_mask =
///     !Style::from_iter([Styles::Hidden, Styles::Strikethrough]);
/// very_loud_style &= remove_mask;
///
/// // `very_loud_style` no longer contains the undesired style
/// // switches...
/// assert!(
///     !very_loud_style.contains(Styles::Hidden)
///         && !very_loud_style.contains(Styles::Strikethrough)
/// );
/// // ...but it retains everything else!
/// assert!(very_loud_style.contains(Styles::Bold));
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Style(u8);

/// Enum containing all of the available style settings that can be
/// applied to a [`Styles`] and by extension, a colrized type.
///
/// ## Implementation of bitwise logical operators
///
/// The implementations of [`BitAnd`], [`BitOr`], [`BitXor`], and
/// [`Not`] are really extensions of [`Style`]'s implementations of
/// the same. [`BitOr`] is great for starting chains of `Styles`'s
/// for creating [`Style`]'s.
///
/// ```
/// # use cnxt::*;
/// let my_styles =
///     // BitOr<Styles> for Styles (Styles | Styles) = Style
///     Styles::Bold | Styles::Underline
///     // BitOr<Styles> for Style (Style | Styles) = Style
///     | Styles::Italic;
///
/// for s in [Styles::Bold, Styles::Underline, Styles::Italic] {
///     assert!(my_styles.contains(s));
/// }
/// ```
///
/// [`Not`] has far fewer use cases but can still find use in
/// turning a `Styles` into a [`Style`] with all styles activated
/// except that `Styles`.
///
/// ```
/// # use cnxt::*;
/// let everything_but_bold = !Styles::Bold;
///
/// assert!(everything_but_bold.contains(Styles::Underline));
/// assert!(everything_but_bold.contains(Styles::Strikethrough));
/// assert!(!everything_but_bold.contains(Styles::Bold));
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[allow(missing_docs)]
pub enum Styles {
    Clear,
    Bold,
    Dimmed,
    Underline,
    Reversed,
    Italic,
    Blink,
    Hidden,
    Strikethrough,
}

impl Styles {
    fn to_str<'a>(self) -> &'a str {
        match self {
            Self::Clear => "", // unreachable, but we don't want to panic
            Self::Bold => "1",
            Self::Dimmed => "2",
            Self::Italic => "3",
            Self::Underline => "4",
            Self::Blink => "5",
            Self::Reversed => "7",
            Self::Hidden => "8",
            Self::Strikethrough => "9",
        }
    }

    fn to_u8(self) -> u8 {
        match self {
            Self::Clear => CLEARV,
            Self::Bold => BOLD,
            Self::Dimmed => DIMMED,
            Self::Italic => ITALIC,
            Self::Underline => UNDERLINE,
            Self::Blink => BLINK,
            Self::Reversed => REVERSED,
            Self::Hidden => HIDDEN,
            Self::Strikethrough => STRIKETHROUGH,
        }
    }

    fn from_u8(u: u8) -> Option<Vec<Self>> {
        if u == CLEARV {
            return None;
        }

        let res: Vec<Self> = STYLES
            .iter()
            .filter(|&(mask, _)| (0 != (u & mask)))
            .map(|&(_, value)| value)
            .collect();
        if res.is_empty() { None } else { Some(res) }
    }
}

impl BitAnd<Self> for Styles {
    type Output = Style;

    fn bitand(self, rhs: Self) -> Self::Output {
        Style(self.to_u8() & rhs.to_u8())
    }
}

auto_impl_ref_binop_trait!(impl BitAnd, bitand for Styles, Styles);

impl BitAnd<Style> for Styles {
    type Output = Style;

    fn bitand(self, rhs: Style) -> Self::Output {
        Style(self.to_u8() & rhs.0)
    }
}

auto_impl_ref_binop_trait!(impl BitAnd, bitand for Styles, Style);

impl BitOr<Self> for Styles {
    type Output = Style;

    fn bitor(self, rhs: Self) -> Self::Output {
        Style(self.to_u8() | rhs.to_u8())
    }
}

auto_impl_ref_binop_trait!(impl BitOr, bitor for Styles, Styles);

impl BitOr<Style> for Styles {
    type Output = Style;

    fn bitor(self, rhs: Style) -> Self::Output {
        Style(self.to_u8() | rhs.0)
    }
}

auto_impl_ref_binop_trait!(impl BitOr, bitor for Styles, Style);

impl BitXor<Self> for Styles {
    type Output = Style;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Style(self.to_u8() ^ rhs.to_u8())
    }
}

auto_impl_ref_binop_trait!(impl BitXor, bitxor for Styles, Styles);

impl BitXor<Style> for Styles {
    type Output = Style;

    fn bitxor(self, rhs: Style) -> Self::Output {
        Style(self.to_u8() ^ rhs.0)
    }
}

auto_impl_ref_binop_trait!(impl BitXor, bitxor for Styles, Style);

impl Not for Styles {
    type Output = Style;

    fn not(self) -> Self::Output {
        Style(!self.to_u8())
    }
}

impl Not for &Styles {
    type Output = Style;

    fn not(self) -> Self::Output {
        Style(!self.to_u8())
    }
}

impl Style {
    /// Check if the current style has one of [`Styles`](Styles) switched on.
    ///
    /// ```rust
    /// # use cnxt::*;
    /// let colored = "".bold().italic();
    /// assert_eq!(colored.style.contains(Styles::Bold), true);
    /// assert_eq!(colored.style.contains(Styles::Italic), true);
    /// assert_eq!(colored.style.contains(Styles::Dimmed), false);
    /// ```
    #[must_use]
    pub fn contains(self, style: Styles) -> bool {
        let s = style.to_u8();
        self.0 & s == s
    }

    pub(crate) fn to_str(self) -> String {
        let styles = Styles::from_u8(self.0).unwrap_or_default();
        styles
            .iter()
            .map(|s| s.to_str())
            .collect::<Vec<&str>>()
            .join(";")
    }

    /// Adds the `two` style switch to this Style.
    ///
    /// ```rust
    /// # use cnxt::*;
    /// let cstr = "".red().bold();
    /// let mut style = cstr.style;
    /// style.add(Styles::Italic);
    /// let mut cstr2 = "".blue();
    /// cstr2.style = style;
    ///
    /// assert!(cstr2.style.contains(Styles::Bold));
    /// assert!(cstr2.style.contains(Styles::Italic));
    /// assert_eq!(cstr2.fgcolor, Some(Color::Blue));
    /// ```
    pub fn add(&mut self, two: Styles) {
        self.0 |= two.to_u8();
    }

    /// Turns off a style switch.
    ///
    /// ```rust
    /// use cnxt::*;
    /// let cstr = "".red().bold().italic();
    /// let mut style = cstr.style;
    /// style.remove(Styles::Italic);
    /// let mut cstr2 = "".blue();
    /// cstr2.style = style;
    /// assert!(cstr2.style.contains(Styles::Bold));
    /// assert!(!cstr2.style.contains(Styles::Italic));
    /// assert_eq!(cstr2.fgcolor, Some(Color::Blue));
    /// ```
    pub fn remove(&mut self, two: Styles) {
        self.0 &= !two.to_u8();
    }

    /// Makes this `Style` include Bold.
    #[must_use]
    pub fn bold(mut self) -> Self {
        self.add(Styles::Bold);
        self
    }

    /// Makes this `Style` include Dimmed.
    #[must_use]
    pub fn dimmed(mut self) -> Self {
        self.add(Styles::Dimmed);
        self
    }

    /// Makes this `Style` include Underline.
    #[must_use]
    pub fn underline(mut self) -> Self {
        self.add(Styles::Underline);
        self
    }

    /// Makes this `Style` include Reversed.
    #[must_use]
    pub fn reversed(mut self) -> Self {
        self.add(Styles::Reversed);
        self
    }

    /// Makes this `Style` include Italic.
    #[must_use]
    pub fn italic(mut self) -> Self {
        self.add(Styles::Italic);
        self
    }

    /// Makes this `Style` include Blink.
    #[must_use]
    pub fn blink(mut self) -> Self {
        self.add(Styles::Blink);
        self
    }

    /// Makes this `Style` include Hidden.
    #[must_use]
    pub fn hidden(mut self) -> Self {
        self.add(Styles::Hidden);
        self
    }

    /// Makes this `Style` include Strikethrough.
    #[must_use]
    pub fn strikethrough(mut self) -> Self {
        self.add(Styles::Strikethrough);
        self
    }
}

impl BitAnd<Self> for Style {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

auto_impl_ref_binop_trait!(impl BitAnd, bitand for Style, Style);

impl BitAnd<Styles> for Style {
    type Output = Self;

    fn bitand(self, rhs: Styles) -> Self::Output {
        Self(self.0 & rhs.to_u8())
    }
}

auto_impl_ref_binop_trait!(impl BitAnd, bitand for Style, Styles);

impl BitOr<Self> for Style {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

auto_impl_ref_binop_trait!(impl BitOr, bitor for Style, Style);

impl BitOr<Styles> for Style {
    type Output = Self;

    fn bitor(self, rhs: Styles) -> Self::Output {
        Self(self.0 | rhs.to_u8())
    }
}

auto_impl_ref_binop_trait!(impl BitOr, bitor for Style, Styles);

impl BitXor<Self> for Style {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

auto_impl_ref_binop_trait!(impl BitXor, bitxor for Style, Style);

impl BitXor<Styles> for Style {
    type Output = Self;

    fn bitxor(self, rhs: Styles) -> Self::Output {
        Self(self.0 ^ rhs.to_u8())
    }
}

auto_impl_ref_binop_trait!(impl BitXor, bitxor for Style, Styles);

impl Not for Style {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl Not for &Style {
    type Output = Style;

    fn not(self) -> Self::Output {
        Style(!self.0)
    }
}

impl_assign_op_trait!(BitAndAssign, bitand_assign for Style, Style, using BitAnd::bitand);

impl_assign_op_trait!(BitAndAssign, bitand_assign for Style, Styles, using BitAnd::bitand);

impl_assign_op_trait!(BitOrAssign, bitor_assign for Style, Style, using BitOr::bitor);

impl_assign_op_trait!(BitOrAssign, bitor_assign for Style, Styles, using BitOr::bitor);

impl_assign_op_trait!(BitXorAssign, bitxor_assign for Style, Style, using BitXor::bitxor);

impl_assign_op_trait!(BitXorAssign, bitxor_assign for Style, Styles, using BitXor::bitxor);

impl Default for Style {
    fn default() -> Self {
        CLEAR
    }
}

impl From<Styles> for Style {
    fn from(value: Styles) -> Self {
        Self(value.to_u8())
    }
}

impl From<&Styles> for Style {
    fn from(value: &Styles) -> Self {
        Self(value.to_u8())
    }
}

impl FromIterator<Styles> for Style {
    fn from_iter<T: IntoIterator<Item = Styles>>(iter: T) -> Self {
        let mut style = Self::default();
        for styles in iter {
            style.add(styles);
        }
        style
    }
}
