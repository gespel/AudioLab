mod synths;
pub mod racks;
use crate::racks::Rack;



use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};


struct MainHandle {
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
    fn run<T>(&mut self, device: &cpal::Device, config: &cpal::StreamConfig) -> Result<(), anyhow::Error> {
        let mut r: Rack = Rack::new(device, config);
        r.process();
        loop {

        }
    }
}

fn main() {
    let mut x = MainHandle {
    };
    x.init();
}
