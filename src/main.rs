use hound::WavReader;
use rustfft::{num_complex::Complex, FftPlanner};
use std::fs;
use std::io::Write;

fn main() {
    let mut reader = WavReader::open("audio/440.wav").expect("failed to open wav");

    let spec = reader.spec();
    println!("wav file info");
    println!("channels {:?}", spec.channels);
    println!("sample_rate {:?}", spec.sample_rate);
    println!("bits_per_sample {:?}", spec.bits_per_sample);
    println!("sample_format {:?}", spec.sample_format);

    let samples: Vec<_> = reader
        .samples::<i16>()
        .map(|res| res.expect("failed to decode wav"))
        .map(|sample| Complex {
            re: sample as f32,
            im: 0.,
        })
        .collect();

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(samples.len());

    let mut spec_buffer = samples.clone();
    fft.process(&mut spec_buffer);

    let frequency_resolution = spec.sample_rate as f32 / samples.len() as f32;

    let fft_buffer: Vec<_> = spec_buffer
        .iter()
        .take(samples.len() / 2)
        .enumerate()
        .map(|(i, c)| {
            let f = i as f32 * frequency_resolution;
            let m = (c.norm() * 2. / samples.len() as f32).sqrt();
            (f, m)
        })
        .collect();

    // Dump buffer.
    let norm_factor: f32 = 1f32 / (samples.len() as f32).sqrt();
    let mut dump_file = fs::File::create("dump.dat").expect("failed to open dump file");
    for (f, m) in fft_buffer {
        write!(dump_file, "{:?} {:?}\n", f, m);
    }
}
