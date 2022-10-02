pub mod sine_synth {
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
        pub fn getSample(&mut self) -> f32 {
            self.phase += ((self.freq) / self.sample_rate) * 2.0 * 3.14159265;
            self.phase.sin()*0.1
        }
        pub fn setFrequency(&mut self, freq: f32) {
            self.freq = freq;
        }
    }
}