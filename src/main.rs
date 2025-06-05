mod cube;
mod color;
mod move_table;
mod coordinate_compute;
mod pruning_table;

use cube::Cube;
use cube::Move;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("usage : {} \"<scramble>\"", args[0]);
        std::process::exit(1);
    }

    let mut cube = Cube::new_solved();

    for cube_move in args[1].split_whitespace() {
        let mut number_move = String::new();
        let mut letter_move = String::new();

        for c in cube_move.chars() {
            if c.is_numeric() {
                number_move.push(c);
            } else {
                letter_move.push(c);
            }
        }

        let movement = Move::from_str(letter_move.clone());

        if movement == Move::None {
            eprintln!("Invalid move: {}", letter_move);
            std::process::exit(1);
        }

        if number_move.is_empty() {
            cube.apply_move(&movement);

            continue;
        }

        for _index in 0..number_move.parse::<i32>().unwrap() {
            cube.apply_move(&movement);
        }
    }

    if cube.is_solved() {
        std::process::exit(0);
    }

    //cube.solve();
}
