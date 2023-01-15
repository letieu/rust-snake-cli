use std::io::stdout;

use game::Game;

mod game;
mod snake;
mod point;
mod direction;
mod command;

fn main() {
    Game::new(stdout(), 70, 20).run();
}
