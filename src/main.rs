extern crate piston_window;

use piston_window::*;

use std::time;
use std::ops;

extern crate charm_internal;

use charm_internal::{units, physics, events, entities};
use charm_internal::prelude::*;

const SPEED: units::Scalar = 100;
const MAX_SKIP: units::Time = units::SEC / 16;

struct Player {
    body: physics::Body,
    radius: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Default)]
struct DirPad<T> {
    up: T,
    down: T,
    left: T,
    right: T,
}

impl<T> ops::Index<Dir> for DirPad<T> {
    type Output = T;
    fn index(&self, index: Dir) -> &T {
        match index {
            Dir::Up    => &self.up,
            Dir::Down  => &self.down,
            Dir::Left  => &self.left,
            Dir::Right => &self.right,
        }
    }
}

impl<T> ops::IndexMut<Dir> for DirPad<T> {
    fn index_mut(&mut self, index: Dir) -> &mut T {
        match index {
            Dir::Up    => &mut self.up,
            Dir::Down  => &mut self.down,
            Dir::Left  => &mut self.left,
            Dir::Right => &mut self.right,
        }
    }
}

impl<T> DirPad<T>
    where T: PartialEq
{
    fn dir(&self, item: T) -> Option<Dir> {
        if      item == self.up    { Some( Dir::Up    ) }
        else if item == self.down  { Some( Dir::Down  ) }
        else if item == self.left  { Some( Dir::Left  ) }
        else if item == self.right { Some( Dir::Right ) }
        else { None }
    }
}

fn duration_in_game(duration: time::Duration) -> units::Time {
    let seconds = duration.as_secs();
    let nanos = duration.subsec_nanos();
    let time_s = seconds as units::Time * units::SEC;
    // a billion times the actual time represented by the nanos
    let time_n_bi =   nanos as units::Time * units::SEC;
    time_s + time_n_bi / 1_000_000_000
}

struct Clock {
    start_instant: Option<time::Instant>,
    last_time: units::Time,
}

impl Clock {
    fn new(start_time: units::Time) -> Clock {
        Clock {
            start_instant: None,
            last_time: start_time,
        }
    }

    fn elapsed_as_of(&self, now: time::Instant) -> time::Duration {
        if let Some(start) = self.start_instant {
            now.duration_since(start)
        } else {
            // time only passes if the clock has started
            time::Duration::new(0,0)
        }
    }

    fn time(&self, now: time::Instant) -> units::Time {
        let elapsed = self.elapsed_as_of(now);
        self.last_time + duration_in_game(elapsed)
    }

    fn stop(&mut self, now: time::Instant) {
        self.last_time = self.time(now);
        self.start_instant = None;
    }

    fn start(&mut self, now: time::Instant) {
        self.stop(now);
        self.start_instant = Some(now);
    }
}

struct Game {
    igt: events::EventQueue,  // in-game time
    clock: Clock,
    last_render: units::Time,

    player: Player,
    space: Owned<entities::Space>,

    move_controls: DirPad<Button>,
    fire_button: Button,
    dirs: DirPad<bool>,
    cursor_pos: units::Position,
}

impl Game {
    fn new(player_loc: units::Position) -> Game {
        let igt = events::EventQueue::new();
        let initial_time = igt.now();  // probably 0

        let mut clock = Clock::new(initial_time);
        clock.start(time::Instant::now());


        let body = physics::Body::new_frozen(player_loc);

        let player = Player {
            body,
            radius: 10.0,
        };

        let space = entities::Space::new();
        let space = Owned::new(space);


        let move_controls = DirPad {
            up:    Button::Keyboard(Key::W),
            down:  Button::Keyboard(Key::S),
            left:  Button::Keyboard(Key::A),
            right: Button::Keyboard(Key::D),
        };
        let fire_button = Button::Mouse(MouseButton::Left);

        let dirs = Default::default();
        let cursor_pos = units::ZERO_VEC;

        Game {
            igt,
            clock,
            last_render: initial_time,

            player,
            space,

            move_controls,
            fire_button,
            dirs,
            cursor_pos,
        }
    }

