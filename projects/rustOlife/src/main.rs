extern crate rand;
extern crate core;
use rand::Rng;
use std::time::Duration;
use std::thread;

fn main() {
    let mut board: [[&'static str; 32]; 32] = [["."; 32]; 32];

    for row in 1..board.len()-1 {
        for cell in 1..board[row].len()-1 {
            let num = generate_random_number(3);
            if num == 1 {
                board[row][cell] = "*";
            }
        }
    }

    for _ in 0..10 {
        println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
        thread::sleep(Duration::from_millis(400));
        board = check_board(board);
    }
}

fn generate_random_number(limit:i32) -> i32 {
    let random_number = rand::thread_rng().gen_range(1,limit);
    random_number
}

fn check_board(board: [[&'static str;32];32]) -> [[&'static str;32];32] {
    let mut new_board: [[&'static str; 32]; 32] = [["."; 32]; 32];

     for row in 1..board.len()-1{
        for cell in 1..board[row].len()-1 {
            let mut count = 0;
            if board[row][cell-1] == "*"{
                count += 1;
            }
            if board[row][cell+1] == "*" {
                count += 1;
            }
            if board[row-1][cell] == "*" {
                count += 1;
            }
            if board[row+1][cell] == "*" {
                count += 1;
            }
            if board[row-1][cell-1] == "*" {
                count += 1;
            }
            if board[row+1][cell+1] == "*" {
                count += 1;
            }
            if board[row+1][cell-1] == "*" {
                count += 1;
            }
            if board[row-1][cell+1] == "*" {
                count += 1;
            }
            
            match board[row][cell] {
                "." => {
                    if count == 3{
                        new_board[row][cell] = "*";
                    }
                },
                "*" => {
                    if count < 2 {
                        new_board[row][cell] = ".";
                    }
                    else if count > 3 {
                        new_board[row][cell] = ".";
                    }
                    else if count == 3 {
                        new_board[row][cell] = "*";
                    }

                },
                _ => println!("eh"),
            }
        }
    }
    for row in new_board.iter(){
        println!("{:?}", row);
    }
    println!("\n");
    new_board
}
