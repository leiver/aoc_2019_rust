use crate::common::utils;
use voca_rs;
use std::collections::HashMap;

pub fn part1() {
    println!("Running day12 part1!");

    let mut moons: Vec<((i32,i32,i32),(i32,i32,i32))> = Vec::new();

    read_moons(&mut moons);

    let mut steps = 0;
    for _i in 0..1000 {
        for moon_index in 0..moons.len() {
            let ((x,y,z),(mut vel_x,mut vel_y,mut vel_z)) = moons[moon_index];

            for other_moon_index in 0..moons.len() {
                if moon_index != other_moon_index {
                    let ((x_other, y_other, z_other), (_vel_x, _vel_y, _vel_z)) = moons[other_moon_index];
                    if x > x_other {
                        vel_x -= 1;
                    } else if x < x_other {
                        vel_x += 1;
                    }
                    if y > y_other {
                        vel_y -= 1;
                    } else if y < y_other {
                        vel_y += 1;
                    }
                    if z > z_other {
                        vel_z -= 1;
                    } else if z < z_other {
                        vel_z += 1;
                    }
                }
            }
            moons[moon_index] = ((x,y,z),(vel_x,vel_y,vel_z));
        }

        for moon_index in 0..moons.len() {
            let ((mut x,mut y,mut z),(vel_x,vel_y,vel_z)) = moons[moon_index];

            x += vel_x;
            y += vel_y;
            z += vel_z;

            moons[moon_index] = ((x,y,z),(vel_x,vel_y,vel_z));
        }
        steps += 1;
    }

    let mut total_energy: i64 = 0;
    for moon_index in 0..moons.len() {
        let ((mut x,mut y,mut z),(mut vel_x,mut vel_y,mut vel_z)) = moons[moon_index];

        let pot: i64 = (x.abs() + y.abs() + z.abs()) as i64;
        let kin: i64 = (vel_x.abs() + vel_y.abs() + vel_z.abs()) as i64;
        total_energy += pot * kin;
    }

    println!("\tTotal energy in system = {}", total_energy);

    println!("Completed day12 part1!\n");
}

pub fn part2() {
    println!("Running day12 part2!");

    let contents = utils::read_whole_file("inputs/day12.txt");

    let mut lines = utils::read_lines_from_file("inputs/day12.txt");

    let (x_initial_1,y_initial_1,z_initial_1) = parse_line(lines.next().unwrap().unwrap());

    let (x_initial_2,y_initial_2,z_initial_2) = parse_line(lines.next().unwrap().unwrap());

    let (x_initial_3,y_initial_3,z_initial_3) = parse_line(lines.next().unwrap().unwrap());

    let (x_initial_4,y_initial_4,z_initial_4) = parse_line(lines.next().unwrap().unwrap());

    let (mut x1,mut y1,mut z1) = (x_initial_1,y_initial_1,z_initial_1);

    let (mut x2,mut y2,mut z2) = (x_initial_2,y_initial_2,z_initial_2);

    let (mut x3,mut y3,mut z3) = (x_initial_3,y_initial_3,z_initial_3);

    let (mut x4,mut y4,mut z4) = (x_initial_4,y_initial_4,z_initial_4);

    let (mut vx1,mut vy1,mut vz1) = (0,0,0);

    let (mut vx2,mut vy2,mut vz2) = (0,0,0);

    let (mut vx3,mut vy3,mut vz3) = (0,0,0);

    let (mut vx4,mut vy4,mut vz4) = (0,0,0);

    let mut x_looped: i64 = 0;
    let mut y_looped: i64 = 0;
    let mut z_looped: i64 = 0;

    let mut steps = 0;
    loop {
        steps += 1;

        if x_looped == 0 {
            change_velocities(&x1,&x2,&x3,&x4, &mut vx1, &mut vx2, &mut vx3, &mut vx4);

            x1 += vx1;
            x2 += vx2;
            x3 += vx3;
            x4 += vx4;

            if (x1,x2,x3,x4,vx1,vx2,vx3,vx4) == (x_initial_1,x_initial_2,x_initial_3,x_initial_4,0,0,0,0) {
                x_looped = steps;
            }
        }

        if y_looped == 0 {
            change_velocities(&y1,&y2,&y3,&y4, &mut vy1, &mut vy2, &mut vy3, &mut vy4);
            y1 += vy1;
            y2 += vy2;
            y3 += vy3;
            y4 += vy4;

            if (y1,y2,y3,y4,vy1,vy2,vy3,vy4) == (y_initial_1,y_initial_2,y_initial_3,y_initial_4,0,0,0,0) {
                y_looped = steps;
            }
        }

        if z_looped == 0 {
            change_velocities(&z1,&z2,&z3,&z4, &mut vz1, &mut vz2, &mut vz3, &mut vz4);
            z1 += vz1;
            z2 += vz2;
            z3 += vz3;
            z4 += vz4;

            if (z1,z2,z3,z4,vz1,vz2,vz3,vz4) == (z_initial_1,z_initial_2,z_initial_3,z_initial_4,0,0,0,0) {
                z_looped = steps;
            }
        }

        if x_looped > 0 && y_looped > 0 && z_looped > 0 {
            break;
        }
    }

    let mut steps_to_initial_state = 0;

    let (mut x_steps, mut y_steps, mut z_steps) = (x_looped, y_looped, z_looped);

    let mut xy_merged = false;
    let mut xz_merged = false;
    let mut yz_merged = false;

    println!("steps=x:{},y:{},z:{}", (x_steps+1i64),(y_steps+1i64),(z_steps+1i64));

    while steps_to_initial_state == 0 {
        if x_steps == y_steps && x_steps == z_steps {
            steps_to_initial_state = x_steps;
        } else if x_steps < y_steps && x_steps < z_steps {
            let lowest = if y_steps < z_steps {y_steps} else {z_steps};
            let difference = lowest - x_steps;
            x_steps += if difference % x_looped == 0 {difference} else {x_looped * ((difference/x_looped) + 1)};
        } else if y_steps < x_steps && y_steps < z_steps {
            let lowest = if x_steps < z_steps {x_steps} else {z_steps};
            let difference = lowest - y_steps;
            y_steps += if difference % y_looped == 0 {difference} else {y_looped * ((difference/y_looped) + 1)};
        } else if z_steps < x_steps && z_steps < y_steps {
            let lowest = if x_steps < y_steps {x_steps} else {y_steps};
            let difference = lowest - z_steps;
            z_steps += if difference % z_looped == 0 {difference} else {z_looped * ((difference/z_looped) + 1)};
        } else if x_steps == y_steps {
            if !xy_merged {
                xy_merged = true;
                x_looped = x_steps;
                y_looped = y_steps;
            }
            x_steps += x_looped;
            y_steps += y_looped;
        } else if x_steps == z_steps {
            if !xz_merged {
                xz_merged = true;
                x_looped = x_steps;
                z_looped = z_steps;
            }
            x_steps += x_looped;
            z_steps += z_looped;
        } else if y_steps == z_steps {
            if !yz_merged {
                yz_merged = true;
                z_looped = z_steps;
                y_looped = y_steps;
            }
            z_steps += z_looped;
            y_steps += y_looped;
        }
    }

    println!("\tSteps to initial state = {}", steps_to_initial_state);

    println!("Completed day12 part1!\n");
}

