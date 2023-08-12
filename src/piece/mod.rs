use crate::{board::Board, game_manager::Players, position::Position};

pub mod goat;
pub mod tiger;

pub trait Piece {
    fn kill(&mut self);
    fn is_killed(&self) -> bool;
    fn display(&self) -> &str;
    fn can_move(&self, start: &Position, end: &Position, board: &Board)
        -> (bool, Option<Position>);
    fn no_of_possible_moves(&self, board: &Board) -> i32;
    fn is_player(&self, player: &Players) -> bool;
}
