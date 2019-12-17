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

    let mut unexplored_tiles: HashMap<(i32,i32),(i32,(i32,i32,i32,i32))> = HashMap::new();
    let mut map: HashMap<(i32,i32),(i32,i32,i32,i32)> = HashMap::new();
    unexplored_tiles.insert((0,0),(-1,(-1,-1,-1,-1)));

    let mut x = 0;
    let mut y = 0;
    let mut oxygen_system = (0,0);

    while !unexplored_tiles.is_empty() {

    }

    println!("Completed day15 part1!\n");
}

pub fn part2() {
    println!("Running day15 part2!");
    println!("Completed day15 part1!\n");
}
