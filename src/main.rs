use std::io;

use connect_four::{
    board,
    constants::{HEIGHT, WIDTH},
    game, Token,
};

pub mod connect_four;

fn main() {
    //create width x height board full of Empty
    let map = board::new();

    //red goes first
    let starting_turn = Token::RED;
    let starting_turn_number = 0;

    loop {
        //start game
        game_loop(map, starting_turn, starting_turn_number);
        println!("Would you like to play again? (Y/N):");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice: String = choice.trim().parse().expect("Not a valid input");
        let choice = choice.as_str();
        match choice {
            "Y" | "y" => continue,
            _ => break,
        }
    }
}

fn game_loop(mut map: board::Board, mut player: Token, mut turn: usize) {
    loop {
        //take turn input
        let play = game::take_turn(&mut map, player);

        //increment turn number
        turn += 1;

        //display game board
        board::print(map);

        if game::check_win(map, player, play) {
            let color = match player {
                Token::RED => "Red",
                Token::BLUE => "Blue",
                _ => panic!("Invalid Player state"),
            };
            println!("Congratulations {color} player! You won!");
            break;
        }

        //end game if board is full
        if turn >= WIDTH * HEIGHT {
            println!("Congratulations on the tie!");
            break;
        }

        //change who's turn it is
        match player {
            Token::BLUE => player = Token::RED,
            Token::RED => player = Token::BLUE,
            _ => panic!("Invalid player state"),
        }
    }
}
