use prelude::*;

use units;
use physics;
use events;

use super::*;

pub enum GrenadeState {
    Cooking {
        target_location: units::Position,
    },
    Smoke,
}

pub struct Grenade {
    space: Link<Space>,
    pub body: physics::Body,
    pub state: GrenadeState,
}

struct GrenadeExplodeEvent {
    target: Link<Grenade>,
}

struct GrenadeDisappearEvent {
    target: Link<Grenade>,
}

impl Grenade {
    pub fn new(
        time: &mut events::EventQueue,
        space: Link<Space>,
        start: units::Position,
        end: units::Position,
        travel_time: units::Duration,
    ) -> Result<Link<Grenade>, Owned<Grenade>> {
        let body = physics::Body::with_end_point(
            start,
            end,
            time.now(),
            travel_time,
        );
        let nade_value = Grenade {
            space: Clone::clone(&space),
            body,
            state: GrenadeState::Cooking { target_location: end },
        };
        let nade = Owned::new(nade_value);

        let end_time = time.now() + travel_time;
        time.enqueue(
            GrenadeExplodeEvent {
                target: Owned::share(&nade)
            },
            end_time
        );

        if let Ok(mut loc) = space.try_borrow_mut() {
            let result = Owned::share(&nade);
            loc.add_entity(nade);
            Ok(result)
        } else {
            Err(nade)
        }
    }
}

impl events::Event for GrenadeExplodeEvent {
    fn invoke(self: Box<Self>, time: &mut events::EventQueue) {
        let target = self.target;

        // update state
        if let Ok(mut nade) = target.try_borrow_mut() {
            // freeze it
            if let GrenadeState::Cooking {
                target_location
            } = nade.state {
                nade.body = physics::Body::new_frozen(target_location)
            }
            // explodify it
            nade.state = GrenadeState::Smoke;
        } else {
            return;
        }

        // let the smoke clear
        let clear_time = time.now() + units::MOMENT;
        time.enqueue(
            GrenadeDisappearEvent {
                target
            },
            clear_time
        );
    }
}

impl events::Event for GrenadeDisappearEvent {
    fn invoke(self: Box<Self>, _time: &mut events::EventQueue) {
        let target = &self.target;
        if let Ok(nade) = target.try_borrow() {
            if let Ok(mut space) = nade.space.try_borrow_mut() {
                space.remove_entity(&nade);
            }
            // TODO unlink from grenade as well
        }
    }
}

