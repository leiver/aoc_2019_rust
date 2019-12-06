use std::collections::HashMap;

use crate::common::utils;

pub fn part1() {
    println!("Running day5 part1!");
    let contents = utils::read_whole_file("inputs/day5.txt");

    let mut program: HashMap<i32, i32> = HashMap::new();
    let mut index = 0;
    for value in contents.split(",") {
        program.insert(index, value.parse::<i32>().unwrap());
        index += 1;
    }

    let input = 1;

    index = 0;
    while *program.entry(index).or_insert(0) != 99 {
        let mut operation = *program.entry(index).or_insert(0);
        index += 1;

        let op_code = operation % 100;
        operation /= 100;
        let mut first_param_mode = 0;
        let mut second_param_mode = 0;
        let third_param_mode = 0;
        if operation > 0 {
            first_param_mode = operation % 10;
            operation /= 10;
        }
        if operation > 0 {
            second_param_mode = operation % 10;
        }

        let mut first_param_index = 0;
        let mut second_param_index = 0;
        let mut third_param_index = 0;

        if first_param_mode == 0 {
            first_param_index = *program.entry(index).or_insert(0);
        } else {
            first_param_index = index;
        }
        index += 1;
        if op_code < 3 {
            if second_param_mode == 0 {
                second_param_index = *program.entry(index).or_insert(0);
            } else {
                second_param_index = index;
            }
            index += 1;
            third_param_index = *program.entry(index).or_insert(0);

            index += 1;

            let first_number = *program.entry(first_param_index).or_insert(0);
            let second_number = *program.entry(second_param_index).or_insert(0);

            let result = if op_code == 1 {
                first_number + second_number
            } else {
                first_number * second_number
            };

            program.insert(third_param_index, result);
        } else if op_code == 3 {
            program.insert(first_param_index, input);
        } else if op_code == 4 {
            println!("\tprogram output: {}", program.get(&first_param_index).unwrap_or(&0));
        } else {
            panic!("expected op_code to be 1 or 2 on index {}", index);
        }
    }

    println!("\tProgram halted");
    println!("Completed day5 part1!\n");
}

pub fn part2() {
    println!("Running day5 part2!");
    let mut contents = utils::read_whole_file("inputs/day5.txt");

    //contents = String::from("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9");

    let mut program: HashMap<i32, i32> = HashMap::new();
    let mut index = 0;
    for value in contents.split(",") {
        let value = value.parse::<i32>().unwrap();
        //println!("value={}", value);
        program.insert(index, value);
        index += 1;
    }

    let input = 5;

    index = 0;
    while *program.entry(index).or_insert(0) != 99 {
        let mut operation = *program.entry(index).or_insert(0);
        index += 1;

        let op_code = operation % 100;
        operation /= 100;
        let mut first_param_mode = 0;
        let mut second_param_mode = 0;
        let third_param_mode = 0;
        if operation > 0 {
            first_param_mode = operation % 10;
            operation /= 10;
        }
        if operation > 0 {
            second_param_mode = operation % 10;
        }

        let mut first_param_index = 0;
        let mut second_param_index = 0;
        let mut third_param_index = 0;

        if first_param_mode == 0 {
            first_param_index = *program.entry(index).or_insert(0);
        } else {
            first_param_index = index;
        }
        index += 1;

        //println!("op_code = {}", op_code);
        if op_code < 3 || op_code > 6 {
            if second_param_mode == 0 {
                second_param_index = *program.entry(index).or_insert(0);
            } else {
                second_param_index = index;
            }
            index += 1;
            third_param_index = *program.entry(index).or_insert(0);

            index += 1;

            let first_number = *program.entry(first_param_index).or_insert(0);
            let second_number = *program.entry(second_param_index).or_insert(0);

            let result = if op_code == 1 {
                //println!("\tadding {} + {}, outputting {} to {}", first_number, second_number, first_number + second_number, third_param_index);
                first_number + second_number
            } else if op_code == 2 {
                //println!("\tmultiplying {} * {}, outputting {} to {}", first_number, second_number, first_number * second_number, third_param_index);
                first_number * second_number
            } else if op_code == 7 {
                if first_number < second_number {
                    1
                } else {
                    0
                }
            } else {
                if first_number == second_number {
                    1
                } else {
                    0
                }
            };

            program.insert(third_param_index, result);
        } else if op_code == 3 {
            program.insert(first_param_index, input);
        } else if op_code == 4 {
            println!("\tprogram output: {}", program.get(&first_param_index).unwrap_or(&0));
        } else if op_code == 5 || op_code == 6 {
            let first_number = *program.entry(first_param_index).or_insert(0);
            let jump = if op_code == 5 {
                first_number != 0
            } else {
                first_number == 0
            };

            if second_param_mode == 0 {
                second_param_index = *program.entry(index).or_insert(0);
            } else {
                second_param_index = index;
            }
            index += 1;

            let second_number = *program.entry(second_param_index).or_insert(0);

            if jump {
                index = second_number;
            }
        } else {
            panic!("expected op_code to be 1 or 2 on index {}", index);
        }
    }

    println!("\tProgram halted");
    println!("Completed day5 part1!\n");
}
