use std::io;
use crate::deck::Deck;
use crate::card::Card;
use crate::utils::clear_terminal;


pub struct Game {}

impl Game {
  pub fn start() {
    const ROUNDS: u8 = 5;
    const GUESS_PROMPT: &str = "Is the next card (h)igher or (l)ower?";
    const HIGHER: &str = "h";
    const LOWER: &str = "l";

	let mut deck: Deck = Deck::new();
	let mut lost: bool = false;
	let mut current_round: u8 = 1;

	let mut drawn: Card = deck.draw().unwrap();

	while !lost && current_round < ROUNDS {
		clear_terminal();

		println!("Current Card:{}\n{}", drawn.to_text(), GUESS_PROMPT);

        let mut guess: String = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: &str = &guess.trim().to_lowercase();

        if guess != HIGHER && guess != LOWER {
            println!("...incorrect input for guess: {guess}");
            continue;
        }

        let next_drawn = deck.draw().unwrap();

        lost = (guess == HIGHER && drawn.value > next_drawn.value) || (guess == LOWER && drawn.value < next_drawn.value);

        drawn = next_drawn;

        current_round += 1;
	}

	clear_terminal();

	if lost {
        println!("Incorrect guess, next card was {}. Game Over", drawn.to_text());
    } else {
        println!("Last Card:{}\nVictory! Congratulations", drawn.to_text());
    }
  }
}