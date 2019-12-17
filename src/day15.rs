use crate::common::utils;
use crate::common::intcode::intcode;
use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;
use std::thread;

pub fn part1() {
    println!("Running day15 part1!");

    let (t_input, r_input) = mpsc::channel();
    let (t_output, r_output) = mpsc::channel();
    let t_output_final = mpsc::Sender::clone(&t_output);

    thread::spawn(move || {
        intcode(program, r_input, t_output);
        t_output_final.send(-1);
    });

    let mut map: HashMap<(i32,i32),(i32,i32,i32,i32)> = HashMap::new();

    let mut x = 0;
    let mut y = 0;
    let mut oxygen_system = (0,0);
    let mut previous_direction = -1;

    while !unexplored_tiles.is_empty() {
        let mut default_map_tile = (-1,-1,-1,-1);
        if previous_direction != -1 {
            default_map_tile[previous_direction] = 1;
        } else {
            previous_direction = 3;
        }
        let (north,east,south,west) = map.entry((x,y)).or_insert(default_map_tile);

        for i in 1..5 {
            let next_direction = (previous_direction + i) % 4;
            if next_direction == 0 && *north == -1 {
                t_input.send(1);
                let output = r_output.recv().unwrap();
                *north = output as i32;
                if output != 0 {
                    y -= 1;
                    previous_direction = 2;
                    break;
                }
            } else if next_direction == 1 && *east == -1 {
                t_input.send(1);
                let output = r_output.recv().unwrap();
                *north = output as i32;
                if output != 0 {
                    y -= 1;
                    previous_direction = 2;
                    break;
                }
            } else if next_direction == 2 {
                t_input.send(1);
                let output = r_output.recv().unwrap();
                *north = output as i32;
                if output != 0 {
                    y -= 1;
                    previous_direction = 2;
                    break;
                }
            } else if next_direction == 3 {
                t_input.send(1);
                let output = r_output.recv().unwrap();
                *north = output as i32;
                if output != 0 {
                    y -= 1;
                    previous_direction = 2;
                    break;
                }
            }
        }

    }

    println!("Completed day15 part1!\n");
}

pub fn part2() {
    println!("Running day15 part2!");
    println!("Completed day15 part1!\n");
}
