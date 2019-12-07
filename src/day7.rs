use crate::common::utils;
use std::collections::HashMap;
use voca_rs::chop::first;
use std::borrow::BorrowMut;
use std::thread;
use std::sync::mpsc;
use voca_rs::query::query;
use std::sync::mpsc::{Receiver, Sender};

pub fn part1() {
    println!("Running day7 part1!");

    //contents = String::from("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
/*
    let mut best_output = 0;
    for A in 0..5 {
        for B in 0..5 {
            for C in 0..5 {
                for D in 0..5 {
                    for E in 0..5 {
                        if A != B && A != C && A != D && A != E &&
                            B != C && B != D && B != E &&
                            C != D && C != E &&
                            D != E {
                            let mut output = 0;
                            let mut contents = utils::read_whole_file("inputs/day7.txt");
                            //contents = String::from("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
                            println!("\trunning A with input {}", A);
                            output = intcode_part_1(contents, A, output);
                            contents = utils::read_whole_file("inputs/day7.txt");
                            //contents = String::from("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
                            println!("\trunning B with input {}", B);
                            output = intcode_part_1(contents, B, output);
                            contents = utils::read_whole_file("inputs/day7.txt");
                            //contents = String::from("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
                            println!("\trunning C with input {}", C);
                            output = intcode_part_1(contents, C, output);
                            contents = utils::read_whole_file("inputs/day7.txt");
                            //contents = String::from("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
                            println!("\trunning D with input {}", D);
                            output = intcode_part_1(contents, D, output);
                            contents = utils::read_whole_file("inputs/day7.txt");
                            //contents = String::from("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
                            println!("\trunning E with input {}", E);
                            output = intcode_part_1(contents, E, output);

                            if best_output == 0 || best_output < output {
                                best_output = output;
                                println!("\t\touput {} from last amplifier wast best yet with amps: A={}, B={}, C={}, D={}, E={}", best_output, A, B, C, D, E);
                            }
                        }
                    }
                }
            }
        }
    }

    println!("best output = {}", best_output);
    */
    println!("Completed day7 part1!\n");
}

pub fn part2() {
    println!("Running day7 part2!");

    //contents = String::from("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");

    let mut best_output = 0;
    for A in 5..10 {
        for B in 5..10 {
            for C in 5..10 {
                for D in 5..10 {
                    for E in 5..10 {
                        if A != B && A != C && A != D && A != E &&
                            B != C && B != D && B != E &&
                            C != D && C != E &&
                            D != E {

                            let (ta, ra) = mpsc::channel();
                            let ta1 = mpsc::Sender::clone(&ta);

                            let (tb, rb) = mpsc::channel();
                            let tb1 = mpsc::Sender::clone(&tb);

                            let (tc, rc) = mpsc::channel();
                            let tc1 = mpsc::Sender::clone(&tc);

                            let (td, rd) = mpsc::channel();
                            let td1 = mpsc::Sender::clone(&td);

                            let (te, re) = mpsc::channel();
                            let te1 = mpsc::Sender::clone(&te);

                            let (tfinal, rfinal) = mpsc::channel();

                            ta1.send(A);
                            ta1.send(0);
                            tb1.send(B);
                            tc1.send(C);
                            td1.send(D);
                            te1.send(E);

                            thread::spawn(move || {
                                let mut output = 0;
                                let mut contents = utils::read_whole_file("inputs/day7.txt");
                                //contents = String::from("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5");
                                println!("\trunning A with input {}", A);
                                output = intcode_part_1(contents, ra, tb);
                                println!("\tfinished A");
                            });

                            thread::spawn(move || {
                                let mut output = 0;
                                let mut contents = utils::read_whole_file("inputs/day7.txt");
                                //contents = String::from("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5");
                                println!("\trunning B with input {}", B);
                                output = intcode_part_1(contents, rb, tc);
                                println!("\tfinished B");
                            });

                            thread::spawn(move || {
                                let mut output = 0;
                                let mut contents = utils::read_whole_file("inputs/day7.txt");
                                //contents = String::from("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5");
                                println!("\trunning C with input {}", C);
                                output = intcode_part_1(contents, rc, td);
                                println!("\tfinished C");
                            });

                            thread::spawn(move || {
                                let mut output = 0;
                                let mut contents = utils::read_whole_file("inputs/day7.txt");
                                //contents = String::from("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5");
                                println!("\trunning D with input {}", D);
                                output = intcode_part_1(contents, rd, te);
                                println!("\tfinished D");
                            });

                            thread::spawn(move || {
                                let mut output = 0;
                                let mut contents = utils::read_whole_file("inputs/day7.txt");
                                //contents = String::from("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5");
                                println!("\trunning E with input {}", E);
                                output = intcode_part_1(contents, re, ta);
                                println!("\tfinished E");
                                tfinal.send(output);
                            });

                             let output = rfinal.recv().unwrap();

                            if best_output == 0 || best_output < output {
                                best_output = output;
                                println!("\t\touput {} from last amplifier wast best yet with amps: A={}, B={}, C={}, D={}, E={}", best_output, A, B, C, D, E);
                            }
                        }
                    }
                }
            }
        }
    }

    println!("best output = {}", best_output);

    println!("Completed day7 part2!\n");
}

fn intcode_part_1(mut contents: String, rx: Receiver<i32>, tx: Sender<i32>) -> i32 {
    let mut first_input = true;

    let mut program: HashMap<i32, i32> = HashMap::new();
    let mut index = 0;
    for value in contents.split(",") {
        let value = value.parse::<i32>().unwrap();
        //println!("value={}", value);
        program.insert(index, value);
        index += 1;
    }

    let mut output: i32 = 0;

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
            let input = rx.recv().unwrap();
            program.insert(first_param_index, input);
            println!("\t\tinputting value: {}", input );
        } else if op_code == 4 {
            output = *program.get(&first_param_index).unwrap_or(&0);
            tx.send(output);
            println!("\t\tprogram output: {}", output );
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

    output
}
