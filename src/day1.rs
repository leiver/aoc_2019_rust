use crate::common::utils;

pub fn part1() {
    println!("Running day1 part1!");
    let mut sum_fuel = 0;

    let contents = utils::read_whole_file("inputs/day1.txt");

    for mass in contents.split(",") {
        let mass_number: i32 = mass.parse().unwrap();

        sum_fuel += (mass_number / 3) - 2;
    }

    println!("\tsum of fuel = {}", sum_fuel);
    println!("Completed day1 part1!\n");
}

pub fn part2() {
    println!("Running day1 part2!");
    let contents = utils::read_whole_file("inputs/day1.txt");

    let mut sum_fuel: i32 = 0;
    for mass in contents.split(",") {
        sum_fuel += calculate_total_amount_of_fuel_from_mass(mass.parse().unwrap())
    }

    println!("\tsum of fuel = {}", sum_fuel);
    println!("Completed day1 part1!\n");
}

fn calculate_total_amount_of_fuel_from_mass(mass: i32) -> i32 {
    let mut sum_fuel: i32 = calculate_fuel_requirement_from_mass(mass);

    let mut fuel_added: i32 = sum_fuel;
    while fuel_added > 0 {
        fuel_added = calculate_fuel_requirement_from_mass(fuel_added);
        sum_fuel += fuel_added;
    }

    sum_fuel
}

fn calculate_fuel_requirement_from_mass(mass: i32) -> i32 {
    let fuel_requirement: i32 = (mass / 3) - 2;

    if fuel_requirement > 0 {
        fuel_requirement
    } else {
        0
    }
}
