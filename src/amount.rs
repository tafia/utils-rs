//! Fractal Global Credits amount
//!
//! This module holds the `Amount` type and the `AmountParseError`. It will eventually hold `MAX`
//! and `MIN` values for `Amount`s when constant expressions are implemented in the compiler in the
//! stable chanel.
//!
//! The maximum and minimum amount values can in any case be known by using `max_value()` and
//! `min_value()` functions in the `Amount` type:
//!
//! ```
//! use std::u64;
//! use fractal_utils::Amount;
//!
//! let max_value = Amount::max_value();
//! let min_value = Amount::min_value();
//!
//! assert_eq!(max_value, Amount::from_repr(u64::MAX));
//! assert_eq!(min_value, Amount::from_repr(u64::MIN));
//! ```

#![allow(trivial_numeric_casts)]

use std::{fmt, str, u64};
use std::error::Error;
use std::ops::{Add, AddAssign, Sub, SubAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign};
use std::num::ParseIntError;

use super::CURRENCY_SYMBOL;

/// Fractal Global Credits amount
///
/// This struct can be used the same way as any other number. An `Amount` can be added or
/// substracted to another `Amount`, and it can be divided and multiplied by an integer. All
/// operations that are defined in the `Amount` scope and that are exact can be used directly as
/// usual integer / float point operations.
///
/// No negative amounts can exist, since an `Amount` is unsigned, sothe negation operator '-',
/// then, has no use with an `Amount`.
///
/// Its internal representation is a 64 bit unsigned number, that is displayed as a fixed point,
/// number of factor 1/1,000. This means that an internal representation of `1,000` would be an
/// external amount of `1`. The internal representation shouldn't be used except when serializing
/// and deserializing the data, since this type is sent in *JSON* as its internal `u64`.
///
/// The use is the following:
///
/// ```
/// use fractal_utils::Amount;
///
/// let amount = Amount::from_repr(1_654); // 1.654
/// let ten = Amount::from_repr(10_000); // 10
/// let add_ten = amount + ten;
/// assert_eq!(add_ten, Amount::from_repr(11_654)); // 11.654
/// ```
///
/// They can be divided and multiplied by any other unsigned integer:
///
/// ```
/// # use fractal_utils::Amount;
/// #
/// let mut amount = Amount::from_repr(7_000); // 7
/// amount *= 10u32;
/// assert_eq!(amount, Amount::from_repr(70_000)); // 70
///
/// amount = amount / 30u16;
/// assert_eq!(amount, Amount::from_repr(2_333)); // 2.333
///
/// amount %= 1u8;
/// assert_eq!(amount, Amount::from_repr(333)); // 0.333
/// ```
///
/// Amounts can easily be displayed using the `Display` trait as any other number:
///
/// ```
/// # use fractal_utils::Amount;
/// #
/// let amount = Amount::from_repr(56_000);
/// assert_eq!(format!("{}", amount), "56");
/// assert_eq!(format!("{:.2}", amount), "56.00");
/// assert_eq!(format!("{:.5}", amount), "56.00000");
/// assert_eq!(format!("{:05.1}", amount), "056.0");
///
/// // And with rounding:
/// let amount = Amount::from_repr(56); // 0.056
/// assert_eq!(format!("{:.2}", amount), "0.06");
/// ```
#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Amount {
    value: u64,
}

impl Amount {
    /// Creates a new amount from its internal representation.
    pub fn from_repr(value: u64) -> Amount {
        Amount { value: value }
    }

    /// Gets the internal representation of the amount.
    pub fn get_repr(&self) -> u64 {
        self.value
    }

    /// Returns the smallest value that can be represented as a currency amount.
    pub fn min_value() -> Amount {
        Amount { value: u64::MIN }
    }

    /// Returns the largest value that can be represented as a currency amount.
    pub fn max_value() -> Amount {
        Amount { value: u64::MAX }
    }
}

