use crate::common::utils;
use crate::common::intcode::intcode;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;
use std::thread;
use std::collections::HashMap;
use std::char;

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
        println!();
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
            path.push(path_char);
        } else {
            let left_coords = if vel_x != 0 {(x,y-vel_x)} else {(x+vel_y,y)};
            let left = if map.contains_key(&left_coords) {*map.get(&left_coords).unwrap()} else {'.'};

            let right_coords = if vel_x != 0 {(x,y+vel_x)} else {(x-vel_y,y)};
            let right = if map.contains_key(&right_coords) {*map.get(&right_coords).unwrap()} else {'.'};

            if left == '.' && right == '.' {
                break; //A=L10L12R6R10L4L4L12  B=L10L12R6L10R10R6L4  C=R10L4L4L12L10R10R6L4  || AABCB
            } else if left != '.' {
                if vel_x != 0 {
                    vel_y = vel_x;
                    vel_y *= -1;
                    vel_x = 0;
                } else {
                    vel_x = vel_y;
                    vel_y = 0;
                }

                path.pop();
                path.push('L');
                map.insert((x,y), 'L');
                steps = 1;
                let steps_char = char::from_digit(steps, 16).unwrap();
                path.push(steps_char);
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
                path.pop();
                path.push('R');
                map.insert((x,y), 'R');
                steps = 1;
                let steps_char = char::from_digit(steps, 16).unwrap();
                path.push(steps_char);
                map.insert(right_coords,steps_char);
            }
            x += vel_x;
            y += vel_y;
        }
    }

    for path_char in path {
        print!("{}", path_char);
    }
    println!();

    for y in 0..max_x {
        for x in 0.. max_y {
            print!("{}", map.entry((x,y)).or_insert('.'));
        }
        println!();
    }


    println!("Completed day17 part2!\n");
}
