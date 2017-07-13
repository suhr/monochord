#![allow(dead_code)]

use std::ops::{Add, Sub, Mul, Div};

pub mod tuning;

/// Hertz is the standard unit of frequency.
/// 
/// It is also the standard unit of pitch as well.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hz(pub f32);

/// You can add an interval to Hz. For example:
///
/// ```rust
/// # use monochord::*;
/// Hz(440.0) + Cents(702.0); // Hz(660.0)
/// ```
impl Add<Cents> for Hz {
    type Output = Hz;
    fn add(self, rhs: Cents) -> Self::Output {
        let freq = self.0 * (rhs.0 / 1200.0).exp2();
        Hz(freq)
    }
}

impl Sub<Cents> for Hz {
    type Output = Hz;
    fn sub(self, rhs: Cents) -> Self::Output {
        let freq = self.0 * (-rhs.0 / 1200.0).exp2();
        Hz(freq)
    }
}

impl Mul<f32> for Hz {
    type Output = Hz;
    fn mul(self, rhs: f32) -> Self::Output {
        Hz(self.0 * rhs)
    }
}

impl Div<Hz> for Hz {
    /// `Hz(b) / Hz(a)` is equivalent to `Cents::from_ratio(b / a)`.
    type Output = Cents;
    fn div(self, rhs: Hz) -> Self::Output {
        Cents::from_ratio(self.0 / rhs.0)
    }
}

/// Cent is the standard unit of musical interval.
/// 
/// 12 EDO semitone is 100 cents large while an octave is 1200 cents large.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cents(pub f32);

impl Cents {
    pub fn from_ratio(ratio: f32) -> Cents {
        Cents(1200.0 * ratio.log2())
    }

    pub fn to_ratio(self) -> f32 {
        (self.0 / 1200.0).exp2()
    }
}

impl Add<Cents> for Cents {
    type Output = Cents;
    fn add(self, rhs: Cents) -> Self::Output {
        Cents(self.0 + rhs.0)
    }
}

impl Sub<Cents> for Cents {
    type Output = Cents;
    fn sub(self, rhs: Cents) -> Self::Output {
        Cents(self.0 - rhs.0)
    }
}

impl Mul<f32> for Cents {
    type Output = Cents;
    fn mul(self, rhs: f32) -> Self::Output {
        Cents(self.0 * rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn cents_ratio() {
        assert_eq!(
            Cents::from_ratio(2.0),
            Cents(1200.0)
        );
    }

    #[test] fn hz_plus_cents() {
        assert_eq!(
            (Hz(440.0) + Cents(-900.0)).0.round(),
            262.0
        );
    }

    #[test] fn hz_div_hz() {
        assert_eq!(
            Hz(660.0) / Hz(440.0),
            Cents::from_ratio(3.0 / 2.0)
        );
    }
}
