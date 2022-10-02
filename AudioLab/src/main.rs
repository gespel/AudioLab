pub mod synths;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};


struct MainHandle {
    pub synth1: synths::sine_synth::SineSynth
}
impl MainHandle {
    
    pub fn init(&mut self) {
        let host = cpal::default_host();
        let device = host.default_output_device().expect("no output device available");
        let config = device.default_output_config().unwrap();
        match config.sample_format() {
            cpal::SampleFormat::F32 => {
                self.run::<f32>(&device, &config.into()).unwrap();
            }
    
            cpal::SampleFormat::I16 => {
                self.run::<i16>(&device, &config.into()).unwrap();
            }
                
            cpal::SampleFormat::U16 => {
                self.run::<u16>(&device, &config.into()).unwrap();
            }    
        }

    }
    fn run<T>(&mut self, device: &cpal::Device, config: &cpal::StreamConfig) -> Result<(), anyhow::Error> where T: cpal::Sample {

        // Get the sample rate and channels number from the config
        let sample_rate = config.sample_rate.0 as f32;
        let channels = config.channels as usize;

        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);
        
        let mut synth1 = synths::sine_synth::SineSynth::new(440.0, sample_rate);
        let mut synth2 = synths::sine_synth::SineSynth::new(4.0, sample_rate);
        
        // Build an output stream
        let stream = device.build_output_stream(
            config,
            move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                for frame in data.chunks_mut(channels) {
                    let x = synth1.getSample();
                    synth1.setFrequency(synth2.getSample()*2000.0);
                    let value: T = cpal::Sample::from::<f32>(&x);
                    for sample in frame.iter_mut() {
                        *sample = value;
                    }
                }
            },
            err_fn,
        )?;

        // Play the stream
        stream.play()?;
        
        // Park the thread so our noise plays continuously until the app is closed
        loop {

        }
    }
}

fn main() {
    let mut x = MainHandle {
        synth1: synths::sine_synth::SineSynth::new(440.0, 48000.0)
    };
    x.init();
}
