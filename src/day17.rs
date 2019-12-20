use crate::common::utils;
use crate::common::intcode::intcode;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;
use std::thread;
use std::collections::HashMap;
use std::char;
use regex::Regex;
use std::cmp::max;

pub fn part1() {
    println!("Running day17 part1!");

    let mut program = utils::read_whole_file("inputs/day17.txt");

    let (t_input, r_input) = mpsc::channel();
    let (t_output, r_output) = mpsc::channel();
    let t_output_final = mpsc::Sender::clone(&t_output);

    thread::spawn(move || {
        intcode(program, r_input, t_output);
        t_output_final.send(-1);
    });

    let mut map: HashMap<(i32,i32),char> = HashMap::new();

    let mut x = 0;
    let mut y = 0;
    loop {
        let output = r_output.recv().unwrap();
        let mut output_char = if output == 46 {
            '.'
        } else if output == 35 {
            '#'
        } else if output == 10 {
            'N'
        } else if output == 60 {
            '<'
        } else if output == 62 {
            '>'
        } else if output == 94 {
            '^'
        } else if output == 118 {
            'v'
        } else {
            'X'
        };

        if output_char == 'N' {
            x = 0;
            y += 1;
        } else if output_char == 'X' {
            break;
        } else {
            map.insert((x,y),output_char);
            x += 1;
        }
    }

    let mut sum = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    for coordinates in map.keys() {
        let tile = map.get(coordinates).unwrap();
        let (x,y) = *coordinates;
        if x > max_x {
            max_x = x;
        }
        if y > max_y {
            max_y = y;
        }
        let left = map.get(&(x - 1,y));
        let right = map.get(&(x + 1,y));
        let up = map.get(&(x,y - 1));
        let down = map.get(&(x,y + 1));
        //println!();
        if tile == &'#' &&
            map.contains_key(&(x - 1,y)) && map.get(&(x - 1,y)).unwrap() == &'#' &&
            map.contains_key(&(x + 1,y)) && map.get(&(x + 1,y)).unwrap() == &'#' &&
            map.contains_key(&(x,y - 1)) && map.get(&(x,y - 1)).unwrap() == &'#' &&
            map.contains_key(&(x,y + 1)) && map.get(&(x,y + 1)).unwrap() == &'#' {
            sum += x * y;
        }
    }

    println!("\tsum = {}", sum);

    println!("Completed day17 part1!\n");
}

