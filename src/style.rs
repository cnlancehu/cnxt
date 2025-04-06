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

macro_rules! impl_binary_op_for_styles {
    (impl $trait_name:ident, $method:ident, $op:tt for Styles) => {
        impl $trait_name<Self> for Styles {
            type Output = Style;

            #[inline]
            fn $method(self, rhs: Self) -> Self::Output {
                Style(self.to_u8() $op rhs.to_u8())
            }
        }

        auto_impl_ref_binop_trait!(impl $trait_name, $method for Styles, Styles);

        impl $trait_name<Style> for Styles {
            type Output = Style;

            #[inline]
            fn $method(self, rhs: Style) -> Self::Output {
                Style(self.to_u8() $op rhs.0)
            }
        }

        auto_impl_ref_binop_trait!(impl $trait_name, $method for Styles, Style);
    };
}

macro_rules! impl_binary_op_for_style {
    (impl $trait_name:ident, $method:ident, $op:tt for Style) => {
        impl $trait_name<Self> for Style {
            type Output = Self;

            #[inline]
            fn $method(self, rhs: Self) -> Self::Output {
                Self(self.0 $op rhs.0)
            }
        }

        auto_impl_ref_binop_trait!(impl $trait_name, $method for Style, Style);

        impl $trait_name<Styles> for Style {
            type Output = Self;

            #[inline]
            fn $method(self, rhs: Styles) -> Self::Output {
                Self(self.0 $op rhs.to_u8())
            }
        }

        auto_impl_ref_binop_trait!(impl $trait_name, $method for Style, Styles);
    };
}

macro_rules! impl_style_method {
    ($name:ident, $style:ident) => {
        /// Enables the specified style attribute for this Style.
        ///
        /// Returns the modified Style for chaining.
        #[must_use]
        #[inline]
        pub fn $name(mut self) -> Self {
            self.add(Styles::$style);
            self
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

/// A combinatorial style representation for text formatting (bold, italic, etc.)
///
/// # Usage Examples
///
/// ## Creating Styles
///
/// - Default (no styles): `Style::default()`
/// - From individual style: `Style::from(Styles::Bold)`
/// - Builder pattern: `Style::default().bold().italic()`
/// - From multiple styles: `Style::from_iter([Styles::Bold, Styles::Italic])`
///
/// ## Combining Styles with Operators
///
/// ```rust
/// use cnxt::*;
///
/// // Combine styles with the | operator
/// let header_style = Style::from(Styles::Bold) | Styles::Underline;
///
/// // Remove styles with & and ! operators
/// let mut styles = Style::default().bold().italic().underline();
/// styles &= !Style::from(Styles::Italic); // Remove italic
///
/// assert!(styles.contains(Styles::Bold));
/// assert!(styles.contains(Styles::Underline));
/// assert!(!styles.contains(Styles::Italic));
/// ```
///
/// ## Adding and Removing Styles
///
/// ```rust
/// use cnxt::*;
///
/// let mut style = Style::default().bold();
/// style.add(Styles::Underline); // Add a style
/// style.remove(Styles::Bold); // Remove a style
///
/// assert!(!style.contains(Styles::Bold));
/// assert!(style.contains(Styles::Underline));
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Style(u8);

/// Individual style flags that can be applied to text.
///
/// # Bitwise Operations
///
/// - Combine styles: `Styles::Bold | Styles::Italic` → returns a `Style`
/// - Invert a style: `!Styles::Bold` → returns a `Style` with all styles except Bold
/// - Combine with existing Style: `existingStyle | Styles::Underline`
///
/// # Example
///
/// ```rust
/// use cnxt::*;
///
/// // Create a Style with multiple attributes
/// let style = Styles::Bold | Styles::Underline;
/// assert!(style.contains(Styles::Bold));
///
/// // Create a Style with everything but Bold
/// let not_bold = !Styles::Bold;
/// assert!(!not_bold.contains(Styles::Bold));
/// assert!(not_bold.contains(Styles::Underline));
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
    #[inline]
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

    #[inline]
    const fn to_u8(self) -> u8 {
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

        let res = STYLES
            .iter()
            .filter_map(|&(mask, value)| (u & mask != 0).then_some(value))
            .collect::<Vec<_>>();

        (!res.is_empty()).then_some(res)
    }
}

// Using our binary operation macros for Styles
impl_binary_op_for_styles!(impl BitAnd, bitand, & for Styles);
impl_binary_op_for_styles!(impl BitOr, bitor, | for Styles);
impl_binary_op_for_styles!(impl BitXor, bitxor, ^ for Styles);

impl Not for Styles {
    type Output = Style;

    #[inline]
    fn not(self) -> Self::Output {
        Style(!self.to_u8())
    }
}

impl Not for &Styles {
    type Output = Style;

    #[inline]
    fn not(self) -> Self::Output {
        Style(!self.to_u8())
    }
}

impl Style {
    /// Checks if this Style has a specific style flag enabled.
    ///
    /// ### Example
    ///
    /// ```rust
    /// let style = Style::default().bold().italic();
    /// assert!(style.contains(Styles::Bold));
    /// assert!(!style.contains(Styles::Underline));
    /// ```
    #[must_use]
    #[inline]
    pub fn contains(self, style: Styles) -> bool {
        let s = style.to_u8();
        self.0 & s == s
    }

    #[inline]
    pub(crate) fn to_str(self) -> String {
        match Styles::from_u8(self.0) {
            Some(styles) => styles
                .iter()
                .map(|s| s.to_str())
                .collect::<Vec<&str>>()
                .join(";"),
            None => String::new(),
        }
    }

    /// Adds a style flag to this Style.
    ///
    /// ### Example
    ///
    /// ```rust
    /// let mut style = Style::default();
    /// style.add(Styles::Bold);
    /// assert!(style.contains(Styles::Bold));
    /// ```
    #[inline]
    pub fn add(&mut self, style: Styles) {
        self.0 |= style.to_u8();
    }

    /// Removes a style flag from this Style.
    ///
    /// ### Example
    ///
    /// ```rust
    /// let mut style = Style::default().bold().italic();
    /// style.remove(Styles::Bold);
    /// assert!(!style.contains(Styles::Bold));
    /// assert!(style.contains(Styles::Italic));
    /// ```
    #[inline]
    pub fn remove(&mut self, style: Styles) {
        self.0 &= !style.to_u8();
    }

    // Using our style method macro for all style methods
    impl_style_method!(bold, Bold);
    impl_style_method!(dimmed, Dimmed);
    impl_style_method!(underline, Underline);
    impl_style_method!(reversed, Reversed);
    impl_style_method!(italic, Italic);
    impl_style_method!(blink, Blink);
    impl_style_method!(hidden, Hidden);
    impl_style_method!(strikethrough, Strikethrough);
}

// Using our binary operation macros for Style
impl_binary_op_for_style!(impl BitAnd, bitand, & for Style);
impl_binary_op_for_style!(impl BitOr, bitor, | for Style);
impl_binary_op_for_style!(impl BitXor, bitxor, ^ for Style);

impl Not for Style {
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl Not for &Style {
    type Output = Style;

    #[inline]
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
    #[inline]
    fn from(value: Styles) -> Self {
        Self(value.to_u8())
    }
}

impl From<&Styles> for Style {
    #[inline]
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
