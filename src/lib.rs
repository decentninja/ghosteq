extern crate hound;
extern crate image;
extern crate goertzel;


const BINS: u32 = 1000;

pub fn draw<P: AsRef<std::path::Path>>(wav: P, image: P) {
    let mut reader = hound::WavReader::open(wav).unwrap();
    let sample_rate = reader.spec().sample_rate;
    let samples: Vec<i16> = reader.samples().map(|x|x.unwrap()).collect();
    let mut image_data: Vec<u8> = vec![];
    let mut image_lines = 0;
    for chunk in samples.chunks(256) {
        if chunk.len() == 256 {
            for bin in (0 .. BINS).map(|x| (x * 256) as f32) {
                    let p = goertzel::Parameters::new(bin, sample_rate, 256);
                    let v = p.start().add(chunk).finish_mag();
                    image_data.push((v / 1000.) as u8);
                    print!("{:14.0} ", v);
            }
            println!("");
            image_lines += 1;
        }
    }
    println!("{}x{} image", BINS, image_lines);
    image::save_buffer(image, &image_data[..], BINS, image_lines, image::ColorType::Gray(8)).unwrap();
}

#[test]
fn it_works() {
    draw("TTY.wav", "image.png")
}

#[test]
fn violin() {
    draw("Violin.wav", "image2.png")
}