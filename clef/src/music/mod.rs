mod duration;

pub use duration::Duration;
pub mod pitch;
pub use pitch::*;

pub mod tune_sys;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct NoteName(i32);
pub const C: NoteName = NoteName(0);
pub const D: NoteName = NoteName(2);
pub const E: NoteName = NoteName(4);
pub const F: NoteName = NoteName(5);
pub const G: NoteName = NoteName(7);
pub const A: NoteName = NoteName(9);
pub const B: NoteName = NoteName(11);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Accidental(i32);
pub const NATURAL: Accidental = Accidental(0);
pub const SHARP: Accidental = Accidental(1);
pub const FLAT: Accidental = Accidental(-1);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Chord {
    root: Pitch
}