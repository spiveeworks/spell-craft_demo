extern crate piston_window;

use piston_window::*;

mod game;

struct Player {
    body: game::Body,
}

struct Game {
    time: game::Time,
    player: Player,
}

impl Game {
    fn new(player_loc: game::Position) -> Game {
        let time = 0;
        let body = game::Body::new(
            player_loc,      // initial location
            game::ZERO_VEC,  // stationary
            time             // any time works since stationary
        );
        let player = Player { body };
        Game { time, player }
    }
}


fn settings() -> WindowSettings {
    WindowSettings::new(
        "piston-tutorial",
        [600, 600]
    )
}

fn main() {
    let game_state = Game::new(game::ZERO_VEC);
    let mut window: PistonWindow =
        settings().exit_on_esc(true)
                  .build()
                  .expect("Failed to build window from settings.");
    while let Some(_e) = window.next() {

    }
}
