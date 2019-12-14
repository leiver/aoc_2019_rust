use crate::common::utils;
use voca_rs;
use std::collections::HashMap;

pub fn part1() {
    println!("Running day14 part1!");

    let mut reaction_map: HashMap<&str, (i32,i32,&str)> = HashMap::new();

    for line in utils::read_lines_from_file("inputs/day14.txt") {
        let line = line.unwrap().as_str();

        let reaction_result = voca_rs::split::split(line, " => ")[1];
        let reaction_result_code = voca_rs::split::split(reaction_result.trim(), " ")[1];
        let reaction_result_amount = voca_rs::split::split(reaction_result.trim(), " ")[0].parse::<i32>().unwrap();
        let reaction_inputs = voca_rs::split::split(line, " => ")[0];

        reaction_map.insert(reaction_result_code, (reaction_result_amount, 0, reaction_inputs));
    }

    println!("Completed day14 part1!\n");
}

pub fn part2() {
    println!("Running day14 part2!");

    println!("Completed day14 part1!\n");
}

fn get_amount_of_ore_needed(code: &str, mut amount_needed: i32, reaction_map: &mut HashMap<&str, (i32,i32,&str)>) {
    let (reaction_amount, leftovers, reaction_input) = *reaction_map.get(code).unwrap();
    amount_needed -= leftovers;

    for input in voca_rs::split(reaction_input.trim(), ", ") {

    }

}