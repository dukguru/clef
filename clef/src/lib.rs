pub mod math;

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
        let _half = crate::math::Fraction::new(1, 2);
    }
}