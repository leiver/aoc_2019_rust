use crate::common::utils;
use crate::common::intcode::intcode;
use std::collections::{HashMap, VecDeque};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;
use std::thread;

pub fn part1() {
    println!("Running day15 part1!");

    let mut program = utils::read_whole_file("inputs/day15.txt");

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

    loop {
        let mut default_map_tile = (-1,-1,-1,-1);
        let (north,east,south,west) = map.entry((x,y)).or_insert(default_map_tile);
        //println!("at coordinate x={},y={} with current directions north={},east={},south={},west={}, and previous direction={}", x, y, north, east, south, west, previous_direction);

        if previous_direction == 0 {
            *north = 1;
        } else if previous_direction == 1 {
            *east = 1;
        } else if previous_direction == 2 {
            *south = 1;
        } else if previous_direction == 3 {
            *west = 1;
        } else {
            previous_direction = 3;
        }

        if x == 0 && y == 0 && *north != -1 && *east != -1 && *south != -1 && *west != -1 {
            break;
        }

        for i in 1..5 {
            let next_direction = (previous_direction + i) % 4;
            if next_direction == 0 && *north != 0 {
                t_input.send(1);
                let output = r_output.recv().unwrap();
                *north = output as i32;
                if output != 0 {
                    y -= 1;
                    if output == 2 {
                        oxygen_system = (x,y);
                    }
                    previous_direction = 2;
                   // println!("\tgoing north");
                    break;
                } else {
                    //println!("\twall north");
                }
            } else if next_direction == 1 && *east != 0 {
                t_input.send(4);
                let output = r_output.recv().unwrap();
                *east = output as i32;
                if output != 0 {
                    x += 1;
                    if output == 2 {
                        oxygen_system = (x,y);
                    }
                    previous_direction = 3;
                    //println!("\tgoing east");
                    break;
                } else {
                    //println!("\twall east");
                }
            } else if next_direction == 2 && *south != 0 {
                t_input.send(2);
                let output = r_output.recv().unwrap();
                *south = output as i32;
                if output != 0 {
                    y += 1;
                    if output == 2 {
                        oxygen_system = (x,y);
                    }
                    previous_direction = 0;
                    //println!("\tgoing south");
                    break;
                } else {
                    //println!("\twall south");
                }
            } else if next_direction == 3 && *west != 0 {
                t_input.send(3);
                let output = r_output.recv().unwrap();
                *west = output as i32;
                if output != 0 {
                    x -= 1;
                    if output == 2 {
                        oxygen_system = (x,y);
                    }
                    previous_direction = 1;
                    //println!("\tgoing west");
                    break;
                } else {
                    //println!("\twall west");
                }
            }
        }
    }

    let mut path: VecDeque<((i32,i32),i32,i32)> = VecDeque::new();
    path.push_back(((0,0),0,-1));

    loop {
        let ((x,y),steps,previous_direction) = path.pop_front().unwrap();
        if x == oxygen_system.0 && y == oxygen_system.1 {
            println!("steps to oxygen system = {}", steps);
            break;
        }
        let (north,east,south,west) = *map.get(&(x,y)).unwrap();

        for i in 0..4 {
            if i == previous_direction {
                continue;
            }
            if i == 0 && north != 0 {
                path.push_back(((x,y-1),steps+1,2))
            } else if i == 1 && east != 0 {
                path.push_back(((x+1,y),steps+1,3))
            } else if i == 2 && south != 0 {
                path.push_back(((x,y+1),steps+1,0))
            } else if i == 3 && west != 0 {
                path.push_back(((x-1,y),steps+1,1))
            }
        }
    }

    println!("Completed day15 part1!\n");
}

