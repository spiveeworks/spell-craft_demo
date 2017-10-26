use charm_internal::units;

use piston_window as app;

mod clock;
mod draw;
mod game_state;
mod user_input;


pub struct Game {
    state: game_state::GameState,
    real_time: clock::Stuttering,
    input: user_input::Input,
}



impl Game {
    pub fn new() -> Game {
        let state = game_state::GameState::new();
        let real_time = clock::Stuttering::new(state.time.now());
        let input = user_input::Input::new();

        Game { state, real_time, input }
    }

    pub fn on_update(&mut self, _upd: app::UpdateArgs) {
        let now = self.real_time.time();
        self.state.simulate(now);
    }

    pub fn on_input(&mut self, bin: app::ButtonArgs) {
        self.input.on_input(&mut self.state, bin);
    }

    pub fn on_mouse_move(&mut self, mouse: [f64; 2]) {
        self.input.on_mouse_move(mouse);
    }

    pub fn on_draw(
        &mut self,
        context: app::Context,
        graphics: &mut app::G2d,
        ren: app::RenderArgs
    ) {
        // methods for operating on our 2d matrices
        use piston_window::Transformed;

        let now = self.state.time.now();
        self.real_time.max_time = now + units::MOMENT;


        app::clear([0.0, 0.0, 0.0, 1.0], graphics);

        let center = context
            .transform
            .trans(
                (ren.width / 2) as f64,
                (ren.height / 2) as f64
            );
        let position = self.state.player.body.position(now);
        draw::draw_at(&self.state.player.shape, position, center, graphics);

        for (&_uid, ent) in &self.state.space {
            // TODO make generic functions for rendering things
            // really the objects should generate a Graphics enum
            // and then Draw should be implemented for the enum itself
            use charm_internal::entity_heap::Entity::{Smoke, Bolt};
            match *ent {
                Smoke(ref item) => {
                    let position = item.body.position(now);
                    draw::draw_at(&item.shape, position, center, graphics);
                },
                Bolt(ref item) => {
                    let position = item.body.position(now);
                    draw::draw_at(&item.shape, position, center, graphics);
                },
            }
        }
    }
}

