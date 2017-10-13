extern crate piston_window;

use piston_window::*;

mod game;

const SPEED: i32 = 1;

struct Player {
    body: game::Body,
    radius: f64,
}

impl Player {
    fn rectangle(&self, now: game::Time) -> [f64; 4] {
        let game::Vec2{x, y} = self.body.position(now);
        let x = x as f64;
        let y = y as f64;
        [x - self.radius, y - self.radius,
            2.0 * self.radius, 2.0 * self.radius]
    }
}

#[derive(Default)]
struct DirPad<T> {
    up: T,
    down: T,
    left: T,
    right: T,
}

struct Game {
    time: game::Time,
    player: Player,
    controls: DirPad<Button>,
    dirs: DirPad<bool>,
}

impl Game {
    fn new(player_loc: game::Position) -> Game {
        let time = 0;

        let body = game::Body::new(
            player_loc,      // initial location
            game::ZERO_VEC,  // stationary
            time             // any time works since stationary
        );
        let player = Player {
            body,
            radius: 10.0,
        };

        let controls = DirPad {
            up:    Button::Keyboard(Key::W),
            down:  Button::Keyboard(Key::S),
            left:  Button::Keyboard(Key::A),
            right: Button::Keyboard(Key::D),
        };

        let dirs = Default::default();

        Game { time, player, controls, dirs }
    }

    fn on_update(&mut self, upd: UpdateArgs) {
        self.time += (game::SEC as f64 * upd.dt) as game::Time;
    }

    fn update_movement(&mut self) {
        let mut x = 0;
        let mut y = 0;
        if self.dirs.left  { x -= SPEED }
        if self.dirs.right { x += SPEED }
        if self.dirs.up    { y -= SPEED }
        if self.dirs.down  { y += SPEED }

        if x != 0 && y != 0 {
            x *= 7;
            x /= 5;
            y *= 7;
            y /= 5;
        }

        self.player.body.update(game::Vec2 { x, y }, self.time);
    }

    fn on_input(&mut self, bin: ButtonArgs) {
        let ButtonArgs { button, state, .. } = bin;
        let state = state == ButtonState::Press;  // true if pressed

        if        button == self.controls.up {
            self.dirs.up = state;
        } else if button == self.controls.down {
            self.dirs.down = state;
        } else if button == self.controls.left {
            self.dirs.left = state;
        } else if button == self.controls.right {
            self.dirs.right = state;
        } else {
            return;  // don't update player velocity
        }

        self.update_movement();
    }

    fn on_draw(
        &mut self,
        context: Context,
        graphics: &mut G2d,
        ren: RenderArgs
    ) {
        clear([0.0, 0.0, 0.0, 1.0], graphics);

        let center = context.transform
                            .trans(
                                (ren.width / 2) as f64,
                                (ren.height / 2) as f64
                            );
        let red = [1.0, 0.0, 0.0, 1.0];
        ellipse(
            red,
            self.player.rectangle(self.time),
            center,
            graphics
        );
    }
}


fn settings() -> WindowSettings {
    WindowSettings::new(
        "piston-tutorial",
        [600, 600]
    )
}

fn main() {
    let mut game_state = Game::new(game::ZERO_VEC);
    let mut window: PistonWindow =
        settings().exit_on_esc(true)
                  .build()
                  .unwrap_or_else(|e| {
                      panic!("Failed to build PistonWindow: {}", e)
                  });
    while let Some(e) = window.next() {
        if let Some(ren) = e.render_args() {
            window.draw_2d(&e, |c, g| game_state.on_draw(c, g, ren));
        }
        if let Some(upd) = e.update_args() {
            game_state.on_update(upd);
        }
        if let Some(bin) = e.button_args() {
            game_state.on_input(bin);
        }
    }
}
