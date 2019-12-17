use crate::common::utils;
use voca_rs;
use std::collections::HashMap;
use std::borrow::Borrow;
use std::fs::File;
use std::fs;
use std::io::{BufRead, BufReader, Lines};

pub fn part1<'a>() {
    println!("Running day14 part1!");

    let file = utils::read_whole_file("inputs/day14.txt");

    let mut reaction_map: HashMap<(char,char,char,char,char), (i32,i32,String)> = HashMap::new();

    let mut lines = voca_rs::split::split(file.as_str(), "\n");

    for line in lines {
        let input_output = voca_rs::split::split(line, " => ");

        let reaction_result = input_output[1];
        let reaction_result_split = voca_rs::split::split(reaction_result.trim(), " ");
        let reaction_result_code = reaction_result_split[1];
        let reaction_result_amount = reaction_result_split[0].parse::<i32>().unwrap();
        let reaction_inputs = input_output[0];

        let mut code_chars = reaction_result_code.trim().chars();
        let char1 = code_chars.next().unwrap();
        let char2_option = code_chars.next();
        let char2 = if char2_option.is_some() {char2_option.unwrap()} else {'?'};
        let char3_option = code_chars.next();
        let char3 = if char3_option.is_some() {char3_option.unwrap()} else {'?'};
        let char4_option = code_chars.next();
        let char4 = if char4_option.is_some() {char4_option.unwrap()} else {'?'};
        let char5_option = code_chars.next();
        let char5: char = if char5_option.is_some() {char5_option.unwrap()} else {'?'};
        let chart_tuple = (char1,char2,char3,char4,char5);
        //println!("putting in {:?} with values {:?}", chart_tuple, (reaction_result_amount, 0, reaction_inputs));
        reaction_map.insert(chart_tuple, (reaction_result_amount, 0, reaction_inputs.trim().to_string()));
    }
    let mut stack: Vec<((char,char,char,char,char),i32)> = Vec::new();
    stack.push((('F','U','E','L','?'), 1));

    let mut ore = 0;

    while !stack.is_empty() {
        let (code, mut amount_needed) = stack.pop().unwrap();
        //println!("processing {:?} with amount needed:{}", code, amount_needed);
        let (reaction_amount, leftovers, reaction_input) = reaction_map.entry(code).or_insert((0,0,"".to_string()));
        //println!("\tentry=reaction amount={}, leftovers={}, reaction input={}", reaction_amount, leftovers, reaction_input);
        if *leftovers >= amount_needed {
            //println!("\tleftovers {} was higher then amount needed {}, so is now reduced to {}", leftovers, amount_needed, *leftovers - amount_needed);
            *leftovers -= amount_needed;
            continue;
        }
        amount_needed -= *leftovers;
        let reaction_leftovers = if amount_needed % *reaction_amount == 0 {0} else {*reaction_amount - (amount_needed % *reaction_amount)};
        let reaction_count = (amount_needed / *reaction_amount) + ( if reaction_leftovers > 0 {1} else {0});
        //println!("\tamount needed after:{}, reaction count:{}, reaction leftovers:{}", amount_needed, reaction_count, reaction_leftovers);
        *leftovers = reaction_leftovers;

        for input in voca_rs::split::split(reaction_input.trim(), ", ") {
            //println!("\t\tinput={}", input);
            let input_split = voca_rs::split::split(input.trim(), " ");
            let input_code = input_split[1];
            let input_amount = input_split[0].parse::<i32>().unwrap();
            let input_amount_needed = input_amount * reaction_count;
            if input_code == "ORE" {
                ore += input_amount_needed;
                //println!("\t\tadding ore = {}", input_amount_needed);
            } else {
                let mut code_chars = input_code.trim().chars();
                let char1 = code_chars.next().unwrap();
                let char2_option = code_chars.next();
                let char2 = if char2_option.is_some() {char2_option.unwrap()} else {'?'};
                let char3_option = code_chars.next();
                let char3 = if char3_option.is_some() {char3_option.unwrap()} else {'?'};
                let char4_option = code_chars.next();
                let char4 = if char4_option.is_some() {char4_option.unwrap()} else {'?'};
                let char5_option = code_chars.next();
                let char5: char = if char5_option.is_some() {char5_option.unwrap()} else {'?'};
                let chart_tuple = (char1,char2,char3,char4,char5);
                //println!("\t\tputting in {:?} with value {}", chart_tuple, input_amount_needed);
                stack.push(((char1,char2,char3,char4,char5), input_amount_needed));
            }
        }
    }

    println!("\tore needed = {}", ore);

    println!("Completed day14 part1!\n");
}

