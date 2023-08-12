use std::fmt::Display;

use crate::{
    board::{index_to_position, Board},
    input_manager::InputManager,
};

#[derive(Clone, PartialEq, Debug)]
pub enum Players {
    Tiger,
    Goat,
}
impl Display for Players {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str: &str;
        match self {
            Players::Tiger => str = "Tiger",
            Players::Goat => str = "Goat",
        }
        write!(f, "{}", str)
    }
}
pub struct GameManager {
    pub current_player: Players,
    board: Board,
    input_manager: InputManager,
}

impl GameManager {
    pub fn new() -> Self {
        GameManager {
            board: Board::new(),
            input_manager: InputManager::new(),
            current_player: Players::Goat,
        }
    }
    fn change_player(&mut self) {
        match self.current_player {
            Players::Goat => self.current_player = Players::Tiger,
            Players::Tiger => self.current_player = Players::Goat,
        }
    }
    pub fn game_loop(&mut self) {
        self.board.render();
        let possible_moves = self.board.no_of_possible_moves(&self.current_player);
        if possible_moves == 0 {
            self.change_player();
            println!("{} wins", self.current_player);
        } else {
            println!(
                "{} moves possible for {}",
                possible_moves, self.current_player
            )
        }

        self.input_manager.ask_input();
        println!("{}[2J", 27 as char);
        println!("Current Input: {}", self.input_manager.user_input);

        if self.input_manager.check_reset() {
            println!("{}, surrenders", self.current_player);
            self.change_player();
            println!("{}, wins", self.current_player);
            println!("\n \n \n New game.......");
            self.reset();
            return;
        }

        let [start_index, end_index] = self.input_manager.input_to_indices();
        match start_index {
            -1 => {
                println!("Absolutely wrong");
            }
            _ => {
                let start = index_to_position(start_index);
                let end = index_to_position(end_index);

                if start.x < 5 && start.y < 5 {
                    if self.board.new_move(start, end, &self.current_player) {
                        self.change_player()
                    };
                }
            }
        }
    }
    pub fn reset(&mut self) {
        self.board = Board::new();
        self.current_player = Players::Goat;
    }
}
