use super::Piece;
use crate::{
    board::{BagchaalStage, Board, Pieces},
    game_manager::Players,
    position::Position,
};

#[derive(Clone)]
pub struct Goat {
    icon: String,
    pub killed: bool,
    player: Players,
}

impl Goat {
    pub fn new() -> Self {
        Goat {
            icon: String::from("🐐"),
            killed: false,
            player: Players::Goat,
        }
    }
}

impl Piece for Goat {
    // creation

    fn kill(&mut self) {
        self.killed = true;
    }

    // retrieval
    fn display(&self) -> &str {
        &self.icon
    }
    fn is_killed(&self) -> bool {
        self.killed
    }

    // mutation
    fn can_move(
        &self,
        start: &Position,
        end: &Position,
        board: &Board,
    ) -> (bool, Option<Position>) {
        let is_one_step = start.is_adjacent(end, 1);
        let is_diag = start.is_diag(end);

        // too far return false
        if !is_one_step || is_diag && !board.is_white(&start) {
            return (false, None);
        }

        // adjacent and open. move to it.
        match board.get_item(&end) {
            Some(_) => return (false, None),
            None => return (true, None),
        }
    }
    fn is_player(&self, player: &Players) -> bool {
        self.player == *player
    }

    fn no_of_possible_moves(&self, board: &Board) -> i32 {
        // let mut possible_moves = Vec::<(Position, Position)>::new();
        let mut no_of_possible_moves = 0;

        let mut empty = Vec::<Position>::new();
        let mut player = Vec::<Position>::new();

        for (row_index, row) in board.board.iter().enumerate() {
            for (col_index, cell) in row.iter().enumerate() {
                match cell {
                    Some(Pieces::Goat(_)) => player.push(Position {
                        x: row_index,
                        y: col_index,
                    }),
                    None => empty.push(Position {
                        x: row_index,
                        y: col_index,
                    }),
                    Some(_) => {}
                }
            }
        }

        match board.stage {
            BagchaalStage::Graze => no_of_possible_moves = empty.len() as i32,

            BagchaalStage::Chase => {
                for start in player.iter() {
                    for end in empty.iter() {
                        if self.can_move(start, end, board).0 {
                            no_of_possible_moves += 1
                        }
                    }
                }
            }
        }

        no_of_possible_moves
    }
}
