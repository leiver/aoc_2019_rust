use std::collections::HashMap;
use voca_rs::split;

use crate::common::utils;

pub fn part1() {
    println!("Running day6 part1!");
    let mut orbit_map: HashMap<(char,char,char),(char,char,char)> = HashMap::new();

    for line in utils::read_lines_from_file("inputs/day6.txt") {
        let line = line.unwrap();
        let split_line: Vec<_> = line.split(")").collect();
        let main_mass_char_array: Vec<&str> = split::chars(split_line[0]);
        let main_mass: (char,char,char) = (main_mass_char_array[0].chars().next().unwrap(),
                                           main_mass_char_array[1].chars().next().unwrap(),
                                           main_mass_char_array[2].chars().next().unwrap());
        let orbital_char_array: Vec<&str> = split::chars(split_line[1]);
        let orbital: (char,char,char)  = (orbital_char_array[0].chars().next().unwrap(),
                                          orbital_char_array[1].chars().next().unwrap(),
                                          orbital_char_array[2].chars().next().unwrap());

        orbit_map.insert(orbital, main_mass);
    }

    let mut amount_of_orbits = 0;
    for keys in orbit_map.keys() {
        let mut orbital = keys;

        while orbital != &('C','O','M') {
            amount_of_orbits += 1;
            orbital = orbit_map.get(orbital).unwrap();
        }
    }

    println!("\torbits: {}", amount_of_orbits);
    println!("Completed day6 part1!\n");
}

pub fn part2() {
    println!("Running day6 part2!");

    let mut orbit_map: HashMap<(char,char,char),(char,char,char)> = HashMap::new();

    for line in utils::read_lines_from_file("inputs/day6.txt") {
        let line = line.unwrap();
        let split_line: Vec<_> = line.split(")").collect();
        let main_mass_char_array: Vec<&str> = split::chars(split_line[0]);
        let main_mass: (char,char,char) = (main_mass_char_array[0].chars().next().unwrap(),
                                           main_mass_char_array[1].chars().next().unwrap(),
                                           main_mass_char_array[2].chars().next().unwrap());
        let orbital_char_array: Vec<&str> = split::chars(split_line[1]);
        let orbital: (char,char,char)  = (orbital_char_array[0].chars().next().unwrap(),
                                          orbital_char_array[1].chars().next().unwrap(),
                                          orbital_char_array[2].chars().next().unwrap());

        orbit_map.insert(orbital, main_mass);
    }

    let mut map_of_orbit_transfers_from_you: HashMap<(char,char,char),i32> = HashMap::new();
    let mut map_of_orbit_transfers_from_santa: HashMap<(char,char,char),i32> = HashMap::new();

    get_orbit_transfers_from_object_to_main_mass(&('Y','O','U'), &mut map_of_orbit_transfers_from_you, &orbit_map);
    get_orbit_transfers_from_object_to_main_mass(&('S','A','N'), &mut map_of_orbit_transfers_from_santa, &orbit_map);

    if map_of_orbit_transfers_from_you.contains_key(&('S','A','N')) {
        let result = map_of_orbit_transfers_from_you.get(&('S','A','N')).unwrap();
        println!("result = {}", result + 1);
    } else if map_of_orbit_transfers_from_santa.contains_key(&('Y','O','U')) {
        let result = map_of_orbit_transfers_from_santa.get(&('Y','O','U')).unwrap();
        println!("result = {}", result + 1);
    } else {
        let mut shortest_distance = 0;
        for keys in map_of_orbit_transfers_from_you.keys() {
            if map_of_orbit_transfers_from_santa.contains_key(keys) {
                let distance_from_you = map_of_orbit_transfers_from_you.get(keys).unwrap();
                let distance_from_santa = map_of_orbit_transfers_from_santa.get(keys).unwrap();
                let distance_between = distance_from_santa + distance_from_you;

                if shortest_distance == 0 || shortest_distance >= distance_between {
                    shortest_distance = distance_between;
                }
            }
        }
        println!("\tshortest distance: {}", shortest_distance-2);
    }
    println!("Completed day6 part2!\n");
}

fn get_orbit_transfers_from_object_to_main_mass(key: &(char,char,char), map: &mut HashMap<(char,char,char),i32>, orbit_map: &HashMap<(char,char,char),(char,char,char)>) {
    let mut orbital_jumps = 0;

    let mut orbital = key;

    while orbital != &('C','O','M') {
        orbital = orbit_map.get(&orbital).unwrap();
        orbital_jumps += 1;
        map.insert(*orbital, orbital_jumps);
    }
}
