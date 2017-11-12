extern crate hound;
extern crate image;
extern crate stft;
use stft::{STFT, WindowType};


const BINS: u32 = 500;

pub fn draw<P: AsRef<std::path::Path>>(wav: P, image: P) {

/*
    let mut reader = hound::WavReader::open(wav).unwrap();
    let sample_rate = reader.spec().sample_rate;
    let samples: Vec<f64> = reader.samples().map(|x: ()| x.unwrap() as f64).collect();
    */
    let sample_rate: usize = 44100;
    let seconds: usize = 10;
    let sample_count = sample_rate * seconds;
    let samples = (0..sample_count).map(|x| x as f64).collect::<Vec<f64>>();

    let mut image_data: Vec<u8> = vec![];
    let mut image_lines = 0;

    let window_type: WindowType = WindowType::Hanning;
    let window_size: usize = 1024;
    let step_size: usize = 512;
    let mut stft = STFT::<f64>::new(window_type, window_size, step_size);
    let mut spectrogram_column: Vec<f64> =
        std::iter::repeat(0.).take(stft.output_size()).collect();
    for some_samples in (&samples[..]).chunks(BINS as usize) {
        stft.append_samples(some_samples);
        while stft.contains_enough_to_compute() {
            stft.compute_column(&mut spectrogram_column[..]);
            if some_samples.len() == window_size { // TODO: Remove this hack
                for bin in &spectrogram_column {
                    println!("{}", bin);
                    image_data.push((bin / 1000.) as u8); // TODO: replace 1000 with something?
                }
                image_lines += 1;
            }
            stft.move_to_next_column();
        }
    }
    image::save_buffer(image, &image_data[..], BINS, image_lines, image::ColorType::Gray(8)).unwrap();
}

/*
#[test]
fn it_works() {
    draw("TTY.wav", "image.png")
}
*/

#[test]
fn violin() {
    draw("Violin.wav", "image2.png")
}