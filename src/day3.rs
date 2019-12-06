use voca_rs::chop;
use std::collections::HashMap;

use crate::common::utils;

pub fn part1() {
    println!("Running day3 part1!");

    let mut first_path: String = String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72");
    let mut second_path: String = String::from("U62,R66,U55,R34,D71,R55,D58,R83");

    let mut first = true;
    for line in utils::read_lines_from_file("inputs/day3.txt") {
        let line = line.unwrap();

        if first {
            first = false;
            first_path = String::from(line);
        } else {
            second_path = String::from(line);
        }
    }

    let shortest_collision = find_closest_collision_from_two_paths(first_path, second_path);

    println!("\tshortest distance: {}", shortest_collision);
    println!("Completed day3 part1!\n");
}

pub fn part2() {
    println!("Running day3 part2!");

    let mut first_path: String = String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72");
    let mut second_path: String = String::from("U62,R66,U55,R34,D71,R55,D58,R83");

    let mut first = true;
    for line in utils::read_lines_from_file("inputs/day3.txt") {
        let line = line.unwrap();

        if first {
            first = false;
            first_path = String::from(line);
        } else {
            second_path = String::from(line);
        }
    }

    let shortest_collision = find_shortest_collision_from_two_paths(first_path, second_path);

    println!("\tshortest distance: {}", shortest_collision);
    println!("Completed day3 part1!\n");
}

fn find_shortest_collision_from_two_paths(first_path: String, second_path: String) -> i32 {
    let mut map_1: HashMap<(i32,i32),(i32,char)> = HashMap::new();
    let mut map_2: HashMap<(i32,i32),(i32,char)> = HashMap::new();

    create_map_from_path_with_shortest_steps_per_node(&mut map_1, first_path);

    create_map_from_path_with_shortest_steps_per_node(&mut map_2, second_path);

    find_collision_shortest_path(&map_1, &map_2)
}

fn find_closest_collision_from_two_paths(first_path: String, second_path: String) -> i32 {
    let mut map_1: HashMap<(i32,i32),(i32,char)> = HashMap::new();
    let mut map_2: HashMap<(i32,i32),(i32,char)> = HashMap::new();

    create_map_from_path_with_shortest_steps_per_node(&mut map_1, first_path);

    create_map_from_path_with_shortest_steps_per_node(&mut map_2, second_path);

    find_collision_closest_path(&map_1, &map_2)
}

fn create_map_from_path_with_shortest_steps_per_node(map: &mut HashMap<(i32,i32),(i32,char)>, path: String) {
    let mut x = 0;
    let mut y = 0;
    let mut steps = 0;
    for instruction in path.split(",") {
        let (direction_code, steps_to_walk, axis, modifier) = parse_instruction(instruction);

        for _steps_walked in 0..steps_to_walk {
            if !map.contains_key(&(x,y)) {
                map.insert((x,y), (steps,direction_code));
            }

            steps += 1;
            if axis == 'x' {
                x += 1*modifier;
            } else {
                y += 1*modifier;
            }
        }
    }
}

fn find_collision_shortest_path(map_1: &HashMap<(i32,i32),(i32,char)>, map_2: &HashMap<(i32,i32),(i32,char)>) -> i32 {
    let mut shortest_distance: i32 = 0;

    for (x,y) in map_1.keys() {
        if (x,y) != (&0,&0) && map_2.contains_key(&(*x,*y)) {
            let (steps_1, _nonce) = map_1.get(&(*x,*y)).unwrap();
            let (steps_2, _nonce) = map_2.get(&(*x,*y)).unwrap();
            let distance: i32 = steps_1 + steps_2;
            if distance < shortest_distance || shortest_distance == 0 {
                shortest_distance = distance;
            }
        }
    }

    shortest_distance
}

fn find_collision_closest_path(map_1: &HashMap<(i32,i32),(i32,char)>, map_2: &HashMap<(i32,i32),(i32,char)>) -> i32 {
    let mut shortest_distance: i32 = 0;

    for (x,y) in map_1.keys() {
        if (x,y) != (&0,&0) && map_2.contains_key(&(*x,*y)) {
            let distance: i32 = x.abs() + y.abs();
            if distance < shortest_distance || shortest_distance == 0 {
                shortest_distance = distance;
            }
        }
    }

    shortest_distance
}

fn parse_instruction(instruction: &str) -> (char,i32,char,i32) {
    let direction_code: String = chop::substring(&instruction, 0, 1);
    let mut direction_code_char: char = 'L';
    let steps_to_walk: i32 = chop::substring(instruction, 1, 0).parse().unwrap();

    let mut axis: char = 'x';
    let mut modifier: i32 = 1;

    if direction_code == String::from("U") {
        direction_code_char = 'U';
        axis = 'x';
        modifier = 1;
    } else if direction_code == String::from("R") {
        direction_code_char = 'R';
        axis = 'y';
        modifier = 1;
    } else if direction_code == String::from("D") {
        direction_code_char = 'D';
        axis = 'x';
        modifier = -1;
    } else {
        direction_code_char = 'L';
        axis = 'y';
        modifier = -1;
    }

    (direction_code_char, steps_to_walk, axis, modifier)
}
