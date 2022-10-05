mod synths;
pub mod racks;
use crate::racks::Rack;



use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};


struct MainHandle {
    pub synth1: synths::SineSynth
}
impl MainHandle {
    
    pub fn init(&mut self, mut r: Rack) {
        let host = cpal::default_host();
        let device = host.default_output_device().expect("no output device available");
        let config = device.default_output_config().unwrap();
        match config.sample_format() {
            cpal::SampleFormat::F32 => {
                self.run::<f32>(&device, &config.into(), r).unwrap();
            }
    
            cpal::SampleFormat::I16 => {
                self.run::<i16>(&device, &config.into(), r).unwrap();
            }
                
            cpal::SampleFormat::U16 => {
                self.run::<u16>(&device, &config.into(), r).unwrap();
            }    
        }

    }
    fn run<T>(&mut self, device: &cpal::Device, config: &cpal::StreamConfig, mut r: Rack) -> Result<(), anyhow::Error> where T: cpal::Sample {

        // Get the sample rate and channels number from the config
        let sample_rate = config.sample_rate.0 as f32;
        let channels = config.channels as usize;

        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);
        
        

        //let mut synth1 = synths::SineSynth::new(880.0, sample_rate);
        let mut synthy: synths::SineSynth = synths::SineSynth::new(420.0, sample_rate);
        //let mut synthx = synths::SquareSynth::new(420.0, sample_rate);
        //let mut synth2 = synths::SineSynth::new(5.0, sample_rate);
        //let mut synth3 = synths::SquareSynth::new(3.0, sample_rate);
        //let mut synth4 = synths::SquareSynth::new(0.5, sample_rate);
        
        // Build an output stream
        let stream = device.build_output_stream(
            config,
            move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                for frame in data.chunks_mut(channels) {
                    //let x = synthy.get_sample()*3.0;
                    //let x = 0.0;
                    let mut x = r.get_sample();
                    //let x = synthy.get_sample();
                    //synth1.set_frequency(synth2.get_sample()*2000.0);
                    //synth2.set_frequency(synth3.get_sample()*1000.0);
                    //synth3.set_frequency(synth4.get_sample()*4000.0);
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
        synth1: synths::SineSynth::new(440.0, 48000.0)
    };
    let mut r = racks::Rack::new();
    let mut synth_a: synths::SineSynth = synths::SineSynth::new(420.0, 48000.0);
    r.add_synth(synths::Synth::Sine(synth_a));
    x.init(r);
}
