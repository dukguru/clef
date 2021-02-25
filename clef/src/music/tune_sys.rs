use super::*;
use contracts::{contract_trait, ensures, requires};

#[contract_trait]
pub trait TuningSystem {
    #[ensures(ret > 0f32)]
    fn to_hertz(&self, pitch: &Pitch) -> f32;
    #[requires(hertz > 0f32)]
    fn to_pitch(&self, hertz: f32) -> Pitch;
}

pub struct EqualTemperament {
    a4_hertz: f32,
}

impl EqualTemperament {
    const TWELFTH_ROOT_OF_TWO: f32 = 1.05946309435929526456182;
    const LN_TWELFTH_ROOT_OF_TWO: f32 = 0.05776226504666210911809767902434;

    pub fn new(a4_hertz: f32) -> EqualTemperament {
        EqualTemperament { a4_hertz }
    }
}

#[contract_trait]
impl TuningSystem for EqualTemperament {
    fn to_hertz(&self, pitch: &Pitch) -> f32 {
        let intervals: i32 = *pitch - A4;
        self.a4_hertz * Self::TWELFTH_ROOT_OF_TWO.powi(intervals)
    }

    fn to_pitch(&self, hertz: f32) -> Pitch {
        let intervals = (hertz / self.a4_hertz).ln() / Self::LN_TWELFTH_ROOT_OF_TWO;
        let intervals_rounded = intervals.round();

        let accidental = match intervals - intervals_rounded {
            d if d > 0.0 => FLAT,
            _ => SHARP,
        };

        let intervals = intervals_rounded as i32;
        let mut octave = A4.octave() + intervals / 12;
        let mut tone = A4.name().0 + intervals % 12;

        if tone < 0 {
            tone += 12;
            octave -= 1;
        } else if tone > 11 {
            tone -= 12;
            octave += 1;
        }

        match tone {
            0 => Pitch::new(C, octave),
            1 => match accidental {
                FLAT => Pitch::new_with_accidental(D, FLAT, octave),
                _ => Pitch::new_with_accidental(C, SHARP, octave),
            },
            2 => Pitch::new(D, octave),
            3 => match accidental {
                FLAT => Pitch::new_with_accidental(E, FLAT, octave),
                _ => Pitch::new_with_accidental(D, SHARP, octave),
            },
            4 => Pitch::new(E, octave),
            5 => Pitch::new(F, octave),
            6 => match accidental {
                FLAT => Pitch::new_with_accidental(G, FLAT, octave),
                _ => Pitch::new_with_accidental(F, SHARP, octave),
            },
            7 => Pitch::new(G, octave),
            8 => match accidental {
                FLAT => Pitch::new_with_accidental(A, FLAT, octave),
                _ => Pitch::new_with_accidental(G, SHARP, octave),
            },
            9 => Pitch::new(A, octave),
            10 => match accidental {
                FLAT => Pitch::new_with_accidental(B, FLAT, octave),
                _ => Pitch::new_with_accidental(A, SHARP, octave),
            },
            11 => Pitch::new(B, octave),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn et_to_hertz_test() {
        let et = EqualTemperament::new(440.0);
        assert_eq!(440.0, et.to_hertz(&A4));
        assert_eq!(523.2512, et.to_hertz(&C5));
        assert_eq!(830.6098, et.to_hertz(&Gs5));
        assert_eq!(233.08176, et.to_hertz(&Bb3));
        assert_eq!(219.99988, et.to_hertz(&A3));
    }

    #[test]
    fn et_to_pitch_test() {
        let et = EqualTemperament::new(440.0);
        assert_eq!(A4, et.to_pitch(440.0));
        assert_eq!(C5, et.to_pitch(523.2512));
        assert_eq!(Gs5, et.to_pitch(830.6));
        assert_eq!(Ab5, et.to_pitch(830.7));
        assert_eq!(A3, et.to_pitch(219.99988));
        assert_eq!(Fs4, et.to_pitch(369.9));
    }
}
