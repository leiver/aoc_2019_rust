use crate::common::intcode::intcode;
use crate::common::utils;
use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;
use std::thread;

pub fn part1() {
    println!("Running day11 part1!");

    let program = utils::read_whole_file("inputs/day11.txt");

    let output = hull_painting_robot(program, 0, false);

    println!("\tThe robot painted {} panels", output);

    println!("Completed day11 part1!\n");
}

pub fn part2() {
    println!("Running 11 part2!");

    let program = utils::read_whole_file("inputs/day11.txt");

    let output = hull_painting_robot(program, 1, true);

    println!("\tOutput in outputs/day11.png");

    println!("Completed day11 part2!\n");
}

fn hull_painting_robot(program: String, first_input: i64, create_image: bool) -> i32 {
    let (t_input, r_input) = mpsc::channel();
    let (t_output, r_output) = mpsc::channel();
    let t_output_final = mpsc::Sender::clone(&t_output);

    thread::spawn(move || {
        intcode(program, r_input, t_output);
        t_output_final.send(-1);
    });

    let mut panels: HashMap<(i32,i32),i64> = HashMap::new();

    let mut max_x = 0;
    let mut min_x = 0;
    let mut max_y = 0;
    let mut min_y = 0;

    let mut x = 0;
    let mut y = 0;

    let mut direction = (0,-1);
    let mut panels_painted = 0;

    panels.insert((0,0),first_input);

    loop {
        let mut was_painted = panels.contains_key(&(x,y));
        let mut panel = *panels.entry((x,y)).or_insert(0);

        t_input.send(panel);

        let mut output = r_output.recv().unwrap();

        if output == -1 {
            break;
        } else {
            panel = output;
            panels.insert((x,y), panel);
            if !was_painted {
                panels_painted += 1;
            }
        }

        output = r_output.recv().unwrap();

        if output == -1 {
            break;
        } else {
            let (mut direction_x,mut direction_y) = direction;
            if direction_x != 0 {
                if (output == 0 && direction_x == -1) || (output == 1 && direction_x == 1) {
                    direction_y = 1;
                } else {
                    direction_y = -1;
                }
                direction_x = 0;
            } else {
                if (output == 0 && direction_y == -1) || (output == 1 && direction_y == 1) {
                    direction_x = -1;
                } else {
                    direction_x = 1;
                }
                direction_y = 0;
            }
            direction = (direction_x,direction_y);
            x += 1 * direction_x;
            y += 1 * direction_y;
            if x > max_x {
                max_x = x;
            } else if x < min_x {
                min_x = x;
            }
            if y > max_y {
                max_y = y;
            } else if y < min_y {
                min_y = y;
            }
        }
    }
    if create_image {
        let mut pixels: Vec<i32> = Vec::new();
        for y in min_y..max_y + 1 {
            for x in min_x..max_x + 1 {
                let mut panel = *panels.entry((x, y)).or_insert(0);
                pixels.push(panel as i32);
            }
        }
        utils::create_image_file("outputs/day11.png", &pixels, ((max_x+1) - min_x) as u32, ((max_y+1) - min_y) as u32);
    }
    panels_painted
}
