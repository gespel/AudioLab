pub enum Synth {
    Sine(SineSynth),
    Square(SquareSynth)
}
impl Synth {
    pub fn get_sample(&self) -> f32 {
        match self {
            Synth::Sine(SineSynth{ref sample_rate, ref freq, ref phase}) => self.get_sample(),
            Synth::Square(SquareSynth{ref sample_rate, ref freq, ref phase}) => self.get_sample()
        }
    }
}

pub struct SineSynth {
    sample_rate: f32,
    freq: f32,
    phase: f32
}
impl SineSynth {
    pub fn new(freq: f32, sample_rate: f32) -> SineSynth {
        SineSynth {
            sample_rate: sample_rate,
            freq: freq,
            phase: 0.0
        }
    }
    pub fn get_sample(&mut self) -> f32 {
        self.phase += ((self.freq) / self.sample_rate) * 2.0 * 3.14159265;
        return self.phase.sin()*0.1;
    }
    pub fn set_frequency(&mut self, freq: f32) {
        self.freq = freq;
    }
}

pub struct SquareSynth {
    sample_rate: f32,
    freq: f32,
    phase: f32
}
impl SquareSynth {
    fn new(freq: f32, sample_rate: f32) -> SquareSynth {
        SquareSynth {
            sample_rate: sample_rate,
            freq: freq,
            phase: 0.0
        }
    }
    fn get_sample(&mut self) -> f32 {
        self.phase += ((self.freq) / self.sample_rate) * 2.0 * 3.14159265;
        let out: f32 = self.phase.sin()*0.1;
        if out > 0.0 {
            return 0.1;
        }
        else if out == 0.0 {
            return 0.0;
        }
        else {
            return -0.1;
        }
    }
    fn set_frequency(&mut self, freq: f32) {
        self.freq = freq;
    }
}