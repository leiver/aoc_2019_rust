use crate::common::utils;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;
use std::thread;
use std::sync::Arc;

pub fn part1() {
    println!("Running day16 part1!");

    let initial_input = utils::read_whole_file("inputs/day16.txt");

    let mut input = Vec::new();
    for char in initial_input.chars() {
        input.push(char.to_string().parse::<i32>().unwrap());
    }

    let base_pattern = Arc::new([0,1,0,-1]);

    for i in 0..100 {
        let mut output_channels = Vec::new();
        let mut threads = Vec::new();
        let input_arc = Arc::new(input);

        for output_index in 0..input_arc.len() {
            let (t_output, r_output) = mpsc::channel();
            output_channels.push(r_output);
            let input_clone = input_arc.clone();
            let base_pattern_clone = base_pattern.clone();

            threads.push(thread::spawn(move || {
                let mut result = 0;
                for input_index in 0..input_clone.len() {
                    result += input_clone[input_index] * base_pattern_clone[((input_index+1) / (output_index+1)) % 4];
                }
                t_output.send(result.abs() % 10);
            }));
        }

        for join in threads {
            join.join();
        }
        input = Vec::new();
        for output_channel in output_channels {
            input.push(output_channel.recv().unwrap());
        }
    }

    println!("\tfirst 8 digits = {}{}{}{}{}{}{}{}", input[0], input[1], input[2], input[3], input[4], input[5], input[6], input[7]);

    println!("Completed day16 part1!\n");
}

pub fn part2() {
    println!("Running day16 part2!");

    let initial_input = utils::read_whole_file("inputs/day16.txt");

    let mut input = Vec::new();
    for i in 0..10000 {
        for char in initial_input.chars() {
            input.push(char.to_string().parse::<i32>().unwrap());
        }
    }

    let output_offset = input[0]*1000000 + input[1]*100000 + input[2]*10000 + input[3]*1000 + input[4]*100 + input[5]*10 + input[6];

    let base_pattern = Arc::new([0,1,0,-1]);

    for i in 0..100 {
        let mut output_channels = Vec::new();
        let mut threads = Vec::new();
        let input_arc = Arc::new(input);

        for output_index in 0..(input_arc.len() / 10000) {
            let (t_output, r_output) = mpsc::channel();
            output_channels.push(r_output);
            let input_clone = input_arc.clone();
            let base_pattern_clone = base_pattern.clone();

            threads.push(thread::spawn(move || {
                for j in 1..10001 {
                    let mut result = 0;
                    for input_index in 0..input_clone.len() {
                        result += input_clone[input_index] * base_pattern_clone[((input_index + 1) / ((output_index * j) + 1)) % 4];
                    }
                    t_output.send(result.abs() % 10);
                }
            }));
        }

        for join in threads {
            join.join();
        }
        input = Vec::new();
        for output_channel in output_channels {
            for j in 0..10000 {
                input.push(output_channel.recv().unwrap());
            }
        }
    }

    println!("\tfirst 8 digits = {}{}{}{}{}{}{}{}", input[0+output_offset as usize], input[1+output_offset as usize], input[2+output_offset as usize], input[3+output_offset as usize], input[4+output_offset as usize], input[5+output_offset as usize], input[6+output_offset as usize], input[7+output_offset as usize]);

    println!("Completed day16 part2!\n");
}
