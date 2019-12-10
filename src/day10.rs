use crate::common::utils;
use std::collections::HashMap;

pub fn part1() {
    println!("Running day10 part1!");
    let mut map: HashMap<(i32,i32),i32> = HashMap::new();

    let mut upper_x = 0;
    let mut upper_y = 0;
    for line in utils::read_lines_from_file("inputs/day10.txt") {
        let line = line.unwrap();
        let mut x = 0;
        for char in line.chars() {
            if char == '#' {
                map.insert((x,upper_y), 0);
            };

            x += 1;
        }
        upper_x = x;
        upper_y += 1;
    }


    let mut highest_seen = 0;
    let mut highest_seen_x = 0;
    let mut highest_seen_y = 0;
    for key in map.keys() {
        let mut asteroid = 0;
        for other in map.keys().clone() {
            if *key == *other {
                continue;
            }
            let other_asteroid = map.get(other).unwrap();
            let (x,y) = *key;
            let (ox,oy) = *other;
            let mut height = y - oy;
            let mut width = x - ox;

            let smallest_number = if height.abs() < width.abs() {
                height.abs()
            } else {
                width.abs()
            };

            for i in (2..smallest_number+1).rev() {
                if height % i == 0 && width % i == 0 {
                    height /= i;
                    width /= i;
                }
            }


            let mut traversal_x = x + width;
            let mut traversal_y = y + height;
            let mut vision_blocked = false;
            while traversal_x >= 0 && traversal_x <= upper_x &&
                traversal_y >= 0 && traversal_y <= upper_y {
                if map.contains_key(&(traversal_x,traversal_y)) {
                    println!("\they");
                    if traversal_x == ox && traversal_y == oy {
                        println!("\t\theyhey");
                        asteroid += 1;
                    } else {
                        break;
                    }
                }
                traversal_x += width;
                traversal_y += height;
            }
        }
        if asteroid > highest_seen {
            highest_seen = asteroid;
            let (x,y) = *key;
            highest_seen_x = x;
            highest_seen_y = y;
        }
    }

    println!("\tAsteroid on coordinate x={},y={} has the highest amount of seen asteroids with {} asteroids", highest_seen_x, highest_seen_y, highest_seen);

    println!("Completed day10 part1!\n");
}

pub fn part2() {
    println!("Running day10 part2!");

    println!("Completed day10 part2!\n");
}