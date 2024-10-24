mod game;
mod fsutils;
mod display;

fn main() {
    let config_path = "Game.toml";

    let mut game = match game::Game::new(config_path) {
        Ok(game) => game,
        Err(e) => {
            eprintln!("Error loading game: {}", e);
            return;
        }
    };

    println!("Config: {:?}", game.config);
    game.run();
}
