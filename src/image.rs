use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::Path;

use png::HasParameters;

#[allow(dead_code)]
pub fn write_to_png(path_to_file: &str, data: &mut [u8], width: u32, height: u32) {
    let path = Path::new(path_to_file);
    let display = path.display();

    let file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&data).unwrap();
}

#[allow(dead_code)]
pub fn write_to_ppm(path_to_file: &str, data: &mut [u8], width: u32, height: u32) {
    let path = Path::new(path_to_file);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    write!(file, "P3\n{} {}\n255\n", width, height).unwrap();
    for x in 0..height {
        for y in 0..width {
            write!(
                file,
                "{} {} {} ",
                data[(x * width + y * 3) as usize],
                data[(x * width + y * 3 + 1) as usize],
                data[(x * width + y * 3 + 2) as usize]
            )
            .unwrap();
        }
        write!(file, "\n").unwrap();
    }
}

pub fn read_png(path_to_file: &str) -> (Vec<u8>, usize, usize){
    let path = Path::new(path_to_file);
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };
    let decoder = png::Decoder::new(file);
    let (info, mut reader) = decoder.read_info().unwrap();
    // Allocate the output buffer.
    let mut buf = vec![0; info.buffer_size()];
    // Read the next frame. Currently this function should only called once.
    // The default options
    reader.next_frame(&mut buf).unwrap();
    (buf, info.width as usize, info.height as usize)
}