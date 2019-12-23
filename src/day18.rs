use crate::common::utils;
use std::collections::HashMap;

pub fn part1() {
    println!("Running day18 part1!");

    let mut map = HashMap::new();
    let mut origin = (0,0);

    let mut x = 0;
    let mut y = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut key_count = 0;
    for line in utils::read_lines_from_file("inputs/day18.txt") {
        x = 0;
        let line = line.unwrap();
        for character in line.chars() {
            if character == '@' {
                origin = (x,y);
            } else if character.is_ascii_lowercase() {
                key_count += 1;
            }
            map.insert((x,y),character);

            x += 1;
            if max_x < x {
                max_x = x;
            }
        }
        y += 1;
        if max_y < y {
            max_y = y;
        }
    }



    let mut minimum_total_steps = -1;





    for y in 0..max_y {
        for x in 0..max_x {
            print!("{}", map.get(&(x,y)).unwrap());
        }
        println!();
    }

    println!("Completed day18 part1!\n");
}

pub fn part2() {
    println!("Running day18 part2!");
    println!("Completed day18 part2!\n");
}
