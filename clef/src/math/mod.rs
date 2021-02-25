use std::cmp;

mod fraction;
pub use fraction::Fraction;

pub fn gcd(a: i32, b: i32) -> i32 {
    let mut a = a.abs();
    let mut b = b.abs();

    while a != 0 && b != 0 {
        if a > b {
            a %= b;
        } else {
            b %= a;
        }
    }

    cmp::max(a, b)
}

pub fn is_power_of_2(x: u32) -> bool {
    (x != 0) && ((x & (x - 1)) == 0)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(11, 21), 1);
        assert_eq!(gcd(14, 35), 7);
        assert_eq!(gcd(35, 14), 7);
        assert_eq!(gcd(14, -35), 7);
        assert_eq!(gcd(-35, 14), 7);
        assert_eq!(gcd(-35, -14), 7);
        assert_eq!(gcd(0, 0), 0);
        assert_eq!(gcd(0, 14), 14);
        assert_eq!(gcd(14, 0), 14);
        assert_eq!(gcd(-14, 0), 14);
        assert_eq!(gcd(0, -14), 14);
    }

    #[test]
    fn test_is_power_of_2() {
        assert!(is_power_of_2(1));
        assert!(is_power_of_2(128));
        assert!(!is_power_of_2(0));
        assert!(!is_power_of_2(1023));
    }
}
