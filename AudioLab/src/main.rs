pub mod synths;

use std::ops::Sub;
use cpal::{Data, Sample, SampleFormat};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use rand::Rng;
use std::thread;


struct MainHandle {
    pub synth1: synths::sine_synth::SineSynth
}
impl MainHandle {
    fn write_silence<T: Sample>(&mut self, synth1: synths::sine_synth::SineSynth, data: &mut [T], _: &cpal::OutputCallbackInfo) {
        let mut synth1: synths::sine_synth::SineSynth = synths::sine_synth::SineSynth::new(440.0);

        for sample in data.iter_mut() {
            *sample = Sample::from(&(synth1.getSample()));
            //println!("{}", phase.sin())
        }
    }
    pub fn run(&mut self) {
        let host = cpal::default_host();
        let device = host.default_output_device().expect("no output device available");
        let mut supported_configs_range = device.supported_output_configs()
            .expect("error while querying configs");
        let supported_config = supported_configs_range.next()
            .expect("no supported config?!")
            .with_max_sample_rate();
        let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);
        let sample_format = supported_config.sample_format();
        let config = supported_config.into();
        let stream = match sample_format {
            SampleFormat::F32 => device.build_output_stream(&config, Sample {
                self.synth1.getSample()
            }, err_fn),
            SampleFormat::I16 => device.build_output_stream(&config, write_silence::<i16>, err_fn),
            SampleFormat::U16 => device.build_output_stream(&config, write_silence::<u16>, err_fn),
        }.unwrap();
        let mut synth1: synths::sine_synth::SineSynth = synths::sine_synth::SineSynth::new(440.0);


        stream.play().unwrap();
        loop {
            let x = synths::sine_synth::SineSynth::new(440.0);
        }
    }
}

fn main() {
    let mut x = MainHandle {
        synth1: synths::sine_synth::SineSynth::new(440.0)
    };
    x.run();
}