pub fn part2() {
    println!("Running day15 part2!");

    let mut program = utils::read_whole_file("inputs/day15.txt");

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

    loop {
        let mut default_map_tile = (-1,-1,-1,-1);
        let (north,east,south,west) = map.entry((x,y)).or_insert(default_map_tile);
        //println!("at coordinate x={},y={} with current directions north={},east={},south={},west={}, and previous direction={}", x, y, north, east, south, west, previous_direction);

        if previous_direction == 0 {
            *north = 1;
        } else if previous_direction == 1 {
            *east = 1;
        } else if previous_direction == 2 {
            *south = 1;
        } else if previous_direction == 3 {
            *west = 1;
        } else {
            previous_direction = 3;
        }

        if x == 0 && y == 0 && *north != -1 && *east != -1 && *south != -1 && *west != -1 {
            break;
        }

        for i in 1..5 {
            let next_direction = (previous_direction + i) % 4;
            if next_direction == 0 && *north != 0 {
                t_input.send(1);
                let output = r_output.recv().unwrap();
                *north = output as i32;
                if output != 0 {
                    y -= 1;
                    if output == 2 {
                        oxygen_system = (x,y);
                    }
                    previous_direction = 2;
                    // println!("\tgoing north");
                    break;
                } else {
                    //println!("\twall north");
                }
            } else if next_direction == 1 && *east != 0 {
                t_input.send(4);
                let output = r_output.recv().unwrap();
                *east = output as i32;
                if output != 0 {
                    x += 1;
                    if output == 2 {
                        oxygen_system = (x,y);
                    }
                    previous_direction = 3;
                    //println!("\tgoing east");
                    break;
                } else {
                    //println!("\twall east");
                }
            } else if next_direction == 2 && *south != 0 {
                t_input.send(2);
                let output = r_output.recv().unwrap();
                *south = output as i32;
                if output != 0 {
                    y += 1;
                    if output == 2 {
                        oxygen_system = (x,y);
                    }
                    previous_direction = 0;
                    //println!("\tgoing south");
                    break;
                } else {
                    //println!("\twall south");
                }
            } else if next_direction == 3 && *west != 0 {
                t_input.send(3);
                let output = r_output.recv().unwrap();
                *west = output as i32;
                if output != 0 {
                    x -= 1;
                    if output == 2 {
                        oxygen_system = (x,y);
                    }
                    previous_direction = 1;
                    //println!("\tgoing west");
                    break;
                } else {
                    //println!("\twall west");
                }
            }
        }
    }

    let mut path: VecDeque<((i32,i32),i32,i32)> = VecDeque::new();
    path.push_back(((oxygen_system.0,oxygen_system.1),0,-1));
    let mut parsedCoordinates: Vec<(i32,i32)> = Vec::new();
    parsedCoordinates.push((oxygen_system.0,oxygen_system.1));

    let mut minutes = 0;
    loop {
        if path.is_empty() {
            break;
        }
        let ((x,y),steps,previous_direction) = path.pop_front().unwrap();
        minutes = steps;
        let (north,east,south,west) = *map.get(&(x,y)).unwrap();

        for i in 0..4 {
            if i == previous_direction {
                continue;
            }
            if i == 0 && north != 0 {
                let coordinates = (x,y-1);
                if !parsedCoordinates.contains(&coordinates) {
                    path.push_back((coordinates,steps+1,2));
                    parsedCoordinates.push(coordinates);
                }
            } else if i == 1 && east != 0 {
                let coordinates = (x+1,y);
                if !parsedCoordinates.contains(&coordinates) {
                    path.push_back((coordinates, steps + 1, 3));
                    parsedCoordinates.push(coordinates);
                }
            } else if i == 2 && south != 0 {
                let coordinates = (x,y+1);
                if !parsedCoordinates.contains(&coordinates) {
                    path.push_back((coordinates, steps + 1, 0));
                    parsedCoordinates.push(coordinates);
                }
            } else if i == 3 && west != 0 {
                let coordinates = (x-1,y);
                if !parsedCoordinates.contains(&coordinates) {
                    path.push_back((coordinates, steps + 1, 1));
                    parsedCoordinates.push(coordinates);
                }
            }
        }
    }

    println!("\tminutes for oxygen to spread = {}", minutes);
    println!("Completed day15 part2!\n");
}
