mod pieces;

extern crate termion;
extern crate rand;

use termion::raw::IntoRawMode;
use termion::async_stdin;
use rand::Rng;
use std::io::{Read, Write, stdout};
use std::thread;
use std::time::Duration;


fn display_map(stdout: &mut termion::raw::RawTerminal<std::io::StdoutLock<'_>>, map: &Vec<Vec<char>>) {
    write!(stdout, "{}{}+", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
    for _x in 0..map.len()-2 {
        write!(stdout, "-", ).unwrap();
    }
    write!(stdout, "+").unwrap();

    let display = map.clone();
    for y in 0..map[0].len() {
        write!(stdout, "{}", termion::cursor::Goto(1,  (y+2) as u16)).unwrap();
        write!(stdout, "|").unwrap();
        for x in 2..map.len() {
            if display[x][y] == '*' {
                write!(stdout, "*").unwrap();
            } else {
                write!(stdout, " ").unwrap();
            }
        }
        write!(stdout, "|").unwrap();
    }

    write!(stdout, "{}+", termion::cursor::Goto(1, map[0].len() as u16 + 2)).unwrap();
    for _x in 0..map.len()-2 {
        write!(stdout, "-", ).unwrap();
    }
    write!(stdout, "+").unwrap();
}

fn is_row_filled(map: &mut Vec<Vec<char>>, row: usize) -> bool {
    for x in 2..map.len() {
        if map[x][row] != '*' {
            return false;
        }
    }
    return true;
}

fn shift_rows_down(map: &mut Vec<Vec<char>>, row: usize) {
    for y in 0..row {
        for x in 2..map.len() {
            map[x][row-y] = map[x][row-y-1];
        }
    }
    for x in 2..map.len() {
        map[x][0] = ' ';
    }
}

fn remove_filled_rows(map: &mut Vec<Vec<char>>) {
    for y in 0..map[0].len() {
        if is_row_filled(map, y) {
            shift_rows_down(map, y);
        }
    }
}

fn check_collision(display: &Vec<Vec<char>>, current_piece: &[[i32; 4]; 4] , x_pos: usize, y_pos: usize) -> bool {
    for y in 0..current_piece.len() {
        for x in 0..current_piece[0].len() {
            if current_piece[y][x] == 1 {
                if x + x_pos >= display.len() {
                    return true;
                }
                if y + y_pos >= display[0].len() {
                    return true;
                }
                if display[x + x_pos][y + y_pos] == '*' {
                    return true;
                }
            }
        }
    }
    return false;
}

fn print_piece(map: &mut Vec<Vec<char>>, current_piece: &[[i32; 4]; 4] , x_pos: usize, y_pos: usize) {
    for y in 0..current_piece.len() {
        for x in 0..current_piece[0].len() {
            if current_piece[y][x] == 1 {
                map[x + x_pos][y + y_pos] = '*';
            }
        }
    }
}

fn get_random_piece() -> [[[i32; 4]; 4]; 4] {
    pieces::SRS[rand::thread_rng().gen_range(0, pieces::SRS.len())]
}

fn main() {
    let x_size = 12;
    let y_size = 11;
    let mut speed = 0;
    let mut map: Vec<Vec<char>> = vec![vec![' '; y_size]; x_size];
    let mut rotation_index = 1;

    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    let mut x_pos = 5;
    let mut y_pos = 0;

    for y in 0..map[0].len() {
        map[1][y] = '*';
    }

    let mut current_piece = get_random_piece();

    loop {
        let mut display = map.clone();

        print_piece(&mut display, &current_piece[rotation_index], x_pos, y_pos);

        display_map(&mut stdout, &display);

        match stdin.next() {
            Some(Ok(b'a')) => {
                if  x_pos != 0 {
                    if check_collision(&map, &current_piece[rotation_index], x_pos - 1, y_pos) == false {
                        x_pos -= 1;
                    }
                }
            }
            Some(Ok(b'd')) => {
                if check_collision(&map, &current_piece[rotation_index], x_pos + 1, y_pos) == false {
                    x_pos += 1;
                }
            }
            Some(Ok(b'e')) => {
                while check_collision(&map, &current_piece[rotation_index], x_pos, y_pos + 1) == false {
                    y_pos += 1;
                }
            }
            Some(Ok(b'w')) => {
                if rotation_index == 0 && check_collision(&map, &current_piece[3], x_pos, y_pos) == false {
                    rotation_index = 3;
                } else if check_collision(&map, &current_piece[rotation_index-1], x_pos, y_pos) == false {
                    rotation_index = rotation_index-1;
                }
            }
            Some(Ok(b's')) => {
                if check_collision(&map, &current_piece[(rotation_index+1)%4], x_pos, y_pos) == false {
                    rotation_index = (rotation_index+1)%4;
                }
            }
            Some(Ok(b'q')) => break,
            _ => {}
        }
        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(10));

        speed += 1;
        if speed > 50 {
            speed = 0;
            // Check if we hit bottom
            if check_collision(&map, &current_piece[rotation_index], x_pos, y_pos + 1) == true {
                print_piece(&mut map, &current_piece[rotation_index], x_pos, y_pos);
                remove_filled_rows(&mut map);

                // Spawn new block
                y_pos = 0;
                x_pos = 5;
                current_piece = get_random_piece();
                // Check if the spawn block is blocked => game over
                if check_collision(&map, &current_piece[rotation_index], x_pos, y_pos) == true {
                    return;
                }
            } else {
                y_pos = y_pos + 1;
            }
        }
    }
}
