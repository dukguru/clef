use crate::math;
use contracts::requires;
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Eq, Ord)]
pub struct Fraction {
    numerator: i32,
    denominator: i32,
}

impl Fraction {
    pub const ZERO: Fraction = Fraction {
        numerator: 0,
        denominator: 1,
    };

    pub const ONE: Fraction = Fraction {
        numerator: 1,
        denominator: 1,
    };

    pub const HALF: Fraction = Fraction {
        numerator: 1,
        denominator: 2,
    };
}

impl Fraction {
    #[requires(denominator != 0, "denominator must not be zero")]
    pub fn new(numerator: i32, denominator: i32) -> Fraction {
        Self {
            numerator,
            denominator,
        }
    }

    pub fn numerator(&self) -> i32 {
        self.numerator
    }

    pub fn denominator(&self) -> i32 {
        self.denominator
    }

    pub fn signum(&self) -> i32 {
        self.numerator.signum() * self.denominator.signum()
    }

    pub fn to_irreducible(&self) -> Self {
        let gcd = math::gcd(self.numerator, self.denominator);

        Self {
            numerator: (self.numerator / gcd).abs() * self.signum(),
            denominator: (self.denominator / gcd).abs(),
        }
    }

    pub fn to_float(&self) -> f32 {
        self.numerator as f32 / self.denominator as f32
    }
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

impl PartialEq for Fraction {
    fn eq(&self, other: &Self) -> bool {
        let l = self.to_irreducible();
        let r = other.to_irreducible();
        l.numerator == r.numerator && l.denominator == r.denominator
    }
}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_float().partial_cmp(&other.to_float())
    }
}

impl Add for Fraction {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        if self.denominator == other.denominator {
            Self::new(self.numerator + other.numerator, self.denominator).to_irreducible()
        } else {
            let numerator = self.numerator * other.denominator + other.numerator * self.denominator;
            let denominator = self.denominator * other.denominator;
            Self::new(numerator, denominator).to_irreducible()
        }
    }
}

impl Add<i32> for Fraction {
    type Output = Self;

    fn add(self, other: i32) -> Self::Output {
        self + Fraction::new(other, 1)
    }
}

impl AddAssign for Fraction {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl AddAssign<i32> for Fraction {
    fn add_assign(&mut self, other: i32) {
        *self = *self + other;
    }
}

impl Div for Fraction {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self::new(
            self.numerator * other.denominator,
            self.denominator * other.numerator,
        )
        .to_irreducible()
    }
}

impl Div<i32> for Fraction {
    type Output = Self;

    fn div(self, other: i32) -> Self::Output {
        Self::new(self.numerator * other, self.denominator * other).to_irreducible()
    }
}

impl DivAssign for Fraction {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}

impl DivAssign<i32> for Fraction {
    fn div_assign(&mut self, other: i32) {
        *self = *self / other;
    }
}

impl Mul for Fraction {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self::new(
            self.numerator * other.numerator,
            self.denominator * other.denominator,
        )
        .to_irreducible()
    }
}

impl Mul<i32> for Fraction {
    type Output = Self;

    fn mul(self, other: i32) -> Self::Output {
        Self::new(self.numerator * other, self.denominator).to_irreducible()
    }
}

impl MulAssign for Fraction {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl MulAssign<i32> for Fraction {
    fn mul_assign(&mut self, other: i32) {
        *self = *self * other;
    }
}

impl Neg for Fraction {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(self.numerator * -1, self.denominator).to_irreducible()
    }
}

impl Sub for Fraction {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self + -other
    }
}

impl Sub<i32> for Fraction {
    type Output = Self;

    fn sub(self, other: i32) -> Self::Output {
        self + -other
    }
}

impl SubAssign for Fraction {
    fn sub_assign(&mut self, other: Self) {
        *self += -other;
    }
}

impl SubAssign<i32> for Fraction {
    fn sub_assign(&mut self, other: i32) {
        *self += -other;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_zero_denominator() {
        let _illegal = Fraction::new(1, 0);
    }

    #[test]
    fn test_signum() {
        assert_eq!(Fraction::new(1, 2).signum(), 1);
        assert_eq!(Fraction::new(-1, 2).signum(), -1);
        assert_eq!(Fraction::new(1, -2).signum(), -1);
        assert_eq!(Fraction::new(-1, -2).signum(), 1);
        assert_eq!(Fraction::new(0, 2).signum(), 0);
    }

    #[test]
    fn test_to_irreducible() {
        assert_eq!(Fraction::new(3, 9).to_irreducible(), Fraction::new(1, 3));
        assert_eq!(Fraction::new(27, 9).to_irreducible(), Fraction::new(3, 1));
        assert_eq!(
            Fraction::new(11, 13).to_irreducible(),
            Fraction::new(11, 13)
        );
        assert_eq!(Fraction::new(-3, 9).to_irreducible(), Fraction::new(-1, 3));
        assert_eq!(Fraction::new(3, -9).to_irreducible(), Fraction::new(-1, 3));
        assert_eq!(Fraction::new(-3, -9).to_irreducible(), Fraction::new(1, 3));
    }

    #[test]
    fn test_to_float() {
        assert_eq!(0.25, Fraction::new(1, 4).to_float());
        assert_eq!(-0.25, Fraction::new(-1, 4).to_float());
    }

    #[test]
    fn test_op_eq() {
        assert!(Fraction::new(5, -10) == Fraction::new(-1, 2));
        assert!(Fraction::new(5, 10) != Fraction::new(1, 3));
    }

    #[test]
    fn test_op_ord() {
        assert!(Fraction::new(3, 5) < Fraction::new(4, 5));
        assert!(Fraction::new(3, 5) <= Fraction::new(15, 25));
    }

    #[test]
    fn test_op_add() {
        assert_eq!(
            Fraction::new(5, 10) + Fraction::new(5, 20),
            Fraction::new(3, 4)
        );
        assert_eq!(
            Fraction::new(-5, 10) + Fraction::new(5, 20),
            Fraction::new(-1, 4)
        );
        assert_eq!(
            Fraction::new(5, -10) + Fraction::new(5, 20),
            Fraction::new(-1, 4)
        );
        assert_eq!(
            Fraction::new(5, 10) + Fraction::new(0, 20),
            Fraction::new(1, 2)
        );

        assert_eq!(Fraction::ZERO + Fraction::new(3, 4), Fraction::new(3, 4));

        assert_eq!(Fraction::new(3, 4) + Fraction::ZERO, Fraction::new(3, 4));

        let mut a = Fraction::new(5, 10);
        let b = Fraction::new(5, 20);
        a += b;
        assert_eq!(a, Fraction::new(3, 4));
    }

    #[test]
    fn test_op_sub() {
        assert_eq!(
            Fraction::new(5, 10) - Fraction::new(5, 20),
            Fraction::new(1, 4)
        );
        assert_eq!(
            Fraction::new(5, 20) - Fraction::new(5, 10),
            Fraction::new(-1, 4)
        );
        assert_eq!(Fraction::new(5, 20) - Fraction::new(5, 20), Fraction::ZERO);
    }

    #[test]
    fn test_op_neg() {
        assert_eq!(-Fraction::new(5, 10), Fraction::new(-1, 2));
        assert_eq!(-Fraction::new(-5, 10), Fraction::new(1, 2));
        assert_eq!(-Fraction::new(5, -10), Fraction::new(1, 2));
        assert_eq!(-Fraction::new(-5, -10), Fraction::new(-1, 2));
    }
}
