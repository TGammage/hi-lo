use std::io;
use crate::game::Game;

mod utils;
mod game;
mod deck;
mod card;

fn main() {
    let mut end_program = false;

    while !end_program {
        Game::start();

        println!("Would you like to (r)estart another game?");

        let mut restart: String = String::new();
        io::stdin().read_line(&mut restart).expect("Failed to read line.");

        if restart.trim().to_lowercase() != "r".to_string() {
            end_program = true;
        }

        println!("Goodbye!");
    }
}