pub fn part2() {
    println!("Running day17 part2!");

    let mut program = utils::read_whole_file("inputs/day17.txt");

    let (t_input, r_input) = mpsc::channel();
    let (t_output, r_output) = mpsc::channel();
    let t_output_final = mpsc::Sender::clone(&t_output);

    thread::spawn(move || {
        intcode(program, r_input, t_output);
        t_output_final.send(-1);
    });

    let mut map: HashMap<(i32,i32),char> = HashMap::new();

    let mut x = 0;
    let mut y = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut robot_origin = (0,0);
    let mut robot_original_vel = (0,0);
    loop {
        let output = r_output.recv().unwrap();
        let mut output_char = if output == 46 {
            '.'
        } else if output == 35 {
            '#'
        } else if output == 10 {
            'N'
        } else if output == 60 {
            robot_origin = (x,y);
            robot_original_vel = (-1,0);
            '<'
        } else if output == 62 {
            robot_origin = (x,y);
            robot_original_vel = (1,0);
            '>'
        } else if output == 94 {
            robot_origin = (x,y);
            robot_original_vel = (0,-1);
            '^'
        } else if output == 118 {
            robot_origin = (x,y);
            robot_original_vel = (0,1);
            'v'
        } else {
            'X'
        };

        if output_char == 'N' {
            x = 0;
            y += 1;
        } else if output_char == 'X' {
            break;
        } else if output_char != '.'{
            map.insert((x,y),output_char);
            x += 1;
        } else {
            x += 1;
        }

        if x > max_x {
            max_x = x;
        }
        if y > max_y {
            max_y = y;
        }
    }

    let mut x = robot_origin.0;
    let mut y = robot_origin.1;
    let mut vel_x = robot_original_vel.0;
    let mut vel_y = robot_original_vel.1;
    let mut steps = 0;

    let mut path: Vec<char> = Vec::new();

    loop {
        let front_tile_coords = (x+vel_x,y+vel_y);
        let front_tile = if map.contains_key(&front_tile_coords) {*map.get(&front_tile_coords).unwrap()} else {'.'};
        if front_tile != '.' {
            x += vel_x;
            y += vel_y;
            steps += 1;
            let path_char = char::from_digit(steps, 16).unwrap();
            map.insert(front_tile_coords, path_char);
        } else {
            let left_coords = if vel_x != 0 {(x,y-vel_x)} else {(x+vel_y,y)};
            let left = if map.contains_key(&left_coords) {*map.get(&left_coords).unwrap()} else {'.'};

            let right_coords = if vel_x != 0 {(x,y+vel_x)} else {(x-vel_y,y)};
            let right = if map.contains_key(&right_coords) {*map.get(&right_coords).unwrap()} else {'.'};

            if left == '.' && right == '.' {
                let steps_char = char::from_digit(steps, 16).unwrap();
                path.push(steps_char);
                break;
            } else if left != '.' {
                if vel_x != 0 {
                    vel_y = vel_x;
                    vel_y *= -1;
                    vel_x = 0;
                } else {
                    vel_x = vel_y;
                    vel_y = 0;
                }

                let steps_char = char::from_digit(steps, 16).unwrap();
                if steps_char != '0' {
                    path.push(steps_char);
                }
                path.push('L');
                map.insert((x,y), 'L');
                steps = 1;
                map.insert(left_coords, steps_char);
            } else {
                if vel_x != 0 {
                    vel_y = vel_x;
                    vel_x = 0;
                } else {
                    vel_x = vel_y;
                    vel_x *= -1;
                    vel_y = 0;
                }
                let steps_char = char::from_digit(steps, 16).unwrap();
                if steps_char != '0' {
                    path.push(steps_char);
                }
                path.push('R');
                map.insert((x,y), 'R');
                steps = 1;
                map.insert(right_coords,steps_char);
            }
            x += vel_x;
            y += vel_y;
        }
    }

    let mut path_string: String = String::new();
    let mut components: Vec<String> = Vec::new();
    let mut path_components: Vec<String> = Vec::new();

    for i in 0..(path.len()/2) {
        let steps_string = if path[(i*2)+1] == 'a' {
            String::from("10")
        } else if path[(i*2)+1] == 'b' {
            String::from("11")
        } else if path[(i*2)+1] == 'c' {
            String::from("12")
        } else {
            format!("{}",path[(i*2)+1])
        };
        path_string = format!("{}{}{}", path_string, path[i*2], steps_string);
        let component = format!("{}{}", path[i*2], steps_string);
        path_components.push(component);
        let component = format!("{}{}", path[i*2], steps_string);
        if !components.contains(&component) {
            components.push(component);
        }
    }

    let mut additional_components_to_use = 1;
    while additional_components_to_use <= path_components.len() / 2 {
        for i in 0..(path_components.len()- additional_components_to_use) {
            let mut component = format!("{}",path_components[i]);
            let mut ascii_commas = 1;
            for j in 1..(additional_components_to_use+1) {
                component = format!("{}{}", component, path_components[i+j]);
                ascii_commas += 2;
            }
            if !components.contains(&component) && (component.len()+ascii_commas) <= 20 {
                components.push(component);
            }
        }
        additional_components_to_use += 1;
    }

    //println!("{}", path_string);
    //println!("{:?}", components);

    let mut program = utils::read_whole_file("inputs/day17.txt");

    let mut program_str = program.as_str();
    let mut first_input = "2,";
    let mut program = String::from(format!("2,{}", voca_rs::chop::substring(program_str, (voca_rs::index::index_of(program_str, ",", 0) + 1 ) as usize, 0).as_str()));

    let (t_input, r_input) = mpsc::channel();
    let (t_output, r_output) = mpsc::channel();
    let t_output_final = mpsc::Sender::clone(&t_output);

    thread::spawn(move || {
        intcode(program, r_input, t_output);
        t_output_final.send(-1);
    });

    let mut functions: Option<[&str;3]> = Option::None;

    'outer: for i in 0..components.len() {
        for j in (i+1)..components.len() {
            for k in max((j+1),5)..components.len() {
                let regex = format!("^({}|{}|{})*$",components[i],components[j],components[k]);
                let reg = Regex::new(regex.as_str()).unwrap();
                if reg.is_match(path_string.as_str()) {
                    functions = Option::Some([components[i].as_str(),components[j].as_str(),components[k].as_str()]);
                    break 'outer;
                }
            }
        }
    }

    let functions = functions.unwrap();
    let mut remainder = path_string;
    let mut main_function = String::new();
    while remainder.len() > 0 {
        for i in 0..functions.len() {
            if remainder.starts_with(functions[i]) {
                main_function = format!("{}{}", main_function, if i == 0 {"A"} else if i == 1 {"B"} else {"C"});
                remainder = voca_rs::chop::substring(remainder.as_str(), functions[i].len(), 0);
                break;
            }
        }
    }

    //println!("{}", main_function);

    let mut char_iterator = main_function.chars();
    let mut first = true;

    loop {
        let char_option = char_iterator.next();
        if char_option.is_none() {
            break;
        }
        let function = char_option.unwrap();
        if !first {
            t_input.send(44);
        }
        first = false;
        t_input.send( if function == 'A' {
            65
        } else if function == 'B' {
            66
        } else {
            67
        });
    }
    t_input.send(10);

    for function in functions.iter() {
        let mut char_iterator = function.chars();
        let mut first = true;
        loop {
            let char_option = char_iterator.next();
            if char_option.is_none() {
                break;
            }
            let operation = char_option.unwrap();

            if operation == 'R' {
                if !first {
                    t_input.send(44);
                }
                first = false;
                t_input.send(82);
                t_input.send(44);
            } else if operation == 'L' {
                if !first {
                    t_input.send(44);
                }
                first = false;
                t_input.send(76);
                t_input.send(44);
            } else {
                t_input.send(operation.to_digit(10).unwrap() as i64 + 48);
            }
        }
        t_input.send(10);
    }
    t_input.send(110);
    t_input.send(10);

    let mut final_output = 0;
    loop {
        let output = r_output.recv().unwrap();
        if output == -1 {
            break;
        }
        final_output = output;
    }

    println!("\toutput = {}", final_output);

    println!("Completed day17 part2!\n");
}
