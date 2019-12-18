use crate::common::utils;

pub fn part1() {
    println!("Running day16 part1!");

    let initial_input = utils::read_whole_file("inputs/day16.txt");

    let mut input: Vec<i32> = Vec::new();
    for char in initial_input.chars() {
        input.push(char.to_string().parse::<i32>().unwrap());
    }



    println!("Completed day16 part1!\n");
}

pub fn part2() {
    println!("Running day16 part2!");

    println!("Completed day16 part2!\n");
}
