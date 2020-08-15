use std::{cmp, ops::Add};

#[cfg(test)]
mod tests;

/// A 2048-bit unsigned integer type.
///
/// # Representation
///
/// The representation of an `Int` is a 32 digit number in base 2⁶⁴.
#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Int {
    dgt: [u64; Self::DIGITS],
    len: usize,
}

impl Int {
    /// The smallest value that can be represented by this integer type
    pub const MIN: Self = Self {
        dgt: [0; Self::DIGITS],
        len: 0,
    };

    /// The largest value that can be represented by this integer type
    pub const MAX: Self = Self {
        dgt: [u64::MAX; Self::DIGITS],
        len: Self::DIGITS,
    };

    /// Create a new `Int` with value `0`.
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new `Int` with the given digits in base 2⁶⁴.
    ///
    /// # Overflow Behavior
    ///
    /// If the number of digits is greater than 32, the exceeding digits are
    /// ignored.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use flint_rs::Int;
    ///
    /// let a = Int::with_digits(&[45979, 38344]);
    /// ```
    pub fn with_digits(digits: &[u64]) -> Self {
        let mut value = Self {
            dgt: [0; Self::DIGITS],
            len: cmp::min(digits.len(), Self::DIGITS),
        };

        for (v, d) in value.dgt.iter_mut().zip(digits.iter()) {
            *v = *d;
        }

        value.remove_leading_zeros();
        value
    }
}

impl Add for &Int {
    type Output = Int;

    fn add(self, rhs: Self) -> Self::Output {
        let mut sum = Int {
            dgt: [0; Int::DIGITS],
            len: cmp::min(cmp::max(self.len, rhs.len) + 1, Int::DIGITS),
        };

        let mut accum = 0;

        for (s, (l, r)) in sum.dgt[..sum.len]
            .iter_mut()
            .zip(self.dgt.iter().zip(rhs.dgt.iter()))
        {
            accum += *l as u128 + *r as u128;
            *s = accum as u64;

            accum >>= Int::BITS_PER_DIGIT;
        }

        sum.remove_leading_zeros();
        sum
    }
}

impl Add<Int> for &Int {
    type Output = Int;

    fn add(self, rhs: Int) -> Self::Output {
        self + &rhs
    }
}

impl Add<&Int> for Int {
    type Output = Int;

    fn add(self, rhs: &Int) -> Self::Output {
        &self + rhs
    }
}

impl Int {
    const BITS_PER_DIGIT: usize = 64;
    const DIGITS: usize = 32;

    fn remove_leading_zeros(&mut self) {
        self.len = match self.dgt[..self.len].iter().rposition(|d| *d != 0) {
            Some(n) => n + 1,
            None => 0,
        };
    }
}
