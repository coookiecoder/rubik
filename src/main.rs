mod cube;
mod color;

use cube::Cube;
use color::Color;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("usage : {} \"<scramble>\"", args[0]);
        std::process::exit(1);
    }

    let mut cube = Cube{
        front: vec![Color::Blue; 9],
        back: vec![Color::Green; 9],
        right: vec![Color::Red; 9],
        left: vec![Color::Orange; 9],
        top: vec![Color::Yellow; 9],
        bottom: vec![Color::White; 9],
    };

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

        if number_move.is_empty() {
            match letter_move.as_str() {
                "U" => cube.up(),
                "U’" => cube.up_prime(),

                "D" => cube.down(),
                "D’" => cube.down_prime(),

                "R" => cube.right(),
                "R’" => cube.right_prime(),

                "L" => cube.left(),
                "L’" => cube.left_prime(),

                "F" => cube.front(),
                "F’" => cube.front_prime(),

                "B" => cube.back(),
                "B’" => cube.back_prime(),

                _ => { println!("Unknown move: {}", cube_move); std::process::exit(1) }
            }

            println!("did move {} 1 times now", letter_move);

            continue;
        }

        for _index in 0..number_move.parse::<i32>().unwrap() {
            match letter_move.as_str() {
                "U" => cube.up(),
                "U’" => cube.up_prime(),

                "D" => cube.down(),
                "D’" => cube.down_prime(),

                "R" => cube.right(),
                "R’" => cube.right_prime(),

                "L" => cube.left(),
                "L’" => cube.left_prime(),

                "F" => cube.front(),
                "F’" => cube.front_prime(),

                "B" => cube.back(),
                "B’" => cube.back_prime(),

                _ => { println!("Unknown move: {}", cube_move); std::process::exit(1) }
            }

            println!("did move {} {} times now", letter_move, number_move.parse::<i32>().unwrap());
        }
    }

    print!("{}", cube);
}
