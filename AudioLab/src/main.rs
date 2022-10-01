use cpal::{Data, Sample, SampleFormat};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use rand::Rng;

fn main() {
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
        SampleFormat::F32 => device.build_output_stream(&config, write_silence::<f32>, err_fn),
        SampleFormat::I16 => device.build_output_stream(&config, write_silence::<i16>, err_fn),
        SampleFormat::U16 => device.build_output_stream(&config, write_silence::<u16>, err_fn),
    }.unwrap();

    fn write_silence<T: Sample>(data: &mut [T], _: &cpal::OutputCallbackInfo) {
        let mut phase: f32 = 0.0;
        for sample in data.iter_mut() {
            *sample = Sample::from(&(phase.sin()*0.1));
            phase += ((220.0) / 48000.0) * 2.0 * 3.14159265;
        }
    }
    stream.play().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(4000));
}
