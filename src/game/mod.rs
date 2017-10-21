
mod real_time;


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

        self.state.draw(context, graphics, ren);
    }


