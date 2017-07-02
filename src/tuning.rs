use super::{Hz, Cents};

const A440: Hz = Hz(440.0);

/// A general trait for tunings.
/// 
/// `tun.pitch(0)` should be the same as `Some(tun.reference_pitch())`
pub trait Tuning {
    /// Returns the reference pitch of the tuning
    fn reference_pitch(&self) -> Hz;
    /// Returns the pitch of a step
    fn pitch(&self, step: i32) -> Option<Hz>;
    /// Returns an interval from one step to another
    fn interval(&self, from: i32, to: i32) -> Option<Cents> {
        match (self.pitch(from), self.pitch(to)) {
            (Some(to), Some(from)) => Some(to / from),
            _ => None,
        }
    }
}

/// Equal division of 2:1
#[derive(Debug, Clone)]
pub struct Edo {
    cardinality: u16,
    reference: Hz,
}

impl Edo {
    /// Creates a new EDO with given cardinality and reference pitch
    pub fn new(cardinality: u16, reference: Hz) -> Self {
        Edo {
            cardinality, reference
        }
    }

    /// Creates a new EDO with given cardinality and `Hz(440.0)` as reference pitch
    pub fn new_a440(cardinality: u16) -> Self {
        Self::new(cardinality, A440)
    }
}

impl Tuning for Edo {
    fn reference_pitch(&self) -> Hz {
        self.reference
    }

    fn pitch(&self, step: i32) -> Option<Hz> {
        let int = Cents(1200.0 / self.cardinality as f32) * step as f32;
        Some(self.reference + int)
    }

    fn interval(&self, from: i32, to: i32) -> Option<Cents> {
        let delta = (to - from) as f32;
        Some(Cents(1200.0 / self.cardinality as f32 * delta))
    }
}

#[derive(Debug, Clone)]
/// Tuning with equal steps
pub struct EqualSteps {
    step: Cents,
    reference: Hz,
}

impl EqualSteps {
    /// Creates a new tuning with step size `step` and given reference pitch
    pub fn new(step: Cents, reference: Hz) -> Self {
        EqualSteps {
            step, reference
        }
    }

    /// Creates a new tuning with given step size and `Hz(440.0)` as reference pitch
    pub fn new_a440(step: Cents) -> Self {
        Self::new(step, A440)
    }
}

impl Tuning for EqualSteps {
    fn reference_pitch(&self) -> Hz {
        self.reference
    }

    fn pitch(&self, step: i32) -> Option<Hz> {
        let int = self.step * step as f32;
        Some(self.reference + int)
    }
}

#[derive(Debug, Clone)]
/// A map from MIDI notes to pitches
/// 
/// This is what your synth should actually use
pub struct MidiTuning {
    pitches: Vec<Hz>,
}

impl MidiTuning {
    /// Lineary maps the tuning to MIDI steps. `refkey` is a MIDI note of the reference pitch.
    pub fn from_tuning<T: Tuning>(tuning: T, refkey: u8) -> Option<Self> {
        assert!(refkey < 127);

        let mut pitches = Vec::with_capacity(127);
        for i in 0..127 {
            if let Some(hz) = tuning.pitch(i - refkey as i32) {
                pitches.push(hz)
            }
            else {
                return None
            }
        }

        Some(MidiTuning {
            pitches
        })
    }

    /// Creates `MidiTuning` from a slice of pitches
    pub fn from_pitches(hzs: &[Hz]) -> Option<Self> {
        if hzs.len() < 127 { return None }

        let pitches = hzs[0..127].to_owned();
        Some(MidiTuning {
            pitches
        })   
    }
}

impl ::std::default::Default for MidiTuning {
    /// Makes 12EDO tuning with A440 at MIDI note 69
    fn default() -> Self {
        let mut pitches = Vec::with_capacity(127);
        pitches.extend((0..127).map(|i| A440 + Cents((i - 69) as f32 * 100.0)));

        MidiTuning {
            pitches
        }
    }
}

impl Tuning for MidiTuning {
    fn reference_pitch(&self) -> Hz {
        self.pitches[0]
    }

    fn pitch(&self, step: i32) -> Option<Hz> {
        if step < 0 { return None }
        self.pitches.get(step as usize).cloned()
    }
}

impl ::std::ops::Index<usize> for MidiTuning {
    type Output = Hz;
    fn index(&self, index: usize) -> &Hz {
        &self.pitches[index]
    }
}
