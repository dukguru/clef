#![allow(non_upper_case_globals)]

use super::*;
use super::{FLAT, NATURAL, SHARP};
use contracts::requires;
use std::ops::Sub;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Pitch {
    name: NoteName,
    octave: i32,
    accidental: Accidental,
}

impl Pitch {
    #[requires(octave >= -1 && octave <= 9, "octave must be in the range -1 to 9")]
    pub fn new(name: NoteName, octave: i32) -> Pitch {
        Pitch {
            name,
            octave,
            accidental: NATURAL,
        }
    }

    #[requires(octave >= -1 && octave <= 9, "octave must be in the range -1 to 9")]
    pub fn new_with_accidental(name: NoteName, accidental: Accidental, octave: i32) -> Pitch {
        Pitch {
            name,
            octave,
            accidental,
        }
    }

    pub fn name(&self) -> NoteName {
        self.name
    }

    pub fn octave(&self) -> i32 {
        self.octave
    }

    pub fn accidental(&self) -> Accidental {
        self.accidental
    }
}

impl Sub for Pitch {
    type Output = i32;

    fn sub(self, other: Self) -> Self::Output {
        let from: i32 = self.name.0 + self.octave * 12 + self.accidental.0;
        let to: i32 = other.name.0 + other.octave * 12 + other.accidental.0;
        from - to
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_op_sub() {
        assert_eq!(0, C4 - C4);
        assert_eq!(-12, C4 - C5);
    }
}

pub const C_1: Pitch = Pitch {
    name: C,
    octave: -1,
    accidental: NATURAL,
};
pub const Cs_1: Pitch = Pitch {
    name: C,
    octave: -1,
    accidental: SHARP,
};
pub const Db_1: Pitch = Pitch {
    name: D,
    octave: -1,
    accidental: FLAT,
};
pub const D_1: Pitch = Pitch {
    name: D,
    octave: -1,
    accidental: NATURAL,
};
pub const Ds_1: Pitch = Pitch {
    name: D,
    octave: -1,
    accidental: SHARP,
};
pub const Eb_1: Pitch = Pitch {
    name: E,
    octave: -1,
    accidental: FLAT,
};
pub const E_1: Pitch = Pitch {
    name: E,
    octave: -1,
    accidental: NATURAL,
};
pub const F_1: Pitch = Pitch {
    name: F,
    octave: -1,
    accidental: NATURAL,
};
pub const Fs_1: Pitch = Pitch {
    name: F,
    octave: -1,
    accidental: SHARP,
};
pub const Gb_1: Pitch = Pitch {
    name: G,
    octave: -1,
    accidental: FLAT,
};
pub const G_1: Pitch = Pitch {
    name: G,
    octave: -1,
    accidental: NATURAL,
};
pub const Gs_1: Pitch = Pitch {
    name: G,
    octave: -1,
    accidental: SHARP,
};
pub const Ab_1: Pitch = Pitch {
    name: A,
    octave: -1,
    accidental: FLAT,
};
pub const A_1: Pitch = Pitch {
    name: A,
    octave: -1,
    accidental: NATURAL,
};
pub const As_1: Pitch = Pitch {
    name: A,
    octave: -1,
    accidental: SHARP,
};
pub const Bb_1: Pitch = Pitch {
    name: B,
    octave: -1,
    accidental: FLAT,
};
pub const B_1: Pitch = Pitch {
    name: B,
    octave: -1,
    accidental: NATURAL,
};

pub const C0: Pitch = Pitch {
    name: C,
    octave: 0,
    accidental: NATURAL,
};
pub const Cs0: Pitch = Pitch {
    name: C,
    octave: 0,
    accidental: SHARP,
};
pub const Db0: Pitch = Pitch {
    name: D,
    octave: 0,
    accidental: FLAT,
};
pub const D0: Pitch = Pitch {
    name: D,
    octave: 0,
    accidental: NATURAL,
};
pub const Ds0: Pitch = Pitch {
    name: D,
    octave: 0,
    accidental: SHARP,
};
pub const Eb0: Pitch = Pitch {
    name: E,
    octave: 0,
    accidental: FLAT,
};
pub const E0: Pitch = Pitch {
    name: E,
    octave: 0,
    accidental: NATURAL,
};
pub const F0: Pitch = Pitch {
    name: F,
    octave: 0,
    accidental: NATURAL,
};
pub const Fs0: Pitch = Pitch {
    name: F,
    octave: 0,
    accidental: SHARP,
};
pub const Gb0: Pitch = Pitch {
    name: G,
    octave: 0,
    accidental: FLAT,
};
pub const G0: Pitch = Pitch {
    name: G,
    octave: 0,
    accidental: NATURAL,
};
pub const Gs0: Pitch = Pitch {
    name: G,
    octave: 0,
    accidental: SHARP,
};
pub const Ab0: Pitch = Pitch {
    name: A,
    octave: 0,
    accidental: FLAT,
};
pub const A0: Pitch = Pitch {
    name: A,
    octave: 0,
    accidental: NATURAL,
};
pub const As0: Pitch = Pitch {
    name: A,
    octave: 0,
    accidental: SHARP,
};
pub const Bb0: Pitch = Pitch {
    name: B,
    octave: 0,
    accidental: FLAT,
};
pub const B0: Pitch = Pitch {
    name: B,
    octave: 0,
    accidental: NATURAL,
};

pub const C1: Pitch = Pitch {
    name: C,
    octave: 1,
    accidental: NATURAL,
};
pub const Cs1: Pitch = Pitch {
    name: C,
    octave: 1,
    accidental: SHARP,
};
pub const Db1: Pitch = Pitch {
    name: D,
    octave: 1,
    accidental: FLAT,
};
pub const D1: Pitch = Pitch {
    name: D,
    octave: 1,
    accidental: NATURAL,
};
pub const Ds1: Pitch = Pitch {
    name: D,
    octave: 1,
    accidental: SHARP,
};
pub const Eb1: Pitch = Pitch {
    name: E,
    octave: 1,
    accidental: FLAT,
};
pub const E1: Pitch = Pitch {
    name: E,
    octave: 1,
    accidental: NATURAL,
};
pub const F1: Pitch = Pitch {
    name: F,
    octave: 1,
    accidental: NATURAL,
};
pub const Fs1: Pitch = Pitch {
    name: F,
    octave: 1,
    accidental: SHARP,
};
pub const Gb1: Pitch = Pitch {
    name: G,
    octave: 1,
    accidental: FLAT,
};
pub const G1: Pitch = Pitch {
    name: G,
    octave: 1,
    accidental: NATURAL,
};
pub const Gs1: Pitch = Pitch {
    name: G,
    octave: 1,
    accidental: SHARP,
};
pub const Ab1: Pitch = Pitch {
    name: A,
    octave: 1,
    accidental: FLAT,
};
pub const A1: Pitch = Pitch {
    name: A,
    octave: 1,
    accidental: NATURAL,
};
pub const As1: Pitch = Pitch {
    name: A,
    octave: 1,
    accidental: SHARP,
};
pub const Bb1: Pitch = Pitch {
    name: B,
    octave: 1,
    accidental: FLAT,
};
pub const B1: Pitch = Pitch {
    name: B,
    octave: 1,
    accidental: NATURAL,
};

pub const C2: Pitch = Pitch {
    name: C,
    octave: 2,
    accidental: NATURAL,
};
pub const Cs2: Pitch = Pitch {
    name: C,
    octave: 2,
    accidental: SHARP,
};
pub const Db2: Pitch = Pitch {
    name: D,
    octave: 2,
    accidental: FLAT,
};
pub const D2: Pitch = Pitch {
    name: D,
    octave: 2,
    accidental: NATURAL,
};
pub const Ds2: Pitch = Pitch {
    name: D,
    octave: 2,
    accidental: SHARP,
};
pub const Eb2: Pitch = Pitch {
    name: E,
    octave: 2,
    accidental: FLAT,
};
pub const E2: Pitch = Pitch {
    name: E,
    octave: 2,
    accidental: NATURAL,
};
pub const F2: Pitch = Pitch {
    name: F,
    octave: 2,
    accidental: NATURAL,
};
pub const Fs2: Pitch = Pitch {
    name: F,
    octave: 2,
    accidental: SHARP,
};
pub const Gb2: Pitch = Pitch {
    name: G,
    octave: 2,
    accidental: FLAT,
};
pub const G2: Pitch = Pitch {
    name: G,
    octave: 2,
    accidental: NATURAL,
};
pub const Gs2: Pitch = Pitch {
    name: G,
    octave: 2,
    accidental: SHARP,
};
pub const Ab2: Pitch = Pitch {
    name: A,
    octave: 2,
    accidental: FLAT,
};
pub const A2: Pitch = Pitch {
    name: A,
    octave: 2,
    accidental: NATURAL,
};
pub const As2: Pitch = Pitch {
    name: A,
    octave: 2,
    accidental: SHARP,
};
pub const Bb2: Pitch = Pitch {
    name: B,
    octave: 2,
    accidental: FLAT,
};
pub const B2: Pitch = Pitch {
    name: B,
    octave: 2,
    accidental: NATURAL,
};

pub const C3: Pitch = Pitch {
    name: C,
    octave: 3,
    accidental: NATURAL,
};
pub const Cs3: Pitch = Pitch {
    name: C,
    octave: 3,
    accidental: SHARP,
};
pub const Db3: Pitch = Pitch {
    name: D,
    octave: 3,
    accidental: FLAT,
};
pub const D3: Pitch = Pitch {
    name: D,
    octave: 3,
    accidental: NATURAL,
};
pub const Ds3: Pitch = Pitch {
    name: D,
    octave: 3,
    accidental: SHARP,
};
pub const Eb3: Pitch = Pitch {
    name: E,
    octave: 3,
    accidental: FLAT,
};
pub const E3: Pitch = Pitch {
    name: E,
    octave: 3,
    accidental: NATURAL,
};
pub const F3: Pitch = Pitch {
    name: F,
    octave: 3,
    accidental: NATURAL,
};
pub const Fs3: Pitch = Pitch {
    name: F,
    octave: 3,
    accidental: SHARP,
};
pub const Gb3: Pitch = Pitch {
    name: G,
    octave: 3,
    accidental: FLAT,
};
pub const G3: Pitch = Pitch {
    name: G,
    octave: 3,
    accidental: NATURAL,
};
pub const Gs3: Pitch = Pitch {
    name: G,
    octave: 3,
    accidental: SHARP,
};
pub const Ab3: Pitch = Pitch {
    name: A,
    octave: 3,
    accidental: FLAT,
};
pub const A3: Pitch = Pitch {
    name: A,
    octave: 3,
    accidental: NATURAL,
};
pub const As3: Pitch = Pitch {
    name: A,
    octave: 3,
    accidental: SHARP,
};
pub const Bb3: Pitch = Pitch {
    name: B,
    octave: 3,
    accidental: FLAT,
};
pub const B3: Pitch = Pitch {
    name: B,
    octave: 3,
    accidental: NATURAL,
};

pub const C4: Pitch = Pitch {
    name: C,
    octave: 4,
    accidental: NATURAL,
};
pub const Cs4: Pitch = Pitch {
    name: C,
    octave: 4,
    accidental: SHARP,
};
pub const Db4: Pitch = Pitch {
    name: D,
    octave: 4,
    accidental: FLAT,
};
pub const D4: Pitch = Pitch {
    name: D,
    octave: 4,
    accidental: NATURAL,
};
pub const Ds4: Pitch = Pitch {
    name: D,
    octave: 4,
    accidental: SHARP,
};
pub const Eb4: Pitch = Pitch {
    name: E,
    octave: 4,
    accidental: FLAT,
};
pub const E4: Pitch = Pitch {
    name: E,
    octave: 4,
    accidental: NATURAL,
};
pub const F4: Pitch = Pitch {
    name: F,
    octave: 4,
    accidental: NATURAL,
};
pub const Fs4: Pitch = Pitch {
    name: F,
    octave: 4,
    accidental: SHARP,
};
pub const Gb4: Pitch = Pitch {
    name: G,
    octave: 4,
    accidental: FLAT,
};
pub const G4: Pitch = Pitch {
    name: G,
    octave: 4,
    accidental: NATURAL,
};
pub const Gs4: Pitch = Pitch {
    name: G,
    octave: 4,
    accidental: SHARP,
};
pub const Ab4: Pitch = Pitch {
    name: A,
    octave: 4,
    accidental: FLAT,
};
pub const A4: Pitch = Pitch {
    name: A,
    octave: 4,
    accidental: NATURAL,
};
pub const As4: Pitch = Pitch {
    name: A,
    octave: 4,
    accidental: SHARP,
};
pub const Bb4: Pitch = Pitch {
    name: B,
    octave: 4,
    accidental: FLAT,
};
pub const B4: Pitch = Pitch {
    name: B,
    octave: 4,
    accidental: NATURAL,
};

pub const C5: Pitch = Pitch {
    name: C,
    octave: 5,
    accidental: NATURAL,
};
pub const Cs5: Pitch = Pitch {
    name: C,
    octave: 5,
    accidental: SHARP,
};
pub const Db5: Pitch = Pitch {
    name: D,
    octave: 5,
    accidental: FLAT,
};
pub const D5: Pitch = Pitch {
    name: D,
    octave: 5,
    accidental: NATURAL,
};
pub const Ds5: Pitch = Pitch {
    name: D,
    octave: 5,
    accidental: SHARP,
};
pub const Eb5: Pitch = Pitch {
    name: E,
    octave: 5,
    accidental: FLAT,
};
pub const E5: Pitch = Pitch {
    name: E,
    octave: 5,
    accidental: NATURAL,
};
pub const F5: Pitch = Pitch {
    name: F,
    octave: 5,
    accidental: NATURAL,
};
pub const Fs5: Pitch = Pitch {
    name: F,
    octave: 5,
    accidental: SHARP,
};
pub const Gb5: Pitch = Pitch {
    name: G,
    octave: 5,
    accidental: FLAT,
};
pub const G5: Pitch = Pitch {
    name: G,
    octave: 5,
    accidental: NATURAL,
};
pub const Gs5: Pitch = Pitch {
    name: G,
    octave: 5,
    accidental: SHARP,
};
pub const Ab5: Pitch = Pitch {
    name: A,
    octave: 5,
    accidental: FLAT,
};
pub const A5: Pitch = Pitch {
    name: A,
    octave: 5,
    accidental: NATURAL,
};
pub const As5: Pitch = Pitch {
    name: A,
    octave: 5,
    accidental: SHARP,
};
pub const Bb5: Pitch = Pitch {
    name: B,
    octave: 5,
    accidental: FLAT,
};
pub const B5: Pitch = Pitch {
    name: B,
    octave: 5,
    accidental: NATURAL,
};

pub const C6: Pitch = Pitch {
    name: C,
    octave: 6,
    accidental: NATURAL,
};
pub const Cs6: Pitch = Pitch {
    name: C,
    octave: 6,
    accidental: SHARP,
};
pub const Db6: Pitch = Pitch {
    name: D,
    octave: 6,
    accidental: FLAT,
};
pub const D6: Pitch = Pitch {
    name: D,
    octave: 6,
    accidental: NATURAL,
};
pub const Ds6: Pitch = Pitch {
    name: D,
    octave: 6,
    accidental: SHARP,
};
pub const Eb6: Pitch = Pitch {
    name: E,
    octave: 6,
    accidental: FLAT,
};
pub const E6: Pitch = Pitch {
    name: E,
    octave: 6,
    accidental: NATURAL,
};
pub const F6: Pitch = Pitch {
    name: F,
    octave: 6,
    accidental: NATURAL,
};
pub const Fs6: Pitch = Pitch {
    name: F,
    octave: 6,
    accidental: SHARP,
};
pub const Gb6: Pitch = Pitch {
    name: G,
    octave: 6,
    accidental: FLAT,
};
pub const G6: Pitch = Pitch {
    name: G,
    octave: 6,
    accidental: NATURAL,
};
pub const Gs6: Pitch = Pitch {
    name: G,
    octave: 6,
    accidental: SHARP,
};
pub const Ab6: Pitch = Pitch {
    name: A,
    octave: 6,
    accidental: FLAT,
};
pub const A6: Pitch = Pitch {
    name: A,
    octave: 6,
    accidental: NATURAL,
};
pub const As6: Pitch = Pitch {
    name: A,
    octave: 6,
    accidental: SHARP,
};
pub const Bb6: Pitch = Pitch {
    name: B,
    octave: 6,
    accidental: FLAT,
};
pub const B6: Pitch = Pitch {
    name: B,
    octave: 6,
    accidental: NATURAL,
};

pub const C7: Pitch = Pitch {
    name: C,
    octave: 7,
    accidental: NATURAL,
};
pub const Cs7: Pitch = Pitch {
    name: C,
    octave: 7,
    accidental: SHARP,
};
pub const Db7: Pitch = Pitch {
    name: D,
    octave: 7,
    accidental: FLAT,
};
pub const D7: Pitch = Pitch {
    name: D,
    octave: 7,
    accidental: NATURAL,
};
pub const Ds7: Pitch = Pitch {
    name: D,
    octave: 7,
    accidental: SHARP,
};
pub const Eb7: Pitch = Pitch {
    name: E,
    octave: 7,
    accidental: FLAT,
};
pub const E7: Pitch = Pitch {
    name: E,
    octave: 7,
    accidental: NATURAL,
};
pub const F7: Pitch = Pitch {
    name: F,
    octave: 7,
    accidental: NATURAL,
};
pub const Fs7: Pitch = Pitch {
    name: F,
    octave: 7,
    accidental: SHARP,
};
pub const Gb7: Pitch = Pitch {
    name: G,
    octave: 7,
    accidental: FLAT,
};
pub const G7: Pitch = Pitch {
    name: G,
    octave: 7,
    accidental: NATURAL,
};
pub const Gs7: Pitch = Pitch {
    name: G,
    octave: 7,
    accidental: SHARP,
};
pub const Ab7: Pitch = Pitch {
    name: A,
    octave: 7,
    accidental: FLAT,
};
pub const A7: Pitch = Pitch {
    name: A,
    octave: 7,
    accidental: NATURAL,
};
pub const As7: Pitch = Pitch {
    name: A,
    octave: 7,
    accidental: SHARP,
};
pub const Bb7: Pitch = Pitch {
    name: B,
    octave: 7,
    accidental: FLAT,
};
pub const B7: Pitch = Pitch {
    name: B,
    octave: 7,
    accidental: NATURAL,
};

pub const C8: Pitch = Pitch {
    name: C,
    octave: 8,
    accidental: NATURAL,
};
pub const Cs8: Pitch = Pitch {
    name: C,
    octave: 8,
    accidental: SHARP,
};
pub const Db8: Pitch = Pitch {
    name: D,
    octave: 8,
    accidental: FLAT,
};
pub const D8: Pitch = Pitch {
    name: D,
    octave: 8,
    accidental: NATURAL,
};
pub const Ds8: Pitch = Pitch {
    name: D,
    octave: 8,
    accidental: SHARP,
};
pub const Eb8: Pitch = Pitch {
    name: E,
    octave: 8,
    accidental: FLAT,
};
pub const E8: Pitch = Pitch {
    name: E,
    octave: 8,
    accidental: NATURAL,
};
pub const F8: Pitch = Pitch {
    name: F,
    octave: 8,
    accidental: NATURAL,
};
pub const Fs8: Pitch = Pitch {
    name: F,
    octave: 8,
    accidental: SHARP,
};
pub const Gb8: Pitch = Pitch {
    name: G,
    octave: 8,
    accidental: FLAT,
};
pub const G8: Pitch = Pitch {
    name: G,
    octave: 8,
    accidental: NATURAL,
};
pub const Gs8: Pitch = Pitch {
    name: G,
    octave: 8,
    accidental: SHARP,
};
pub const Ab8: Pitch = Pitch {
    name: A,
    octave: 8,
    accidental: FLAT,
};
pub const A8: Pitch = Pitch {
    name: A,
    octave: 8,
    accidental: NATURAL,
};
pub const As8: Pitch = Pitch {
    name: A,
    octave: 8,
    accidental: SHARP,
};
pub const Bb8: Pitch = Pitch {
    name: B,
    octave: 8,
    accidental: FLAT,
};
pub const B8: Pitch = Pitch {
    name: B,
    octave: 8,
    accidental: NATURAL,
};

pub const C9: Pitch = Pitch {
    name: C,
    octave: 9,
    accidental: NATURAL,
};
pub const Cs9: Pitch = Pitch {
    name: C,
    octave: 9,
    accidental: SHARP,
};
pub const Db9: Pitch = Pitch {
    name: D,
    octave: 9,
    accidental: FLAT,
};
pub const D9: Pitch = Pitch {
    name: D,
    octave: 9,
    accidental: NATURAL,
};
pub const Ds9: Pitch = Pitch {
    name: D,
    octave: 9,
    accidental: SHARP,
};
pub const Eb9: Pitch = Pitch {
    name: E,
    octave: 9,
    accidental: FLAT,
};
pub const E9: Pitch = Pitch {
    name: E,
    octave: 9,
    accidental: NATURAL,
};
pub const F9: Pitch = Pitch {
    name: F,
    octave: 9,
    accidental: NATURAL,
};
pub const Fs9: Pitch = Pitch {
    name: F,
    octave: 9,
    accidental: SHARP,
};
pub const Gb9: Pitch = Pitch {
    name: G,
    octave: 9,
    accidental: FLAT,
};
pub const G9: Pitch = Pitch {
    name: G,
    octave: 9,
    accidental: NATURAL,
};
pub const Gs9: Pitch = Pitch {
    name: G,
    octave: 9,
    accidental: SHARP,
};
pub const Ab9: Pitch = Pitch {
    name: A,
    octave: 9,
    accidental: FLAT,
};
pub const A9: Pitch = Pitch {
    name: A,
    octave: 9,
    accidental: NATURAL,
};
pub const As9: Pitch = Pitch {
    name: A,
    octave: 9,
    accidental: SHARP,
};
pub const Bb9: Pitch = Pitch {
    name: B,
    octave: 9,
    accidental: FLAT,
};
pub const B9: Pitch = Pitch {
    name: B,
    octave: 9,
    accidental: NATURAL,
};
