use crate::common::utils;
use voca_rs;
use std::ops::Index;
use image;
use image::{Rgb, RgbImage};
use std::borrow::Borrow;

const pixels_wide: usize = 25;
const pixels_tall: usize = 6;
const pixels_per_layer: usize = pixels_tall*pixels_wide;

pub fn part1() {
    println!("Running day8 part1!");
    let mut raw_image = utils::read_whole_file("inputs/day8.txt");
    //raw_image = String::from("0222112222120000");

    let pixels_in_image = voca_rs::count::count(raw_image.as_str());
    let layers = pixels_in_image / pixels_per_layer;

    let mut least_zero_digits = -1;
    let mut result = 0;
    for layer_index in 0..layers {
        let layer_array: Vec<i32> = get_array_of_layer_pixels(raw_image.as_str(), layer_index);

        let mut one_digits = 0;
        let mut two_digits = 0;
        let mut zero_digits = 0;

        for digit in layer_array {
            if digit == 0 {
                zero_digits += 1;
            } else if digit == 1 {
                one_digits += 1;
            } else if digit == 2 {
                two_digits += 1;
            }
        }

        if least_zero_digits > zero_digits || least_zero_digits == -1 {
            least_zero_digits = zero_digits;
            result = one_digits * two_digits;
        }
    }

    println!("\tresult: {}", result);
    println!("Completed day8 part1!\n");
}

pub fn part2() {
    println!("Running day8 part2!");
    let mut raw_image = utils::read_whole_file("inputs/day8.txt");
    //raw_image = String::from("0222112222120000");

    let pixels_in_image = voca_rs::count::count(raw_image.as_str());
    let layers = pixels_in_image / pixels_per_layer;

    let mut pixels: Vec<i32> = get_array_of_layer_pixels(raw_image.as_str(), layers-1);
    println!("\n\tlayer {}", layers-1);
    print_pixels(&pixels);

    for layer_index in (0..layers-1).rev() {
        let layer_array: Vec<i32> = get_array_of_layer_pixels(raw_image.as_str(), layer_index);
        println!("\n\tlayer {}", layer_index);
        print_pixels(&layer_array);

        for pixel_index in 0..pixels_per_layer {
            let pixel = layer_array.get(pixel_index).unwrap();
            if pixel != &2 {
                pixels.insert(pixel_index, *pixel);
            }
        }

    }

    println!("\n\tfinal image");
    print_pixels(&pixels);

    create_image_file("outputs/day8.png", &pixels);

    println!("Completed day8 part2!\n");
}

fn get_array_of_layer_pixels(raw_image: &str, layer_index: usize) -> Vec<i32> {
    let layer = voca_rs::chop::substring(raw_image, (layer_index*pixels_per_layer), (layer_index+1)*pixels_per_layer);
    voca_rs::split::chars(layer.as_str())
        .into_iter()
        .map( |character: &str| {character.parse::<i32>().unwrap()})
        .collect()
}

fn print_pixels(pixels: &Vec<i32>) {

    for x in 0..pixels_tall {
        print!("\t\t");
        for y in 0..pixels_wide {
            let pixel = pixels.get((x*pixels_wide) + y).unwrap();
            if pixel == &2 {
                print!("*");
            } else if pixel == &1 {
                print!("1")
            } else if pixel == &0 {
                print!("0")
            }
        }
        println!();
    }
}

fn create_image_file(file_name: &str, pixels: &Vec<i32>) {
    let mut image_buffer = image::ImageBuffer::new(25, 6);

    /*let buffer: &[u8] = pixels.into_iter()
        .map(|pixel: i32| {Rgb([pixel,pixel,pixel])})
        .collect()
        .into_boxed_slice()
        .borrow();
*/
    for x in 0..25 {
        for y in 0..6 {
            let pixel = pixels[(y*25) + x];
            let u8_pixel: u8 = if pixel == 0 {
                0u8
            } else {
                1u8
            };
            image_buffer.put_pixel(x as u32, y as u32, Rgb([u8_pixel,u8_pixel,u8_pixel]))
        }
    }

    image_buffer.save(file_name);

}