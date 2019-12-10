use crate::common::utils;
use std::collections::{HashMap, BTreeMap};
use num::traits::float::Float;
use std::f32::consts::PI;

pub fn part1() {
    println!("Running day10 part1!");
    let mut asteroids: Vec<(i32,i32)> = Vec::new();

    let (upper_x,upper_y) = map_asteroids(&mut asteroids);

    let (highest_seen,highest_seen_x,highest_seen_y) = find_asteroid_with_most_visible_asteroids(&asteroids, (upper_x, upper_y));

    println!("\tAsteroid on coordinate x={},y={} has the highest amount of seen asteroids with {} asteroids", highest_seen_x, highest_seen_y, highest_seen);

    println!("Completed day10 part1!\n");
}

pub fn part2() {
    println!("Running day10 part2!");
    let mut asteroids: Vec<(i32,i32)> = Vec::new();

    let (upper_x,upper_y) = map_asteroids(&mut asteroids);

    let (_highest_seen,origin_x,origin_y) = find_asteroid_with_most_visible_asteroids(&asteroids, (upper_x, upper_y));

    let (result_x, result_y) = shoot_200_asteroids_and_return_last_shot(&asteroids, (origin_x,origin_y), (upper_x,upper_y));

    println!("\t200th asteroid shot was on coordinate x={},y={}", result_x, result_y);

    println!("Completed day10 part2!\n");
}

fn map_asteroids(asteroids: &mut Vec<(i32,i32)>) -> (i32,i32) {
    let mut upper_x = 0;
    let mut upper_y = 0;
    for line in utils::read_lines_from_file("inputs/day10.txt") {
        let line = line.unwrap();
        let mut x = 0;
        for char in line.chars() {
            if char == '#' {
                asteroids.push((x,upper_y));
            };

            x += 1;
        }
        upper_x = x;
        upper_y += 1;
    }
    (upper_x, upper_y)
}

fn find_asteroid_with_most_visible_asteroids(asteroids: &Vec<(i32,i32)>, bounds: (i32,i32)) -> (i32,i32,i32) {
    let (upper_x,upper_y) = bounds;

    let mut highest_seen = 0;
    let mut highest_seen_x = 0;
    let mut highest_seen_y = 0;

    for key in asteroids {
        let mut seen_asteroids = 0;
        for other in asteroids {
            if *key == *other {
                continue;
            }
            let (x, y) = *key;
            let (ox, oy) = *other;
            let mut height = oy - y;
            let mut width = ox - x;

            let smallest_number = if height.abs() < width.abs() {
                height.abs()
            } else {
                width.abs()
            };

            if width == 0 {
                height /= height.abs();
            } else if height == 0 {
                width /= width.abs();
            } else {
                for i in (2..smallest_number + 1).rev() {
                    if height % i == 0 && width % i == 0 {
                        height /= i;
                        width /= i;
                    }
                }
            }

            let mut traversal_x = x + width;
            let mut traversal_y = y + height;
            let mut vision_blocked = false;
            while traversal_x >= 0 && traversal_x <= upper_x &&
                traversal_y >= 0 && traversal_y <= upper_y {
                if asteroids.contains(&(traversal_x,traversal_y)) {
                    if traversal_x == ox && traversal_y == oy {
                        seen_asteroids += 1;
                    } else {
                    }
                    break;
                }
                traversal_x += width;
                traversal_y += height;
            }
        }
        if seen_asteroids > highest_seen {
            let (x,y) = *key;

            highest_seen = seen_asteroids;
            highest_seen_x = x;
            highest_seen_y = y;
        }
    }
    (highest_seen,highest_seen_x,highest_seen_y)
}

fn shoot_200_asteroids_and_return_last_shot(asteroids: &Vec<(i32,i32)>, origin_asteroid: (i32,i32), bounds: (i32,i32)) -> (i32,i32) {
    let (upper_x, upper_y) = bounds;
    let (origin_x, origin_y) = origin_asteroid;

    let mut angle_map: BTreeMap<(i32,i32),(i32,i32)> = BTreeMap::new();

    for asteroid in asteroids {
        let (asteroid_x,asteroid_y) = *asteroid;
        if asteroid_x == origin_x && asteroid_y == origin_y {
            continue;
        }
        let x_difference = (asteroid_x-origin_x) as f32;
        let y_difference = (asteroid_y-origin_y) as f32;
        let mut angle = y_difference.atan2(x_difference);
        angle += PI/2.0;
        angle = angle.to_degrees();
        if angle < 0f32 {
            angle += 360f32;
        }

        let manhatten_distance = (x_difference.abs() + y_difference.abs()) as i32;

        angle_map.insert(((angle * 100f32) as i32, manhatten_distance), (asteroid_x, asteroid_y));
    }

    let mut asteroids_shot: Vec<(i32,i32)> = Vec::new();

    let mut last_angle_shot = -1;
    let mut asteroid_nr_200 = (0,0);
    while asteroid_nr_200 == (0,0) {
        for asteroid_key in angle_map.keys() {
            let asteroid = angle_map.get(asteroid_key).unwrap();
            let (x,y) = *asteroid;
            let (angle,_distance) = *asteroid_key;

            if last_angle_shot != angle && !asteroids_shot.contains(asteroid) {
                last_angle_shot = angle;
                asteroids_shot.push(*asteroid);
                if asteroids_shot.len() == 200 {
                    asteroid_nr_200 = *asteroid;
                    break;
                }
            }
        }
    }

    asteroid_nr_200
}
