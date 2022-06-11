use std::env;
use std::io;
use play2048::*;

const DEFAULT_TARGET: usize = 2048;

fn main() {
    // get command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 && args.len() != 2 {
        println!("Usage: play2048 <board_dimension> [target]");
        return;
    }

    let board_dimension = args[1].parse::<usize>().expect("board_dimensions should be a positive number");
    let mut target = DEFAULT_TARGET;

    if args.len() == 3 {
       target = args[2].parse::<usize>().expect("target should be a positive number");
    }

    let mut game = play2048::new(board_dimension, target).unwrap();
    println!("Use a,s,d,f to move left, bottom, right and top. q to quit. Good luck!");

    loop {
        game.print_board();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");   

        match input.trim().parse::<char>() {
            Ok(operation) => {
                if ['d', 'a', 'w', 's'].contains(&operation) {
                    apply_action(&mut game, &operation);
                } else if operation == 'q' {
                    break;
                } else {
                    println!("Unrecognized input");
                    continue;
                }
            },
            Err(_) => continue,
        };

        match game.get_status() {
            Status::Won => {
                println!("Congratulations, you have won!");
                game.print_board();
                return;
            },
            Status::Lost => {
                println!("Game over!");
                game.print_board();
                return;
            },
            _ => (), 
        };

        match game.refresh() {
            Ok(()) => (),
            Err(_err) => {
                println!("Game over!");
                game.print_board();
                return;
            },
        }

        print!("{}[2J", 27 as char);
    }
}

fn apply_action(game: &mut Game, operation: &char) {
    match operation {
        'd' => {
            game.combine_right();
        },
        'a' => {
            game.combine_left();
        },
        'w' => {
            game.combine_top();
        },
        's' => {
            game.combine_bottom();
        },
        _ => (),
    }
}