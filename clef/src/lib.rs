pub mod math;

mod music;
pub use music::pitch::*;
pub use music::tune_sys;
pub use music::Accidental;
pub use music::Chord;
pub use music::Duration;
pub use music::NoteName;
pub use music::{A, B, C, D, E, F, G};
pub use music::{FLAT, NATURAL, SHARP};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let _half = crate::Duration::new(2);
    }
}
