use crate::common::intcode::intcode;
use crate::common::utils;
use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;
use std::thread;
use voca_rs;

pub fn part1() {
    println!("Running day13 part1!");

    let mut program = utils::read_whole_file("inputs/day13.txt");

    let (t_input, r_input) = mpsc::channel();
    let (t_output, r_output) = mpsc::channel();
    let t_output_final = mpsc::Sender::clone(&t_output);

    thread::spawn(move || {
        intcode(program, r_input, t_output);
        t_output_final.send(-1);
    });

    let mut game_board: HashMap<(i64,i64),i64> = HashMap::new();

    let mut block_tiles = 0;
    loop {
        let x = r_output.recv().unwrap();
        if x == -1 {
            break;
        }
        let y = r_output.recv().unwrap();
        if y == -1 {
            break;
        }
        let tile = r_output.recv().unwrap();
        if tile == -1 {
            break;
        } else if tile == 2 {
            block_tiles += 1;
        }
        game_board.insert((x,y), tile);
    }

    println!("\tBlock tiles on the screen: {}", block_tiles);

    println!("Completed day13 part1!\n");
}

pub fn part2() {
    println!("Running day13 part2!");

    let mut program = utils::read_whole_file("inputs/day13.txt");

    let mut program_str = program.as_str();
    let mut first_input = "2,";
    let mut program = String::from(format!("2,{}", voca_rs::chop::substring(program_str, (voca_rs::index::index_of(program_str, ",", 0) + 1 ) as usize, 0).as_str()));

    let (t_input, r_input) = mpsc::channel();
    let (t_output, r_output) = mpsc::channel();
    let t_output_final = mpsc::Sender::clone(&t_output);

    thread::spawn(move || {
        intcode(program, r_input, t_output);
        t_output_final.send(-99);
    });

    let mut game_board: HashMap<(i64,i64),i64> = HashMap::new();
    let mut ball = (0,0);
    let mut ball_vel = (0,0);
    let mut paddle = (0,0);
    let mut score = 0;

    loop {
        if t_input. {
            println!("\t\tx was not ready, parsing");
            let (vx, vy) = ball_vel;
            let mut joystick_input = 0;
            if (vy > 0) {
                let (pad_x, pad_y) = paddle;
                let (ball_x, ball_y) = ball;
                let (mut temp_ball_x, mut temp_ball_y) = ball;
                let mut temp_vx = vx;
                while temp_ball_y != pad_y {
                    temp_ball_y += 1;
                    let mut next_x = temp_ball_x + temp_vx;
                    let tile = *game_board.entry((temp_ball_x,temp_ball_y)).or_insert(0);
                    if tile == 1 {
                        next_x -= temp_vx;
                        temp_vx *= -1;
                    }
                    temp_ball_x = next_x;
                }
                let distance_from_destination = temp_ball_x - pad_x;
                joystick_input = if distance_from_destination == 0 {
                    0
                } else {
                    distance_from_destination / distance_from_destination.abs()
                };
                println!("\t\tmoving paddle towards projected ball position {} from paddle position {} with joystick input {}", temp_ball_x, pad_x, joystick_input);
            }
            t_input.send(joystick_input);
        }
        let x = r_output.recv().unwrap();
        if x == -99 {
            break;
        }
        let y = r_output.recv().unwrap();
        if y == -99 {
            break;
        }
        let output = r_output.recv().unwrap();
        if output == -99 {
            break;
        }

        println!("\t\treceived input x={},y={},output={}", x, y, output);
        if x == -1 && y == 0 {
            score = output;
        } else if output == 3 {
            let (prev_x, prev_y) = paddle;
            println!("\t\tnew paddle coordinate x={},y={} from previous x={},y={}", x, y, prev_x, prev_y);
            paddle = (x,y);
        } else if output == 4 {
            let (prev_x, prev_y) = ball;
            let (prev_vx, prev_vy) = ball_vel;
            let (mut new_vx, mut new_vy) = ball_vel;

            if x == prev_x {
                new_vx *= -1;
            } else {
                let difference = x - prev_x;
                new_vx = difference / difference.abs();
            }
            if y == prev_y {
                new_vy *= -1;
            } else {
                let difference = y - prev_y;
                new_vy = difference / difference.abs();
            }
            ball = (x,y);
            ball_vel = (new_vx, new_vy);
            println!("\t\tnew ball position x={},y={} and vel x={},y={}, from position x={},y={} and vel x={},y={}", x, y, new_vx, new_vy, prev_x, prev_y, prev_vx, prev_vy);
        }
        if x != -1 || y != 0 {
            println!("\t\tinserted {} on game board coordinate x={},y={}", output, x, y);
            game_board.insert((x, y), output);
        }
    }

    println!("\tScore = {}", score);
    println!("Completed day13 part1!\n");
}
