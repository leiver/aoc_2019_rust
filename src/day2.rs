use crate::common::utils;

use std::collections::HashMap;

pub fn part1() {
    println!("Running day2 part1!");
    let contents = utils::read_whole_file("inputs/day2.txt");

    let mut program: HashMap<i32, i32> = HashMap::new();
    let mut index = 0;
    for value in contents.split(",") {
        program.insert(index, value.parse::<i32>().unwrap());
        index += 1;
    }

    index = 0;
    while *program.entry(index).or_insert(0) != 99 {
        let op_code = *program.entry(index).or_insert(0);
        let first_number_index = *program.entry(index + 1).or_insert(0);
        let first_number = *program.entry(first_number_index).or_insert(0);
        let second_number_index = *program.entry(index + 2).or_insert(0);
        let second_number = *program.entry(second_number_index).or_insert(0);
        let output_index = *program.entry(index + 3).or_insert(0);

        if op_code == 1 {
            program.insert(output_index, first_number + second_number);
        } else if op_code == 2 {
            program.insert(output_index, first_number * second_number);
        } else {
            panic!("expected op_code to be 1 or 2 on index {}", index)
        }

        index += 4;
    }

    println!("\tProgram finished, result in index 0 is: {}", program[&0]);
    println!("Completed day2 part1!\n");
}

pub fn part2() {
    println!("Running day2 part2!");
    for noun in 0..99 {
        for verb in 0..99 {
            let contents = utils::read_whole_file("inputs/day2.txt");
            let mut program: HashMap<i32, i32> = HashMap::new();
            initialize_hash_map(&mut program, contents);
            program.insert(1, noun);
            program.insert(2, verb);

            let result = run_program_and_return_index_0(program);
            if result == 19690720 {
                println!("\tnoun={} and verb={}", noun, verb);
                break;
            }
        }
    }
    println!("Completed day2 part2!\n");
}

fn run_program_and_return_index_0(mut program: HashMap<i32,i32>) -> i32 {
    let mut index = 0;
    while *program.entry(index).or_insert(0) != 99 {
        let op_code = get_value_from_hash_map_with_0_default(&mut program, index);
        let first_number = use_value_as_index_from_hash_map_with_0_default(&mut program, index+1);
        let second_number = use_value_as_index_from_hash_map_with_0_default(&mut program, index+2);
        let output_index = get_value_from_hash_map_with_0_default(&mut program, index+3);

        if op_code == 1 {
            program.insert(output_index, first_number + second_number);
        } else if op_code == 2 {
            program.insert(output_index, first_number * second_number);
        } else {
            panic!("expected op_code to be 1 or 2 on index {}", index)
        }

        index += 4;
    }
    program[&0]
}

fn initialize_hash_map(program: &mut HashMap<i32,i32>, contents: String) {
    let mut index = 0;
    for value in contents.split(",") {
        program.insert(index, value.parse::<i32>().unwrap());
        index += 1;
    }
}

fn get_value_from_hash_map_with_0_default(map: &mut HashMap<i32, i32>, index: i32) -> i32 {
    *map.entry(index).or_insert(0)
}

fn use_value_as_index_from_hash_map_with_0_default(map: &mut HashMap<i32, i32>, index: i32) -> i32 {
    let new_index = *map.entry(index).or_insert(0);
    *map.entry(new_index).or_insert(0)
}

