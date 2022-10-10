use cpal::Stream;
use cpal::traits::{DeviceTrait, StreamTrait};
use crate::synths::SineSynth;
use crate::synths::SquareSynth;
use crate::synths::Synth;

use frunk::hlist;
use frunk::labelled::chars::T;
use crate::synths;

pub struct Rack<'a> {
    modules: Vec<Synth>,
    device: &'a cpal::Device,
    config: &'a cpal::StreamConfig
}
impl Rack<'_> {
    pub fn new<'a>(device: &'a cpal::Device, config: &'a cpal::StreamConfig) -> Rack<'a> {
        Rack{
            modules: Vec::new(),
            device: device,
            config: config
        }
    }
    pub fn add_synth(&mut self, input: Synth) {
        self.modules.push(input);
    }
    pub fn process(&mut self) {
        let sample_rate = self.config.sample_rate.0 as f32;
        let channels = self.config.channels as usize;
        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);
        let mut synthy: synths::SineSynth = synths::SineSynth::new(440.0, sample_rate);

        let stream = self.device.build_output_stream(
            self.config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                for frame in data.chunks_mut(channels) {
                    //let x = synthy.get_sample()*3.0;
                    //let x = 0.0;
                    let x = synthy.get_sample();
                    //synth1.set_frequency(synth2.get_sample()*2000.0);
                    //synth2.set_frequency(synth3.get_sample()*1000.0);
                    //synth3.set_frequency(synth4.get_sample()*4000.0);
                    let value: f32 = cpal::Sample::from::<f32>(&x);
                    for sample in frame.iter_mut() {
                        *sample += value;
                    }
                }
            },
            err_fn,
        );
        stream.expect("REASON").play();
    }
}