use crate::game_manager::Players;
use crate::piece::goat::Goat;
use crate::piece::tiger::Tiger;
use crate::piece::Piece;
use crate::position::Position;

#[derive(Clone)]
pub enum Pieces {
    Goat(Goat),
    Tiger(Tiger),
}

impl Piece for Pieces {
    fn display(&self) -> &str {
        match self {
            Pieces::Goat(g) => g.display(),
            Pieces::Tiger(t) => t.display(),
        }
    }
    fn is_killed(&self) -> bool {
        match self {
            Pieces::Goat(g) => g.is_killed(),
            Pieces::Tiger(t) => t.is_killed(),
        }
    }
    fn kill(&mut self) {
        match self {
            Pieces::Goat(g) => g.kill(),
            Pieces::Tiger(t) => t.kill(),
        }
    }
    fn can_move(&self, a: &Position, b: &Position, board: &Board) -> (bool, Option<Position>) {
        match self {
            Pieces::Goat(g) => g.can_move(a, b, board),
            Pieces::Tiger(t) => t.can_move(a, b, board),
        }
    }
    fn is_player(&self, player: &Players) -> bool {
        match self {
            Pieces::Goat(g) => g.is_player(player),
            Pieces::Tiger(t) => t.is_player(player),
        }
    }
    fn no_of_possible_moves(&self, board: &Board) -> i32 {
        match self {
            Pieces::Goat(g) => g.no_of_possible_moves(board),
            Pieces::Tiger(t) => t.no_of_possible_moves(board),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum BagchaalStage {
    Graze,
    Chase,
}

pub struct Board {
    pub board: [[Option<Pieces>; 5]; 5],
    black: String,
    white: String,
    pub stage: BagchaalStage,
    goat_in_board: i32,
    dead_goats: i32,
}

impl Board {
    pub fn new() -> Self {
        Board {
            board: [
                [
                    Some(Pieces::Tiger(Tiger::new())),
                    None,
                    None,
                    None,
                    Some(Pieces::Tiger(Tiger::new())),
                ],
                [None, None, None, None, None],
                [None, None, None, None, None],
                [None, None, None, None, None],
                [
                    Some(Pieces::Tiger(Tiger::new())),
                    None,
                    None,
                    None,
                    Some(Pieces::Tiger(Tiger::new())),
                ],
            ],
            black: String::from("\x1b[30;40m"),
            white: String::from("\x1b[30;47m"),
            stage: BagchaalStage::Graze,
            goat_in_board: 0,
            dead_goats: 0,
        }
    }
    pub fn render(&self) {
        let mut index = 1;

        if self.stage == BagchaalStage::Graze {
            println!("Grazing!!!");
            for _ in 0..(20 - self.goat_in_board) {
                print!("🐐")
            }
            print!("\n")
        } else {
            println!("Chasing!!!");
        }

        for (x, row) in self.board.iter().enumerate() {
            for (y, item) in row.iter().enumerate() {
                let index_print = format!("{:02}", index);
                print!(
                    "{} {} ",
                    if self.is_white(&Position { x, y }) {
                        &self.white
                    } else {
                        &self.black
                    },
                    match item {
                        Some(item) => item.display(),
                        None => &index_print,
                    }
                );
                index += 1;
            }
            print!("\x1b[0m\n")
        }
        if self.dead_goats > 0 {
            println!("Dead goats");
            for _ in 0..self.dead_goats {
                print!("🐐")
            }
            print!("\n")
        }
    }
    pub fn get_item(&self, position: &Position) -> &Option<Pieces> {
        &self.board[position.x][position.y]
    }
    fn set_item(&mut self, position: &Position, value: Pieces) {
        self.board[position.x][position.y] = Some(value);
    }
    fn remove_item(&mut self, position: &Position) {
        let item = &mut self.board[position.x][position.y];
        match item {
            Some(item) => {
                item.kill();
                self.board[position.x][position.y] = None
            }
            None => return,
        }
    }

    pub fn is_white(&self, position: &Position) -> bool {
        (position.x + position.y) % 2 == 0
    }
    fn add(&mut self, position: &Position, item: Pieces) -> bool {
        let current_item = &self.board[position.x][position.y];
        match current_item {
            Some(_) => return false,
            None => {
                self.set_item(position, item);
                return true;
            }
        }
    }
    fn move_piece(&mut self, start: &Position, end: &Position) -> bool {
        let item = self.board[start.x][start.y].clone();

        if self.out_of_board(*start) || self.out_of_board(*end) {
            return false;
        }

        match item {
            Some(item) => {
                let (can_move, position_to_kill) = item.can_move(start, end, self);
                if can_move {
                    self.set_item(end, item);
                    self.remove_item(start);
                    match position_to_kill {
                        Some(position) => {
                            self.remove_item(&position);
                            self.dead_goats += 1;
                        }
                        None => {}
                    }
                    return true;
                }
                false
            }
            None => return false,
        }
    }

    pub fn out_of_board(&self, pos: Position) -> bool {
        pos.x >= 5 || pos.y >= 5
    }

    pub fn new_move(&mut self, start: Position, end: Position, player: &Players) -> bool {
        let item = self.get_item(&start);

        if self.out_of_board(start) {
            return false;
        }

        let mut flag_can_move = true;

        match item {
            Some(item) => {
                if !item.is_player(player) {
                    flag_can_move = false;
                }
            }
            None => flag_can_move = false,
        }

        if self.stage == BagchaalStage::Graze && *player == Players::Goat {
            let flag = self.add(&start, Pieces::Goat(Goat::new()));
            if flag {
                self.goat_in_board += 1;
                if self.goat_in_board >= 20 {
                    self.stage = BagchaalStage::Chase
                }
                return true;
            }
            return false;
        }

        if flag_can_move {
            return self.move_piece(&start, &end);
        }
        false
    }
    pub fn no_of_possible_moves(&self, player: &Players) -> i32 {
        match player {
            Players::Goat => {
                let goat = Goat::new();
                goat.no_of_possible_moves(self)
            }
            Players::Tiger => {
                let tiger = Tiger::new();
                tiger.no_of_possible_moves(self)
            }
        }
    }
}

pub fn index_to_position(index: i32) -> Position {
    Position {
        y: usize::try_from(index % 5).unwrap_or(100),
        x: usize::try_from(index / 5).unwrap_or(100),
    }
}
