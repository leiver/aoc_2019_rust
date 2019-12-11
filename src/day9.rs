use crate::common::intcode;
use crate::common::utils;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;

pub fn part1() {
    println!("Running day9 part1!");

    let mut contents = utils::read_whole_file("inputs/day9.txt");
    //contents = String::from("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
    let mut inputs: Vec<i64> = Vec::new();
    inputs.insert(0, 1);

    let (ta, ra) = mpsc::channel();
    let (tfinal, rfinal) = mpsc::channel();
    ta.send(1);
    let output = intcode::intcode(contents, ra, tfinal);

    println!("\tFinal output: {}", output);
    println!("Completed day9 part1!\n");
}

pub fn part2() {
    println!("Running day9 part2!");

    let mut contents = utils::read_whole_file("inputs/day9.txt");
    //contents = String::from("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
    let mut inputs: Vec<i64> = Vec::new();
    inputs.insert(0, 1);

    let (ta, ra) = mpsc::channel();
    let (tfinal, rfinal) = mpsc::channel();
    ta.send(2);
    let output = intcode::intcode(contents, ra, tfinal);

    println!("\tFinal output: {}", output);

    println!("Completed day9 part2!\n");
}
