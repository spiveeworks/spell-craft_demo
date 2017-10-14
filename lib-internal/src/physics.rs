use units;

#[derive(Clone, Debug)]
pub struct Body {
    last_position: units::Position,
    current_velocity: units::Velocity,
    last_time: units::Time,
}

impl Body {
    pub fn new(
        position: units::Position,
        velocity: units::Velocity,
        time: units::Time
    ) -> Body {
        Body {
            last_position: position,
            current_velocity: velocity,
            last_time: time,
        }
    }

    pub fn with_end_point(
        start: units::Position,
        end: units::Position,
        start_time: units::Time,
        travel_time: units::Duration,
    ) -> Body {
        if travel_time != 0 {
            Body {
                last_position: start,
                current_velocity: (end - start) / travel_time,
                last_time: start_time,
            }
        } else {
            Body::new_frozen(end)
        }
    }

    pub fn new_frozen(position: units::Position) -> Body {
        Body {
            last_position: position,
            current_velocity: units::ZERO_VEC,
            last_time: 0,
        }
    }

    pub fn position(&self, now: units::Time) -> units::Position {
        let dtime = now - self.last_time;
        let displacement = self.current_velocity * dtime;
        self.last_position + displacement
    }

    pub fn velocity(&self) -> units::Velocity {
        self.current_velocity
    }

    pub fn split(
        &self,
        velocity: units::Velocity,
        now: units::Time
    ) -> Body {
        Body {
            last_position: self.position(now),
            current_velocity: velocity,
            last_time: now,
        }
    }

    pub fn split_to(
        &self,
        end_point: units::Vec2,
        now: units::Time,
        end_time: units::Time
    ) -> Body {
        Body::with_end_point(
            self.position(now),
            end_point,
            now,
            end_time
        )
    }

    pub fn bounce(
        &mut self,
        velocity: units::Velocity,
        now: units::Time
    ) {
        *self = self.split(velocity, now);
    }

    pub fn bounce_to(
        &mut self,
        end_point: units::Vec2,
        now: units::Time,
        end_time: units::Time
    ) {
        *self = self.split_to(end_point, now, end_time);
    }

    pub fn freeze(&mut self, now: units::Time) {
        self.bounce(units::Vec2 { x: 0, y: 0 }, now);
    }
}



