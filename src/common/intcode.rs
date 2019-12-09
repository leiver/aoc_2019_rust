use std::sync::mpsc::{Receiver, Sender};
use std::option::Option;
use std::collections::HashMap;


pub fn intcode(mut contents: String, rx: Receiver<i64>, tx: Sender<i64>) -> i64 {
    let mut input_index: i64 = 0;

    let mut program: HashMap<i64, i64> = HashMap::new();
    let mut index: i64 = 0;
    for value in contents.split(",") {
        let value = value.parse::<i64>().unwrap();
        //println!("value={}", value);
        program.insert(index, value);
        index += 1;
    }

    let mut output: i64 = 0;

    index = 0;
    let mut base_index = 0;
    while *program.entry(index).or_insert(0) != 99 {
        let mut operation = *program.entry(index).or_insert(0);
        index += 1;

        let op_code = operation % 100;
        operation /= 100;
        let mut first_param_mode = 0;
        let mut second_param_mode = 0;
        let mut third_param_mode = 0;
        if operation > 0 {
            first_param_mode = operation % 10;
            operation /= 10;
        }
        if operation > 0 {
            second_param_mode = operation % 10;
            operation /= 10;
        }
        if operation > 0 {
            third_param_mode = operation % 10;
        }

        let mut first_param_index = 0;
        let mut second_param_index = 0;
        let mut third_param_index = 0;

        if first_param_mode == 0 {
            first_param_index = *program.entry(index).or_insert(0);
        } else if first_param_mode == 1 {
            first_param_index = index;
        } else {
            first_param_index = base_index + *program.entry(index).or_insert(0);
        }
        index += 1;

        if op_code < 3 || (op_code > 6 && op_code < 9) {
            if second_param_mode == 0 {
                second_param_index = *program.entry(index).or_insert(0);
            } else if second_param_mode == 1 {
                second_param_index = index;
            } else {
                second_param_index = base_index + *program.entry(index).or_insert(0);
            }
            index += 1;

            if third_param_mode == 0 {
                third_param_index = *program.entry(index).or_insert(0);
            } else if third_param_mode == 1 {
                third_param_index = index;
            } else {
                third_param_index = base_index + *program.entry(index).or_insert(0);
            }
            index += 1;

            let first_number = *program.entry(first_param_index).or_insert(0);
            let second_number = *program.entry(second_param_index).or_insert(0);

            let result = if op_code == 1 {
                first_number + second_number
            } else if op_code == 2 {
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
            println!("\tinput: {}", input);

            program.insert(first_param_index, input);
        } else if op_code == 4 {
            output = *program.get(&first_param_index).unwrap_or(&0);
            println!("\toutput: {}", output);
            tx.send(output);
        } else if op_code == 9 {
            let number = *program.entry(first_param_index).or_insert(0);
            base_index += number;
        } else if op_code == 5 || op_code == 6 {
            let first_number = *program.entry(first_param_index).or_insert(0);
            let jump = if op_code == 5 {
                first_number != 0
            } else {
                first_number == 0
            };

            if second_param_mode == 0 {
                second_param_index = *program.entry(index).or_insert(0);
            } else if second_param_mode == 1 {
                second_param_index = index;
            } else {
                second_param_index = base_index + *program.entry(index).or_insert(0);
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