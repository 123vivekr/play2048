use rand::Rng;
use std::{error::Error, fmt};

pub struct Game {
    board: Vec<Vec<usize>>,
    target: usize,
}

pub enum Status {
    Won,
    Lost,
    Running
}

#[derive(Debug)]
pub enum InitializationError {
    InvalidTarget
}

impl Error for InitializationError {}

impl fmt::Display for InitializationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg:&str = match self {
            InitializationError::InvalidTarget => "target should be a positive power of 2"
        };

        write!(f, "{}", msg)
    }
}

#[derive(Debug)]
pub struct NoValidMovesLeft {}

impl Error for NoValidMovesLeft {}

impl fmt::Display for NoValidMovesLeft {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No valid moves left")
    }
}
  

pub fn new(dimension: usize, target: usize) -> Result<Game, InitializationError> {
    if (target & (target - 1)) != 0 || target < 5 {
        return Err(InitializationError::InvalidTarget);
    }

    Ok(Game {
        board: create_board(dimension),
        target,
    })
}


fn create_board(dimension: usize) -> Vec<Vec<usize>> {
    let mut board = Vec::new();
    for _ in 0..dimension {
        let mut row = Vec::new();
        for _ in 0..dimension {
            row.push(0);
        }
        board.push(row);
    }

    add_2_or_4_at_random_location(&mut board);
    add_2_or_4_at_random_location(&mut board);

    board
}

fn add_2_or_4_at_random_location(board: &mut Vec<Vec<usize>>) {
    let dimension = board.len();
    let rand_2_or_4 = if rand::thread_rng().gen_range(0..2) == 0 {
        2
    } else {
        4
    };

    loop {
        let rand_row = rand::thread_rng().gen_range(0..dimension) as usize;
        let rand_col = rand::thread_rng().gen_range(0..dimension) as usize;

        if board[rand_row][rand_col] == 0 {
            board[rand_row][rand_col] = rand_2_or_4;
            return;
        }
    }
}

impl Game {
    pub fn print_board(&self) {
        for i in &self.board {
            for j in i {
                print!("{} ", j);
            }
            println!();
        }
    }

    pub fn get_status(&self) -> Status {
        if contains_target(&self.board, &self.target) {
            return Status::Won;
        }

        if no_valid_moves_left(&self.board) {
            return Status::Lost;
        }

        Status::Running
    }

    pub fn combine_right(&mut self) {
        for row in self.board.iter_mut() {
            row.reverse();
            combine_array(row);
            row.reverse();
        }
    }

    pub fn combine_left(&mut self) {
        for row in self.board.iter_mut() {
            combine_array(row);
        }
    }

    pub fn combine_top(&mut self) {
        transpose_board(&mut self.board);
        for row in self.board.iter_mut() {
            combine_array(row);
        }
        transpose_board(&mut self.board);
    }

    pub fn combine_bottom(&mut self) {
        transpose_board(&mut self.board);
        for row in self.board.iter_mut() {
            row.reverse();
            combine_array(row);
            row.reverse();
        }
        transpose_board(&mut self.board);
    }

    pub fn refresh(&mut self) -> Result<(), NoValidMovesLeft> {
        add_2_or_4_at_random_location(&mut self.board);

        if no_valid_moves_left(&self.board) {
            return Err(NoValidMovesLeft {});
        }

        Ok(())
    }

}

fn contains_target(board: &Vec<Vec<usize>>, target: &usize) -> bool {
    for row in board {
        if row.contains(target) {
            return true;
        }
    }

    false
}

fn no_valid_moves_left(board: &Vec<Vec<usize>>) -> bool {
    for row in board {
        if row.contains(&0) {
            return false;
        }
    }

    let dimension = board.len();
    for i in 0..dimension - 1 {
        for j in 0..dimension - 1 {
            if board[i][j] == board[i][j+1] || 
            board[i][j] == board[i+1][j] {
                return false;
            }
        }
    }

    true
}

// combines array to the left
fn combine_array(array: &mut Vec<usize>) {
    let mut move_or_merge = true;
    let mut merged = vec![false; array.len()];

    while move_or_merge {
        move_or_merge = false;
        let mut i = 0;
        while i < array.len() - 1 {
            if array[i] == 0 && array[i+1] == 0 {
                i += 1;
                continue;
            } else if array[i] == 0 && array[i+1] != 0 {
                array[i] = array[i+1];
                array[i+1] = 0;
                if merged[i+1] {
                    merged[i] = true;
                    merged[i+1] = false;
                }
                move_or_merge = true;
            } else if !merged[i] && !merged[i+1] && array[i] == array[i+1] {
                array[i] *= 2;
                array[i+1] = 0;
                merged[i] = true; 
                i += 1;
                move_or_merge = true;
            }

            i += 1;
        }
    } 
}

fn transpose_board(board: &mut Vec<Vec<usize>>) {
    let board_clone = board.clone();
    let dimension = board.len();

    for i in 0..dimension {
        for j in 0..dimension {
            board[i][j] = board_clone[j][i];
        }
    }
}