fn read_moons(moons: &mut Vec<((i32,i32,i32),(i32,i32,i32))>) {
    for line in utils::read_lines_from_file("inputs/day12.txt") {
        let line = line.unwrap();

        let (x,y,z) = parse_line(line);

        moons.push(((x,y,z),(0,0,0)));
    }
}

fn parse_line(line: String) -> (i32,i32,i32) {
    let mut line_str = line.as_str();
    let index_x = voca_rs::index::search(line_str, "x=-?\\d+,", 0);
    let index_y = voca_rs::index::search(line_str, "y=-?\\d+,", 0);
    let index_z = voca_rs::index::search(line_str, "z=-?\\d+>", 0);
    let end = voca_rs::index::index_of(line_str, ">", 0);

    let x: i32 = voca_rs::chop::substring(line_str, (index_x + 2) as usize, (index_y - 2) as usize).parse().unwrap();
    let y: i32 = voca_rs::chop::substring(line_str, (index_y + 2) as usize, (index_z - 2) as usize).parse().unwrap();
    let z: i32 = voca_rs::chop::substring(line_str, (index_z + 2) as usize, (end) as usize).parse().unwrap();

    (x,y,z)
}

fn change_velocities(p1: &i32, p2: &i32, p3: &i32, p4: &i32, v1: &mut i32, v2: &mut i32, v3: &mut i32, v4: &mut i32) {

    *v1 +=
        ((p2 - p1) / if (p2 - p1).abs() == 0 {1} else {(p2 - p1).abs()}) +
            ((p3 - p1) / if (p3 - p1).abs() == 0 {1} else {(p3 - p1).abs()}) +
            ((p4 - p1) / if (p4 - p1).abs() == 0 {1} else {(p4 - p1).abs()});
    *v2 +=
        ((p1 - p2) / if (p1 - p2).abs() == 0 {1} else {(p1 - p2).abs()}) +
            ((p3 - p2) / if (p3 - p2).abs() == 0 {1} else {(p3 - p2).abs()}) +
            ((p4 - p2) / if (p4 - p2).abs() == 0 {1} else {(p4 - p2).abs()});
    *v3 +=
        ((p1 - p3) / if (p1 - p3).abs() == 0 {1} else {(p1 - p3).abs()}) +
            ((p2 - p3) / if (p2 - p3).abs() == 0 {1} else {(p2 - p3).abs()}) +
            ((p4 - p3) / if (p4 - p3).abs() == 0 {1} else {(p4 - p3).abs()});
    *v4 +=
        ((p2 - p4) / if (p2 - p4).abs() == 0 {1} else {(p2 - p4).abs()}) +
            ((p3 - p4) / if (p3 - p4).abs() == 0 {1} else {(p3 - p4).abs()}) +
            ((p1 - p4) / if (p1 - p4).abs() == 0 {1} else {(p1 - p4).abs()});
}
