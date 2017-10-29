use std::ops;

use charm_internal::units;

use game::grenade_builder;

use piston_window as app;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Default)]
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



pub enum DeviceUpdate {
    Nop,
    Cast {
        target: units::Position
    },
    ChangeMovement {
        dirs: DirPad<bool>
    },
    AddToCluster {
        target: units::Position
    },
    ArsenalUpdate {
        upd: ::game::grenade_builder::ArsenalUpdate
    },
}

pub struct Input {
    move_controls: DirPad<app::Button>,
    fire_button: app::Button,
    cluster_buffer_button: app::Button,
    arsenal_registers: [app::Button; 9],
    build_basic: app::Button,
    build_cluster: app::Button,
    grenade_settings: [app::Button; 12],
    save_mode: app::Button,

    in_save_mode: bool,
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

        let cluster_buffer_button = app::Button::Mouse(app::MouseButton::Right);

        let arsenal_registers = [
            app::Button::Keyboard(app::Key::D1),
            app::Button::Keyboard(app::Key::D2),
            app::Button::Keyboard(app::Key::D3),
            app::Button::Keyboard(app::Key::D4),
            app::Button::Keyboard(app::Key::D5),
            app::Button::Keyboard(app::Key::D6),
            app::Button::Keyboard(app::Key::D7),
            app::Button::Keyboard(app::Key::D8),
            app::Button::Keyboard(app::Key::D9),
        ];

        let build_basic = app::Button::Keyboard(app::Key::D0);

        let build_cluster = app::Button::Keyboard(app::Key::Equals);

        let grenade_settings = [
            // red
            app::Button::Keyboard(app::Key::N),
            app::Button::Keyboard(app::Key::H),
            app::Button::Keyboard(app::Key::Y),

            // green
            app::Button::Keyboard(app::Key::M),
            app::Button::Keyboard(app::Key::J),
            app::Button::Keyboard(app::Key::U),

            // blue
            app::Button::Keyboard(app::Key::Comma),
            app::Button::Keyboard(app::Key::K),
            app::Button::Keyboard(app::Key::I),

            // radius
            app::Button::Keyboard(app::Key::Period),
            app::Button::Keyboard(app::Key::L),
            app::Button::Keyboard(app::Key::O),
        ];

        let save_mode = app::Button::Keyboard(app::Key::LShift);

        let in_save_mode = false;
        let dirs = Default::default();
        let cursor_pos = units::ZERO_VEC;

        Input {
            move_controls,
            fire_button,
            cluster_buffer_button,
            arsenal_registers,
            build_basic,
            build_cluster,
            grenade_settings,
            save_mode,

            in_save_mode,
            dirs,
            cursor_pos,
        }
    }

    fn interpret_gb(
        self: &mut Self,
        button: app::Button,
        butt_pressed: bool,
    ) -> Option<grenade_builder::ArsenalUpdate> {
        if !butt_pressed {
            return None;
        }
        let upd = {
            use game::grenade_builder::ArsenalUpdate::*;
            let register = self.arsenal_registers
                               .iter()
                               .position(|reg| *reg == button);
            let setting = self.grenade_settings
                              .iter()
                              .position(|reg| *reg == button);
            if let Some(which) = register {
                if self.in_save_mode {
                    SaveNade { which }
                } else {
                    LoadNade { which }
                }
            } else if let Some(setting) = setting {
                let which = setting / 3;
                let level_num = setting % 3;
                let level = match level_num {
                    0 => grenade_builder::Level::Low,
                    1 => grenade_builder::Level::Medium,
                    2 => grenade_builder::Level::High,
                    _ => unreachable!(),
                };

                SetLevel { which, level }
            } else if button == self.build_basic {
                BuildBasic
            } else if button == self.build_cluster {
                BuildCluster
            } else {
                return None;
            }
        };
        Some(upd)
    }


    pub fn interpret(
        &mut self,
        bin: app::ButtonArgs
    ) -> DeviceUpdate {
        let app::ButtonArgs { button, state, .. } = bin;
        let butt_pressed = state == app::ButtonState::Press;

        if button == self.save_mode {
            self.in_save_mode = butt_pressed;
            DeviceUpdate::Nop
        } else if let Some(dir) = self.move_controls.dir(button) {
            // short circuit to avoid unnecessary updates/rounding
            if self.dirs[dir] != butt_pressed {
                self.dirs[dir] = butt_pressed;
                DeviceUpdate::ChangeMovement { dirs: self.dirs.clone() }
            } else {
                DeviceUpdate::Nop
            }
        } else if butt_pressed && button == self.fire_button {
            DeviceUpdate::Cast { target: self.cursor_pos }
        } else if butt_pressed && button == self.cluster_buffer_button {
            DeviceUpdate::AddToCluster { target: self.cursor_pos }
        } else if let Some(upd) = self.interpret_gb(button, butt_pressed) {
            DeviceUpdate::ArsenalUpdate { upd }
        } else {
            DeviceUpdate::Nop
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

