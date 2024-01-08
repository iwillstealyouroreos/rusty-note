use hound::{WavWriter, SampleFormat};
use rand::Rng;

fn main() -> Result<(), hound::Error> {
    let file_spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int
    };
    
    let mut wrtr = WavWriter::create("sample.wav", file_spec)?;

    let amplitude = i16::MAX as f32;
    let frequency = 440.0;
    let duration_secs = 20;
    let sample_rate = file_spec.sample_rate as f32;
    
    let mut rng = rand::thread_rng();
    let random_height: f32 = rng.gen_range(-1.0..1.5);

    for time in 0..(sample_rate as usize * duration_secs) {
        let sample = (amplitude * (random_height * std::f32::consts::PI * frequency * (time as f32) / sample_rate).sin()) as i16;
        wrtr.write_sample(sample)?;
    }

    wrtr.finalize()?;
    Ok(())
}