impl fmt::Display for Amount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let units = self.value / 1_000;
        let decimal_repr = self.value % 1_000;
        let result = match f.precision() {
            None => {
                if decimal_repr == 0 {
                    format!("{}", units)
                } else if decimal_repr % 100 == 0 {
                    format!("{}.{:01}", units, decimal_repr / 100)
                } else if decimal_repr % 10 == 0 {
                    format!("{}.{:02}", units, decimal_repr / 10)
                } else {
                    format!("{}.{:03}", units, decimal_repr)
                }
            }
            Some(0) => {
                format!("{}",
                        if decimal_repr >= 500 {
                            units + 1
                        } else {
                            units
                        })
            }
            Some(1) => {
                format!("{}.{:01}",
                        units,
                        if decimal_repr % 100 >= 50 {
                            decimal_repr / 100 + 1
                        } else {
                            decimal_repr / 100
                        })
            }
            Some(2) => {
                format!("{}.{:02}",
                        units,
                        if decimal_repr % 10 >= 5 {
                            decimal_repr / 10 + 1
                        } else {
                            decimal_repr / 10
                        })
            }
            Some(p) => {
                let mut string = format!("{}.{:03}", units, decimal_repr);
                for _ in 3..p {
                    string.push('0');
                }
                string
            }
        };

        match f.width() {
            None => write!(f, "{}", result),
            Some(w) => {
                if w < result.len() {
                    write!(f, "{}", result)
                } else {
                    let mut pad = String::new();
                    for _ in result.len()..w {
                        pad.push('0');
                    }
                    write!(f, "{}{}", pad, result)
                }
            }
        }
    }
}

/// Amount parsing error.
///
/// This struct represents an amount parsing error. It explains the exact error that lead to the
/// parsing error, and implements common `Error` and `Display` traits.
#[derive(Debug)]
pub struct AmountParseError {
    description: String,
    cause: Option<ParseIntError>,
}

impl fmt::Display for AmountParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl Error for AmountParseError {
    fn description(&self) -> &str {
        &self.description
    }

    fn cause(&self) -> Option<&Error> {
        match self.cause.as_ref() {
            Some(c) => Some(c),
            None => None,
        }
    }
}

impl fmt::Debug for Amount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "Amount {{ {:?} }} ({} {})",
               self.value,
               CURRENCY_SYMBOL,
               self)
    }
}

macro_rules! impl_ops_int {
    ($($t:ty)*) => ($(
        impl Div<$t> for Amount {
            type Output = Amount;

            fn div(self, rhs: $t) -> Amount {
                Amount { value: self.value / rhs as u64 }
            }
        }

        impl DivAssign<$t> for Amount {
            fn div_assign(&mut self, rhs: $t) {
                self.value /= rhs as u64
            }
        }

        impl Rem<$t> for Amount {
            type Output = Amount;

            fn rem(self, rhs: $t) -> Amount {
                Amount { value: self.value % (rhs as u64 * 1_000)}
            }
        }

        impl RemAssign<$t> for Amount {
            fn rem_assign(&mut self, rhs: $t) {
                self.value %= rhs as u64 * 1_000
            }
        }

        impl Mul<$t> for Amount {
            type Output = Amount;

            fn mul(self, rhs: $t) -> Amount {
                Amount { value: self.value * rhs as u64 }
            }
        }

        impl Mul<Amount> for $t {
            type Output = Amount;

            fn mul(self, rhs: Amount) -> Amount {
                Amount { value: self as u64 * rhs.value }
            }
        }

        impl MulAssign<$t> for Amount {
            fn mul_assign(&mut self, rhs: $t) {
                self.value *= rhs as u64
            }
        }
    )*)
}

impl_ops_int! { u8 u16 u32 u64 usize }

impl Add for Amount {
    type Output = Amount;

    fn add(self, rhs: Amount) -> Amount {
        Amount { value: self.value + rhs.value }
    }
}

impl AddAssign for Amount {
    fn add_assign(&mut self, rhs: Amount) {
        self.value += rhs.value
    }
}

impl Sub for Amount {
    type Output = Amount;

    fn sub(self, rhs: Amount) -> Amount {
        Amount { value: self.value - rhs.value }
    }
}

impl SubAssign for Amount {
    fn sub_assign(&mut self, rhs: Amount) {
        self.value -= rhs.value
    }
}
