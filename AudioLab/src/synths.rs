pub mod sine_synth {
    pub struct SineSynth {
        sample_rate: f32,
        freq: f32,
        phase: f32
    }
    impl SineSynth {
        pub fn new(freq: f32) -> SineSynth {
            SineSynth {
                sample_rate: 3.0,
                freq: freq,
                phase: 0.0
            }
        }
        pub fn getSample(&mut self) -> f32 {
            self.phase += ((220.0) / 48000.0) * 2.0 * 3.14159265;
            self.phase.sin()*0.1
        }
    }
}