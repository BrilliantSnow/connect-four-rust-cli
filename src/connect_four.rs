pub mod constants {
    pub const WIDTH: usize = 10;
    pub const HEIGHT: usize = 6;
    pub const GOAL: usize = 4;
}

#[derive(Clone, Copy, PartialEq)]
pub enum Token {
    RED,
    BLUE,
    EMPTY,
}

pub mod game {
    use std::io;

    use super::{
        board,
        constants::{HEIGHT, WIDTH},
        Token, win_checker,
    };

    pub fn take_turn(state: &mut board::Board, token: Token) -> (usize, usize) {
        let player = match token {
            Token::BLUE => "Blue",
            Token::RED => "Red",
            _ => panic!("Invalid player")
        };
        println!("{player}'s Turn:");
        loop {
            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let chosen_column: usize = input.trim().parse().expect("Please type a number!");

            if 0 < chosen_column && chosen_column <= WIDTH {
                let potential_move = play_token(token, chosen_column - 1, state);

                match potential_move {
                    Option::Some(x) => return x,
                    _ => println!("No room in that column. Try again:")
                }
            } else {
                println!("Choose a number between 1 and {WIDTH}:");
            }
        }
    }

    pub fn check_win(state: board::Board, player: Token, _: (usize, usize)) -> bool {
        win_checker::check_vertical(state, player) ||
        win_checker::check_horizontal(state, player) ||
        win_checker::check_diagonal(state, player)
    }

    fn play_token(token: Token, column: usize, map: &mut board::Board) -> Option<(usize, usize)> {
        for row in 0..HEIGHT {
            match map[row * WIDTH + column] {
                Token::EMPTY => {
                    map[row * WIDTH + column] = token;
                    return Some((row, column));
                }
                Token::RED => continue,
                Token::BLUE => continue,
            }
        }
        return Option::None;
    }
}

pub mod board {
    use super::{
        constants::{HEIGHT, WIDTH},
        Token,
    };

    pub type Board = [Token; WIDTH * HEIGHT];

    pub fn new() -> Board {
        let empty_board: Board = [Token::EMPTY; WIDTH * HEIGHT];
        return empty_board;
    }

    pub fn print(board: Board) {
        print!(
            "\u{250F}\u{2501}\u{2501}{}\u{2501}\u{2513}\n",
            "\u{2501}\u{2533}\u{2501}\u{2501}".repeat(WIDTH - 1)
        );
        for y in (0..HEIGHT).rev() {
            print!("\u{2503}");
            for x in 0..WIDTH {
                let token = board[x + y * WIDTH];
                match token {
                    Token::RED => print!("{} \u{2503}", "\u{1F534}"),
                    Token::BLUE => print!("{} \u{2503}", "\u{1F535}"),
                    Token::EMPTY => print!("{} \u{2503}", "  "),
                };
            }
            print!("\n");
            if y > 0 {
                print!(
                    "\u{2523}\u{2501}\u{2501}{}\u{2501}\u{252B}\n",
                    "\u{2501}\u{254B}\u{2501}\u{2501}".repeat(WIDTH - 1)
                );
            }
        }
        print!(
            "\u{2517}\u{2501}\u{2501}{}\u{2501}\u{251B}\n",
            "\u{2501}\u{253B}\u{2501}\u{2501}".repeat(WIDTH - 1)
        );
        println!();
    }
}

pub mod win_checker {
    use std::cmp::min;

    use super::{constants::{HEIGHT, WIDTH, GOAL}, board, Token};

    pub fn check_diagonal(map: board::Board, player: Token) -> bool {
        check_downwards_diagonal(map, player) ||
        check_upwards_diagonal(map, player)
    }

    fn check_downwards_diagonal(map: board::Board, player: Token) -> bool {
        // right to left
        // iterate over each diagonal that touches y = height
        for diagonals in 0..WIDTH - GOAL + 1 {
            let mut count = 0;
            // iterate along the diagonal
            // top to bottom
            for step in 0..min(GOAL + diagonals, HEIGHT) {
                let base_x = WIDTH - GOAL - diagonals;
                let base_y = HEIGHT - 1;
                if map[(base_y - step) * WIDTH + (base_x + step)] == player {
                    count += 1;
                } else {
                    count = 0;
                }
                if count >= GOAL {
                    return true;
                }
            }
        }

        // left to right
        // iterate over each diagonal that touches x = 0 (but not y = height)
        for diagonals in 0..HEIGHT - GOAL {
            let mut count = 0;
            // iterate along the diagonal
            // top to bottom
            for step in 0..min(GOAL + diagonals, WIDTH) {
                let base_x = 0;
                let base_y = GOAL - 1 + diagonals;
                if map[(base_y - step) * WIDTH + (base_x + step)] == player {
                    count += 1;
                } else {
                    count = 0;
                }
                if count >= GOAL {
                    return true;
                }
            }
        }

        return false;
    }

    fn check_upwards_diagonal(map: board::Board, player: Token) -> bool {
        // left to right
        // iterate over each diagonal that touches y = height
        for diagonals in 0..WIDTH - GOAL + 1 {
            let mut count = 0;
            // iterate along the diagonal
            // top to bottom
            for step in 0..min(GOAL + diagonals, HEIGHT) {
                let base_x = GOAL + diagonals - 1;
                let base_y = HEIGHT - 1;
                if map[(base_y - step) * WIDTH + (base_x - step)] == player {
                    count += 1;
                } else {
                    count = 0;
                }
                if count >= GOAL {
                    return true;
                }
            }
        }

        // right to left
        // iterate over each diagonal that touches x = width (but not y = height)
        for diagonals in 0..HEIGHT - GOAL {
            let mut count = 0;
            // iterate along the diagonal
            // top to bottom
            for step in 0..min(GOAL + diagonals, WIDTH) {
                let base_x = WIDTH - 1;
                let base_y = GOAL - 1 + diagonals;
                if map[(base_y - step) * WIDTH + (base_x - step)] == player {
                    count += 1;
                } else {
                    count = 0;
                }
                if count >= GOAL {
                    return true;
                }
            }
        }

        return false;
    }

    pub fn check_vertical(map: board::Board, turn: Token) -> bool {
        let mut count: usize = 0;
        for column in 0..WIDTH {
            for y in 0..HEIGHT {
                if map[column + y * WIDTH] == turn {
                    count += 1;
                } else {
                    count = 0;
                }
                if count >= 4 {
                    return true;
                }
            }
            count = 0;
        }
        return false;
    }

    pub fn check_horizontal(map: board::Board, turn: Token) -> bool {
        let mut count: usize = 0;
        for row in 0..HEIGHT {
            for x in 0..WIDTH {
                if map[row * HEIGHT + x] == turn {
                    count += 1;
                } else {
                    count = 0;
                }
                if count >= 4 {
                    return true;
                }
            }
            count = 0;
        }
        return false;
    }
}