pub fn part2() {
    println!("Running day14 part2!");

    let file = utils::read_whole_file("inputs/day14.txt");

    let mut reaction_map: HashMap<(char,char,char,char,char), (i32,i32,String)> = HashMap::new();

    let mut lines = voca_rs::split::split(file.as_str(), "\n");

    for line in lines {
        let input_output = voca_rs::split::split(line, " => ");

        let reaction_result = input_output[1];
        let reaction_result_split = voca_rs::split::split(reaction_result.trim(), " ");
        let reaction_result_code = reaction_result_split[1];
        let reaction_result_amount = reaction_result_split[0].parse::<i32>().unwrap();
        let reaction_inputs = input_output[0];

        let mut code_chars = reaction_result_code.trim().chars();
        let char1 = code_chars.next().unwrap();
        let char2_option = code_chars.next();
        let char2 = if char2_option.is_some() {char2_option.unwrap()} else {'?'};
        let char3_option = code_chars.next();
        let char3 = if char3_option.is_some() {char3_option.unwrap()} else {'?'};
        let char4_option = code_chars.next();
        let char4 = if char4_option.is_some() {char4_option.unwrap()} else {'?'};
        let char5_option = code_chars.next();
        let char5: char = if char5_option.is_some() {char5_option.unwrap()} else {'?'};
        let chart_tuple = (char1,char2,char3,char4,char5);
        //println!("putting in {:?} with values {:?}", chart_tuple, (reaction_result_amount, 0, reaction_inputs));
        reaction_map.insert(chart_tuple, (reaction_result_amount, 0, reaction_inputs.trim().to_string()));
    }
    let mut stack: Vec<((char,char,char,char,char),i32)> = Vec::new();
    stack.push((('F','U','E','L','?'), 1));

    let mut ore = 0;
    let mut fuel_produced = 0;

    while ore <= 1_000_000_000_000i64 {
        if stack.is_empty() {
            fuel_produced += 1;
            stack.push((('F','U','E','L','?'), 1));
        }
        let (code, mut amount_needed) = stack.pop().unwrap();
        //println!("processing {:?} with amount needed:{}", code, amount_needed);
        let (reaction_amount, leftovers, reaction_input) = reaction_map.entry(code).or_insert((0,0,"".to_string()));
        //println!("\tentry=reaction amount={}, leftovers={}, reaction input={}", reaction_amount, leftovers, reaction_input);
        if *leftovers >= amount_needed {
            //println!("\tleftovers {} was higher then amount needed {}, so is now reduced to {}", leftovers, amount_needed, *leftovers - amount_needed);
            *leftovers -= amount_needed;
            continue;
        }
        amount_needed -= *leftovers;
        let reaction_leftovers = if amount_needed % *reaction_amount == 0 {0} else {*reaction_amount - (amount_needed % *reaction_amount)};
        let reaction_count = (amount_needed / *reaction_amount) + ( if reaction_leftovers > 0 {1} else {0});
        //println!("\tamount needed after:{}, reaction count:{}, reaction leftovers:{}", amount_needed, reaction_count, reaction_leftovers);
        *leftovers = reaction_leftovers;

        for input in voca_rs::split::split(reaction_input.trim(), ", ") {
            //println!("\t\tinput={}", input);
            let input_split = voca_rs::split::split(input.trim(), " ");
            let input_code = input_split[1];
            let input_amount = input_split[0].parse::<i32>().unwrap();
            let input_amount_needed = input_amount * reaction_count;
            if input_code == "ORE" {
                ore += input_amount_needed as i64;
                //println!("\t\tadding ore = {}", input_amount_needed);
            } else {
                let mut code_chars = input_code.trim().chars();
                let char1 = code_chars.next().unwrap();
                let char2_option = code_chars.next();
                let char2 = if char2_option.is_some() {char2_option.unwrap()} else {'?'};
                let char3_option = code_chars.next();
                let char3 = if char3_option.is_some() {char3_option.unwrap()} else {'?'};
                let char4_option = code_chars.next();
                let char4 = if char4_option.is_some() {char4_option.unwrap()} else {'?'};
                let char5_option = code_chars.next();
                let char5: char = if char5_option.is_some() {char5_option.unwrap()} else {'?'};
                let chart_tuple = (char1,char2,char3,char4,char5);
                //println!("\t\tputting in {:?} with value {}", chart_tuple, input_amount_needed);
                stack.push(((char1,char2,char3,char4,char5), input_amount_needed));
            }
        }
    }

    println!("\tfuel produced = {}", fuel_produced);

    println!("Completed day14 part1!\n");
}
/*
fn get_amount_of_ore_needed<'a>(intial_code: String, mut amount_needed: i32, reaction_map: &mut HashMap<String, (i32, i32, String)>) -> i32 {
    ore
}*/