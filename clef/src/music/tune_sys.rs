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

    pub fn new(a4_hertz: f32) -> Self {
        Self { a4_hertz }
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

pub struct JustIntonation<'a> {
    ref_pitch: Pitch,
    ref_hertz: f32,
    ratio: &'a [f32; 12],
}

impl<'a> JustIntonation<'a> {
    const standard_ratio: [f32; 12] = [
        1.0,
        25.0 / 24.0,
        9.0 / 8.0,
        6.0 / 5.0,
        5.0 / 4.0,
        4.0 / 3.0,
        45.0 / 32.0,
        3.0 / 2.0,
        8.0 / 5.0,
        5.0 / 3.0,
        9.0 / 5.0,
        15.0 / 8.0,
    ];
}

impl<'a> JustIntonation<'a> {
    pub fn new(ref_pitch: Pitch, ref_hertz: f32) -> JustIntonation<'a> {
        Self {
            ref_pitch,
            ref_hertz,
            ratio: &Self::standard_ratio,
        }
    }
}

#[contract_trait]
impl<'a> TuningSystem for JustIntonation<'a> {
    fn to_hertz(&self, pitch: &Pitch) -> f32 {
        let intervals: i32 = *pitch - self.ref_pitch;
        let mut octave = intervals / 12;
        let mut tone = intervals % 12;
        
        if tone < 0 {
            tone += 12;
            octave -= 1;
        }
        
        self.ref_hertz * self.ratio[tone as usize] * 2.0f32.powi(octave)
    }

    fn to_pitch(&self, _hertz: f32) -> Pitch {
        C4
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

    #[test]
    fn just_to_hertz_test() {
        let just = JustIntonation::new(C4, 261.63);
        assert_eq!(418.608 * 0.25, just.to_hertz(&Ab2));
        assert_eq!(418.608 * 0.5, just.to_hertz(&Ab3));
        assert_eq!(436.05, just.to_hertz(&A4));
        assert_eq!(418.608, just.to_hertz(&Ab4));
        assert_eq!(523.26, just.to_hertz(&C5));
        assert_eq!(418.608 * 2.0, just.to_hertz(&Ab5));
    }
}