    fn on_update(&mut self, _upd: UpdateArgs) {
        let now = time::Instant::now();
        let mut ig_now = self.clock.time(now);

        // maximum in-game time before rendering again
        let max_time = self.last_render + MAX_SKIP;
        if ig_now > max_time {
            ig_now = max_time;
            self.clock = Clock {
                start_instant: Some(now),
                last_time: max_time,
            }
        }

        self.igt.simulate(ig_now);
    }

    fn update_movement(&mut self, dir: Dir, state: bool) {
        if self.dirs[dir] == state {
            // short circuit to avoid unnecessary rounding
            return;
        }
        self.dirs[dir] = state;

        let mut x = 0;
        let mut y = 0;
        if self.dirs.left  { x -= SPEED }
        if self.dirs.right { x += SPEED }
        if self.dirs.up    { y -= SPEED }
        if self.dirs.down  { y += SPEED }

        if x != 0 && y != 0 {
            x *= 5;
            x /= 7;
            y *= 5;
            y /= 7;
        }

        self.player.body.bounce(units::Vec2 { x, y }, self.igt.now());
    }

    fn fire(&mut self) {
        let time_now = self.igt.now();
        let _result = entities::Grenade::new(
            &mut self.igt,
            Owned::share(&self.space),
            self.player.body.position(time_now),
            self.cursor_pos,
            1 * units::SEC
        );
    }

    fn on_input(&mut self, bin: ButtonArgs) {
        let ButtonArgs { button, state, .. } = bin;
        let state = state == ButtonState::Press;  // true if pressed

        if let Some(dir) = self.move_controls.dir(button) {
            self.update_movement(dir, state);
        }

        if state && button == self.fire_button {
            self.fire();
        }
    }

    fn on_mouse_move(&mut self, mouse: [f64; 2]) {
        let x = (mouse[0] - 300.0) * units::DOT as f64;
        let y = (mouse[1] - 300.0) * units::DOT as f64;
        self.cursor_pos = units::Vec2 {
            x: x as units::Scalar,
            y: y as units::Scalar,
        };
    }

    fn on_draw(
        &mut self,
        context: Context,
        graphics: &mut G2d,
        ren: RenderArgs
    ) {
        self.last_render = self.igt.now();
        clear([0.0, 0.0, 0.0, 1.0], graphics);

        let center = context.transform
                            .trans(
                                (ren.width / 2) as f64,
                                (ren.height / 2) as f64
                            );
        let red = [1.0, 0.0, 0.0, 1.0];
        ellipse(
            red,
            rectangle(&self.player.body, self.player.radius, self.igt.now()),
            center,
            graphics
        );
        let space = self.space.share();
        if let Ok(space) = space.try_borrow() {
            for nade in &space.nades {
                let nade = nade.share();
                if let Ok(nade) = nade.try_borrow() {
                    let (rad, col) = match nade.state {
                        entities::GrenadeState::Cooking{..} => (7.0, [0.0, 0.2, 0.0, 1.0]),
                        entities::GrenadeState::Smoke => (150.0, [1.0, 0.8, 0.0, 1.0]),
                    };
                    ellipse(
                        col,
                        rectangle(&nade.body, rad, self.igt.now()),
                        center,
                        graphics
                    );
                }
                std::mem::drop(nade);
            }
        }
        std::mem::drop(space);
    }
}

fn rectangle(body: &physics::Body, radius: f64, now: units::Time) -> [f64; 4] {
    let units::Vec2{x, y} = body.position(now);
    let x = x as f64 / units::DOT as f64;
    let y = y as f64 / units::DOT as f64;
    [x - radius, y - radius,
        2.0 * radius, 2.0 * radius]
}



fn settings() -> WindowSettings {
    WindowSettings::new(
        "charm-game",
        [600, 600]
    )
}

fn main() {
    let mut game_state = Game::new(units::ZERO_VEC);
    game_state.clock.start(time::Instant::now());

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
        if let Some(mouse) = e.mouse_cursor_args() {
            game_state.on_mouse_move(mouse);
        }
    }
}
