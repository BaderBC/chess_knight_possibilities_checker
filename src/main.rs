use std::io;
use std::sync::{Arc, Mutex};
use rayon::prelude::*;

fn main() {
    let mut x = String::new();
    let mut y = String::new();
    let mut board_size = String::new();
    let stdin = io::stdin();

    println!("Get knight route");
    println!("This program shows you the most optimal way, to visit all fields exactly one times\n");

    println!("Enter x position of knight (we count from 0)");
    stdin.read_line(&mut x).unwrap();
    println!("Enter y position of knight (we count from 0)");
    stdin.read_line(&mut y).unwrap();
    println!("Enter board size (we recommend using board no smaller than 3, and no bigger than 5 (program is really slow when you check for 6 or higher)");
    stdin.read_line(&mut board_size).unwrap();

    let x: u8 = x.trim().parse::<u8>().unwrap_or(0);
    let y: u8 = y.trim().parse::<u8>().unwrap_or(0);
    let board_size = board_size.trim().parse::<u8>().expect("You should pass int value");

    let starting_point = (x, y);
    
    let mut how_many_iterations = Arc::new(Mutex::new(0));

    let knight_path: Vec<(u8, u8)> = find_chess_knight_path(board_size, vec![starting_point.clone()], &mut how_many_iterations).unwrap();

    println!("\n\nWe finally got a results:");
    println!("{:?}", knight_path);
}

fn find_chess_knight_path(board_size: u8, moves_already_done: Vec<(u8, u8)>, how_many_iterations: &mut Arc<Mutex<u128>>) -> Option<Vec<(u8, u8)>> {
    let knight_position = *moves_already_done.last()
        .expect("Error, vector with default position should have at least 1 element");
    let (x, y) = (knight_position.0 as i8, knight_position.1 as i8);
    let possible_moves: Vec<(i8, i8)> = vec![
        (x + 2, y + 1),
        (x + 2, y - 1),
        (x + 1, y + 2),
        (x + 1, y - 2),
        (x - 2, y + 1),
        (x - 2, y - 1),
        (x - 1, y + 2),
        (x - 1, y - 2)
    ];
    
    let how_many_iterations_clone = how_many_iterations.clone();
    let mut iterations = how_many_iterations_clone.lock().unwrap();
    *iterations += 1;
    if *iterations % 1000000 == 0 {
        println!("Iterations: 1 milion * {}", *iterations / 1000000);
    }
    drop(iterations);

    let possible_moves: Vec<(u8, u8)> = possible_moves
        .into_iter()
        .filter(|(x, y)| *x >= 0 && *y >= 0 && *x < board_size as i8 && *y < board_size as i8)
        .map(|(x, y)| (x as u8, y as u8))
        .filter(|position| !moves_already_done.contains(&position))
        .collect();
    
    let the_longest_path = possible_moves
        .par_iter()  // Parallel iterator
        .map(|&potential_knight_position| {
            let mut moves_that_will_be_done = moves_already_done.clone();
            moves_that_will_be_done.push(potential_knight_position);
            let mut how_many_iterations = how_many_iterations.clone();
            find_chess_knight_path(board_size, moves_that_will_be_done.clone(), &mut how_many_iterations)
                .unwrap_or_else(|| moves_that_will_be_done)
        })
        .max_by_key(|vec| vec.len());

    the_longest_path
}
