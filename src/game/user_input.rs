use std::ops;

use charm_internal::units;

use game::game_state;

use piston_window as app;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Default)]
pub struct DirPad<T> {
    pub up: T,
    pub down: T,
    pub left: T,
    pub right: T,
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




pub struct Input {
    move_controls: DirPad<app::Button>,
    fire_button: app::Button,
    dirs: DirPad<bool>,
    cursor_pos: units::Position,
}

impl Input {
    pub fn new() -> Input {
        let move_controls = DirPad {
            up:    app::Button::Keyboard(app::Key::W),
            down:  app::Button::Keyboard(app::Key::S),
            left:  app::Button::Keyboard(app::Key::A),
            right: app::Button::Keyboard(app::Key::D),
        };
        let fire_button = app::Button::Mouse(app::MouseButton::Left);

        let dirs = Default::default();
        let cursor_pos = units::ZERO_VEC;

        Input {
            move_controls,
            fire_button,
            dirs,
            cursor_pos,
        }
    }

    // TODO g_state? game_state is a module... this happens a lot
    pub fn on_input(&mut self, g_state: &mut game_state::GameState, bin: app::ButtonArgs) {
        let app::ButtonArgs { button, state, .. } = bin;
        let butt_pressed = state == app::ButtonState::Press;

        if let Some(dir) = self.move_controls.dir(button) {
            // short circuit to avoid unnecessary updates/rounding
            if self.dirs[dir] != butt_pressed {
                self.dirs[dir] = butt_pressed;
                g_state.update_movement(&self.dirs);
            }
        }

        if butt_pressed && button == self.fire_button {
            g_state.fire(self.cursor_pos);
        }
    }

    pub fn on_mouse_move(&mut self, mouse: [f64; 2]) {
        let x = (mouse[0] - 300.0) * units::DOT as f64;
        let y = (mouse[1] - 300.0) * units::DOT as f64;
        self.cursor_pos = units::Vec2 {
            x: x as units::Scalar,
            y: y as units::Scalar,
        };
    }
}

