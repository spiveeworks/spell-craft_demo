use charm_internal::units;

use piston_window as app;

mod clock;
mod draw;
mod game_state;
mod user_input;


pub struct Game {
    state: game_state::GameState,
    time: clock::Stuttering,
    input: user_input::Input,
}



impl Game {
    fn new() -> Game {
        let state = game_state::GameState::new();
        let time = clock::Stuttering::new(state.time.now());
        let input = user_input::Input::new();

        Game { state, time, input }
    }

    fn on_update(&mut self, _upd: UpdateArgs) {
        let now = self.time.now();
        self.state.time.simulate(now);
    }

    fn on_input(&mut self, bin: ButtonArgs) {
    }

    fn on_mouse_move(&mut self, mouse: [f64; 2]) {
    }

    fn on_draw(
        &mut self,
        context: Context,
        graphics: &mut G2d,
        ren: RenderArgs
    ) {
        let now = self.time.now();
        self.time.max_time = now + units::MOMENT;


        clear([0.0, 0.0, 0.0, 1.0], graphics);

        let center = context
            .transform
            .trans(
                (ren.width / 2) as f64,
                (ren.height / 2) as f64
            );
        let position = self.player.body.position(now);
        draw::draw_at(&self.player.shape, position, center, graphics);

        let space = self.space.try_borrow();
        for ent in space.ents {
            // TODO make generic functions for rendering things
            // really the objects should generate a Graphics enum
            // and then Draw should be implemented for the enum itself
            match *ent {
                Smoke(ref item) => {
                    if let Ok(item) = item.try_borrow() {
                        let position = item.loc.body.position(now);
                        draw::draw_at(&item.shape, position, center, graphics);
                    }
                },
                Bolt(ref item) => {
                    if let Ok(item) = item.try_borrow() {
                        let position = item.loc.body.position(now);
                        draw::draw_at(&item.shape, position, center, graphics);
                    }
                },
            }
        }
    }
}

