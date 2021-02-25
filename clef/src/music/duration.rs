use crate::math;
use crate::math::Fraction;
use contracts::requires;
use std::convert::TryFrom;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Duration {
    denominator: u16,
    dots: u8,
}

impl Duration {
    #[requires(math::is_power_of_2(denominator as u32), "denominator must be a power of 2")]
    pub fn new(denominator: u16) -> Duration {
        Duration {
            denominator,
            dots: 0,
        }
    }

    #[requires(math::is_power_of_2(denominator as u32), "denominator must be a power of 2")]
    #[requires(dots <= 4, "dots must be no more than 4")]
    pub fn new_with_dots(denominator: u16, dots: u8) -> Duration {
        Duration { denominator, dots }
    }

    pub fn denominator(&self) -> u16 {
        self.denominator
    }

    pub fn dots(&self) -> u8 {
        self.dots
    }

    pub fn to_fraction(&self) -> Fraction {
        let denominator = self.denominator as i32;
        let mut fraction = Fraction::new(1, denominator);

        if self.dots > 0 {
            let mut base_pow = 2;
            for _ in 1..=self.dots {
                fraction += Fraction::new(1, denominator * base_pow);
                base_pow *= 2;
            }
        }

        fraction
    }
}

impl TryFrom<Fraction> for Duration {
    type Error = &'static str;

    fn try_from(fraction: Fraction) -> Result<Self, Self::Error> {
        if fraction.signum() <= 0 {
            return Err("fraction must be positive");
        }

        let mut fraction = fraction;
        let mut denominator = 0u16;
        let mut dots = 0;

        let mut d = 1i32;
        while d <= 128 {
            let base = Fraction::new(1, d);
            if fraction >= base {
                fraction -= base;
                denominator = d as u16;
                break;
            }
            d *= 2
        }

        if denominator == 0 {
            return Err("denominator too large");
        }

        let mut half = Fraction::new(1, (denominator * 2).into());
        while dots < 4 {
            if fraction >= half {
                fraction -= half;
                dots += 1;
                half *= Fraction::HALF;
            } else {
                break;
            }
        }

        if fraction != Fraction::ZERO {
            return Err("not a durational fraction")
        }

        Ok(Duration::new_with_dots(denominator, dots))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_not_power_of_2_denominator() {
        let _illegal = Duration::new(3);
    }

    #[test]
    #[should_panic]
    fn test_more_than_4_dots() {
        let _illegal = Duration::new_with_dots(4, 5);
    }

    #[test]
    fn test_to_fraction() {
        assert_eq!(Fraction::new(1, 1), Duration::new(1).to_fraction());
        assert_eq!(Fraction::new(1, 4), Duration::new(4).to_fraction());
        assert_eq!(
            Fraction::new(1, 8),
            Duration::new_with_dots(8, 0).to_fraction()
        );
        assert_eq!(
            Fraction::new(7, 16),
            Duration::new_with_dots(4, 2).to_fraction()
        );
    }

    #[test]
    fn test_from_fraction() {
        assert_eq!(Duration::new(4), Duration::try_from(Fraction::new(1, 4)).unwrap());
        assert_eq!(Duration::new_with_dots(4, 2), Duration::try_from(Fraction::new(7, 16)).unwrap());
    }

    #[test]
    fn test_from_illegal_fraction() {
        assert!(Duration::try_from(Fraction::new(9, 16)).is_err());
        assert!(Duration::try_from(Fraction::new(1, 256)).is_err());
    }
}
