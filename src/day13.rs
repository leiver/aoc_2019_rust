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

        //println!("\t\treceived input x={},y={},output={}", x, y, output);
        if x == -1 && y == 0 {
            score = output;
            //println!("\t\tupdating score to {}", score);
        } else if output == 3 {
            let (prev_x, prev_y) = paddle;
            //println!("\t\tnew paddle coordinate x={},y={} from previous x={},y={}", x, y, prev_x, prev_y);
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
            //println!("\t\tnew ball position x={},y={} and vel x={},y={}, from position x={},y={} and vel x={},y={}", x, y, new_vx, new_vy, prev_x, prev_y, prev_vx, prev_vy);

            let mut joystick_input = 0;
            if paddle != (0,0) {
                //let mut next_ball_x = ball.0 + ball_vel.0;
                //let next_ball_y = ball.1 + ball_vel.1;
                //let next_ball_tile = *game_board.entry((next_ball_x, next_ball_y)).or_insert(0);
                //let next_ball_tile2 = *game_board.entry((next_ball_x, ball.1)).or_insert(0);
                //let next_ball_tile3 = *game_board.entry((ball.0, next_ball_y)).or_insert(0);
                //println!("\t\tnext_ball_tile {} on x={},y={}", next_ball_tile, next_ball_x, next_ball_y);
                //println!("\t\tnext_ball_tile2 {} on x={},y={}", next_ball_tile2, next_ball_x, ball.1);
                //println!("\t\tnext_ball_tile3 {} on x={},y={}", next_ball_tile3, ball.0, next_ball_y);
                //if next_ball_tile == 2 || next_ball_tile == 1 {
                    //next_ball_x += (ball_vel.0 * -1) * 2;
                //}
                let distance_from_paddle = ball.0 - paddle.0;
                if distance_from_paddle != 0 {
                    joystick_input = distance_from_paddle / distance_from_paddle.abs();
                }
                //println!("\t\tmoving paddle towards projected ball position {} from paddle position {} with joystick input {}", ball.0, paddle.0, joystick_input);

            }
            /*if paddle != (0,0) {
                let (pad_x, pad_y) = paddle;
                let (ball_x, ball_y) = ball;
                let (mut temp_ball_x, mut temp_ball_y) = ball;
                let mut temp_vx = ball_vel.0;
                let mut temp_vy = ball_vel.1;
                let mut temp_game_board: HashMap<(i64,i64),i64> = HashMap::new();
                while temp_ball_y < pad_y-1 {
                    let mut next_y = temp_ball_y + temp_vy;
                    let mut next_x = temp_ball_x + temp_vx;
                    let diagonal_tile = *temp_game_board.entry((next_x,next_y))
                        .or_insert(*game_board.entry((next_x,next_y)).or_insert(0));
                    let x_tile = *temp_game_board.entry((next_x,temp_ball_y))
                        .or_insert(*game_board.entry((next_x,temp_ball_y)).or_insert(0));
                    let y_tile = *temp_game_board.entry((temp_ball_x,next_y))
                        .or_insert(*game_board.entry((temp_ball_x,next_y)).or_insert(0));
                    if diagonal_tile == 1 || diagonal_tile == 2 {
                        if diagonal_tile == 2 {
                            temp_game_board.insert((next_x, next_y), 0);
                        }
                        temp_vx *= -1;
                        temp_vy *= -1;
                        next_x -= temp_vx * 2;
                        next_y -= temp_vy * 2;
                    } else if x_tile == 1 || x_tile == 2 || y_tile == 1 || y_tile == 2 {
                        if x_tile == 1 || x_tile == 2 {
                            if x_tile == 2 {
                                temp_game_board.insert((next_x, temp_ball_y), 0);
                            }
                            temp_vx *= -1;
                            next_x -= temp_vx * 2;
                        }
                        if y_tile == 1 || y_tile == 2 {
                            if y_tile == 2 {
                                temp_game_board.insert((temp_ball_x, next_y), 0);
                            }
                            temp_vy *= -1;
                            next_y -= temp_vy * 2;
                        }
                    }
                    temp_ball_x = next_x;
                    temp_ball_y = next_y;
                }
                let distance_from_destination = temp_ball_x - pad_x;
                joystick_input = if distance_from_destination == 0 {
                    0
                } else {
                    distance_from_destination / distance_from_destination.abs()
                };
                println!("\t\tmoving paddle towards projected ball position {} from paddle position {} with joystick input {}", temp_ball_x, pad_x, joystick_input);
            }*/
            t_input.send(joystick_input);
        }
        if x != -1 || y != 0 {
            //println!("\t\tinserted {} on game board coordinate x={},y={}", output, x, y);
            game_board.insert((x, y), output);
        }
    }

    println!("\tScore = {}", score);
    println!("Completed day13 part1!\n");
}
