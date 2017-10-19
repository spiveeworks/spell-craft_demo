
pub trait Cast {
    fn cast(
        self: &Self,
        time: &mut events::EventQueue,
        space: Link<Space>,
        ref_frame: physics::Body,
        target: units::Position,
    );
}

pub trait Effect {
    fn color(self: &Self) -> [u8; 4];
}


