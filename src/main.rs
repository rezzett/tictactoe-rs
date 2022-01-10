use game::Game;

mod constants;
mod desk;
mod game;

fn main() {
    let mut game = Game::new();
    game.run();
}
