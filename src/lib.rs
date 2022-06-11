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
pub struct InvalidTargetError {}

impl Error for InvalidTargetError {}

impl fmt::Display for InvalidTargetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "target should be a positive power of 2")
    }
}

#[derive(Debug)]
pub struct NoValidMovesLeftError {}

impl Error for NoValidMovesLeftError {}

impl fmt::Display for NoValidMovesLeftError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No valid moves left")
    }
}

/// Start a new game
/// 
/// `dimension`: Dimensions of the board. Example `4` for a 4x4 board.
/// 
/// `target`: Target to achieve to win the game. (Must be a power of 2). Example `2048`. (Defaults to 2048)
pub fn new(dimension: usize, target: usize) -> Result<Game, InvalidTargetError> {
    if (target & (target - 1)) != 0 || target < 5 {
        return Err(InvalidTargetError {});
    }

    Ok(Game {
        board: create_board(dimension),
        target,
    })
}

// creates a board with given dimensions and initializes with 2 random numbers
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

// add a 2 or 4 at a random location
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
    /// Print the board
    pub fn print_board(&self) {
        for i in &self.board {
            for j in i {
                print!("    {}", j);
            }
            println!();
        }
    }

    /// Get status of gameplay
    pub fn get_status(&self) -> Status {
        if contains_target(&self.board, &self.target) {
            return Status::Won;
        }

        if no_valid_moves_left(&self.board) {
            return Status::Lost;
        }

        Status::Running
    }

    /// Combine to the right
    pub fn combine_right(&mut self) {
        for row in self.board.iter_mut() {
            row.reverse();
            combine_array(row);
            row.reverse();
        }
    }

    /// Combine to the left
    pub fn combine_left(&mut self) {
        for row in self.board.iter_mut() {
            combine_array(row);
        }
    }

    /// Combine to the top
    pub fn combine_top(&mut self) {
        transpose_board(&mut self.board);
        for row in self.board.iter_mut() {
            combine_array(row);
        }
        transpose_board(&mut self.board);
    }

    /// Combine to the bottom
    pub fn combine_bottom(&mut self) {
        transpose_board(&mut self.board);
        for row in self.board.iter_mut() {
            row.reverse();
            combine_array(row);
            row.reverse();
        }
        transpose_board(&mut self.board);
    }

    /// This will add a 2 or a 4 at a random position
    /// 
    /// `refresh()` will return with a `NoValidMovesLeftError` error if there aren't any valid moves possible
    pub fn refresh(&mut self) -> Result<(), NoValidMovesLeftError> {
        add_2_or_4_at_random_location(&mut self.board);

        if no_valid_moves_left(&self.board) {
            return Err(NoValidMovesLeftError {});
        }

        Ok(())
    }

}

// check if board contains the target number
fn contains_target(board: &Vec<Vec<usize>>, target: &usize) -> bool {
    for row in board {
        if row.contains(target) {
            return true;
        }
    }

    false
}

// check if a valid move is possible
fn no_valid_moves_left(board: &Vec<Vec<usize>>) -> bool {
    // if board has a `0`, then valid moves available
    for row in board {
        if row.contains(&0) {
            return false;
        }
    }

    // if no tile can be combined, then no valid moves
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

// convert rows into columns
fn transpose_board(board: &mut Vec<Vec<usize>>) {
    let board_clone = board.clone();
    let dimension = board.len();

    for i in 0..dimension {
        for j in 0..dimension {
            board[i][j] = board_clone[j][i];
        }
    }
}