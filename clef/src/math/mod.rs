pub struct Fraction {
    numerator: i32,
    denominator: i32
}

impl Fraction {
    pub fn new(numerator: i32, denominator: i32) -> Fraction{
        Fraction {
            numerator,
            denominator
        }
    }

    pub fn get_numerator(&self) -> i32 {
        self.numerator
    }

    pub fn get_denominator(&self) -> i32 {
        self.denominator
    }
}