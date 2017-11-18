extern crate hound;
extern crate image;
extern crate stft;
use stft::{STFT, WindowType};



pub fn draw<P: AsRef<std::path::Path>>(wav: P, file_path: P) {
    let mut reader = hound::WavReader::open(wav).unwrap();
    let samples: Vec<f64> = reader.samples::<i16>().map(|x| x.unwrap() as f64).collect();

    let mut image_data: Vec<u8> = vec![];
    let mut image_lines = 0;

    let window_type: WindowType = WindowType::Hanning;
    let window_size: usize = 2048;
    let step_size: usize = 256;
    let mut stft = STFT::<f64>::new(window_type, window_size, step_size);
    let mut spectrogram_column: Vec<f64> = std::iter::repeat(0.).take(stft.output_size()).collect();
    let mut max = std::f64::MIN;
    let mut min = std::f64::MAX;
    let half = spectrogram_column.len() / 2;
    for some_samples in (&samples[..]).chunks(step_size as usize) {
        stft.append_samples(some_samples);
        while stft.contains_enough_to_compute() {
            stft.compute_column(&mut spectrogram_column);
            for bin in &spectrogram_column[..half] {
                max = f64::max(max, *bin);
                min = f64::min(min, *bin);
                let val = (stft::log10_positive(*bin) * std::u8::MAX as f64) as u8;
                image_data.push(val);
            }
            image_lines += 1;
            stft.move_to_next_column();
        }
    }
    println!("max: {}, min: {}", max, min);
    let spectrogram: image::GrayImage = image::ImageBuffer::from_raw(half as u32, image_lines as u32, image_data).expect("Could not create image");
    let rotated = image::imageops::rotate270(&spectrogram);
    rotated.save(file_path).expect("Could not write file");
}

#[test]
fn violin() {
    draw("Violin.wav", "violin.png")
}