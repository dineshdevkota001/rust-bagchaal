use game_manager::GameManager;

mod board;
mod game_manager;
mod input_manager;
mod piece;
mod position;

fn main() {
    let mut game = GameManager::new();
    loop {
        game.game_loop()
    }
}
