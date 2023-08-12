use super::Piece;
use crate::{
    board::{Board, Pieces},
    game_manager::Players,
    position::Position,
};

#[derive(Clone)]
pub struct Tiger {
    icon: String,
    killed: bool,
    player: Players,
}

impl Tiger {
    pub fn new() -> Self {
        Tiger {
            icon: String::from("🐯"),
            killed: false,
            player: Players::Tiger,
        }
    }
}

impl Piece for Tiger {
    // creation
    fn kill(&mut self) {}

    // retrieval
    fn display(&self) -> &str {
        return &self.icon;
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
        let is_two_step = start.is_adjacent(end, 2);
        let is_one_step = start.is_adjacent(end, 1);
        let is_diag = start.is_diag(end);

        let false_value = (false, None);

        if !is_one_step && !is_two_step {
            return false_value;
        }

        // diagonal but not black. cannot move. white ones cannot move diagonally
        if !board.is_white(start) && is_diag {
            return false_value;
        }
        // adjacent and open. move to it.
        if is_one_step {
            match board.get_item(&end) {
                Some(_) => return false_value,
                None => return (true, None),
            }
        }

        // double and has goat in between jump and remove the goat.
        if is_two_step {
            match board.get_item(&end) {
                Some(_) => {
                    return false_value;
                }
                None => {
                    let mid = start.clone().mid(*end);
                    match mid {
                        Some(mid) => match board.get_item(&mid) {
                            Some(Pieces::Goat(_)) => return (true, Some(mid)),
                            _ => {
                                return false_value;
                            }
                        },
                        None => {
                            return false_value;
                        }
                    }
                }
            }
        }

        false_value
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
                    Some(Pieces::Tiger(_)) => player.push(Position {
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

        for start in player.iter() {
            for end in empty.iter() {
                if self.can_move(start, end, board).0 {
                    no_of_possible_moves += 1
                }
            }
        }

        no_of_possible_moves
    }
}
