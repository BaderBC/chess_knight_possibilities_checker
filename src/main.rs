fn main() {
    println!("Hello, world!");
    let starting_point: (u8, u8) = (1, 0);
    let knight_path: Vec<(u8, u8)> = find_chess_knight_path(starting_point, 6, vec![starting_point]).unwrap();

    println!("{:?}", knight_path);
}

fn find_chess_knight_path(knight_position: (u8, u8), board_size: i8, moves_already_done: Vec<(u8, u8)>) -> Option<Vec<(u8, u8)>> {
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

    let possible_moves = possible_moves
        .into_iter()
        .filter(|(x, y)| *x >= 0 && *y >= 0 && *x < board_size && *y < board_size)
        .map(|(x, y)| (x as u8, y as u8))
        .filter(|position| !moves_already_done.contains(position));


    let the_longest_path = possible_moves
        .map(|potential_knight_position| {
            let mut moves_that_will_be_done = moves_already_done.clone();
            moves_that_will_be_done.push(potential_knight_position);
            let path_option = find_chess_knight_path(
                potential_knight_position,
                board_size,
                moves_that_will_be_done,
            );
            if let Some(mut path) = path_option {
                path.insert(0, potential_knight_position);
                path
            } else {
                println!("one route is check, here results: {:?} \n\n\n", moves_already_done);
                vec![potential_knight_position]
            }
        });

    the_longest_path
        .max_by_key(|vec| vec.len())
}
