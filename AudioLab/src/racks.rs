use crate::synths::SineSynth;
use crate::synths::SquareSynth;
use crate::synths::Synth;

use frunk::hlist;

pub struct Rack {
    modules: Vec<Synth>
}
impl Rack {
    pub fn new() -> Rack {
        Rack{
            modules: Vec::new()   
        }
    }
    pub fn add_synth(&mut self, input: Synth) {
        self.modules.push(input);
    }
    pub fn get_sample(&mut self) -> f32 {
        let mut out: f32 = 0.0;
        for module in self.modules.iter() {
            out += module.get_sample();
        }
        return out;
    }
}