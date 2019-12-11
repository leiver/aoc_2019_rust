use std::fs::File;
use std::fs;
use std::io::{BufRead, BufReader, Lines};
use image;
use image::{Rgb, RgbImage};

pub fn read_whole_file(file_name: &str) -> String{
    fs::read_to_string(file_name)
        .expect("Something went wrong reading the file")
}

pub fn read_lines_from_file(file_name: &str) -> Lines<BufReader<File>> {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    reader.lines()
}

pub fn create_image_file(file_name: &str, pixels: &Vec<i32>, width: u32, heigth: u32) {
    let mut image_buffer = image::ImageBuffer::new(width, heigth);

    for x in 0..width {
        for y in 0..heigth {
            let pixel = pixels[((y*width) + x) as usize];
            let u8_pixel: u8 = if pixel == 0 {
                1u8
            } else {
                0u8
            };
            image_buffer.put_pixel(x as u32, y as u32, Rgb([u8_pixel * u8::max_value(),u8_pixel * u8::max_value(),u8_pixel * u8::max_value()]))
        }
    }

    image_buffer.save(file_name).unwrap();

}