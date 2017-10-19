pub struct Entity {
    space: Link<Space>,
    body: physics::Body,
}

pub fn remove_entity(
    entity_borrow: &mut links::RefMut<Entity, T>
) -> Option<Owned<T>> {
    let as_owned = entity_borrow
        .space
        .try_borrow_mut()
        .map(|space| {
            let compare = links::RefMut::compare_source(entity_borrow);
            space.remove_entity(compare)
        });
    if as_owned.is_some() {
        entity_borrow.space = Link::new();
    }
    as_owned
}


#[derive(Clone)]
pub struct Circle {
    color: rc::Rc<Effect>,
    radius: units::Scalar,
}

pub struct Smoke {
    loc: Entity,
    shape: rc::Rc<Circle>,
}

pub struct SmokeClearEvent {
    target: Link<Smoke>
}

impl Event for SmokeClearEvent {
    fn invoke(self: Box<Self>, _time: &mut events::EventQueue) {
        let target = &self.target;
        if let Ok(smoke) = target.try_borrow_mut() {
            let loc = links::RefMut::map(|smoke_data| &mut smoke_data.loc);
            remove_entity(loc);
        }
    }
}

pub struct SmokeCast {
    shape: Circle,
    duration: units::Duration,
}

impl Cast for SmokeCast {
    fn cast(
        self: &Self,
        time: &mut events::EventQueue,
        space: Link<Space>,
        ref_frame: physics::Body,
        target: units::Position,
    ) {
        let mut space_borrow = space.try_borrow_mut();
        let body = ref_frame.frozen(time.now());
        let loc = Entity { space, body };
        let shape = self.shape.clone();
        let smoke = Smoke { loc, shape };
        let smoke = Owned::new(smoke);

        time.enqueue(
            SmokeClearEvent { target: smoke.share() },
            self.duration,
        );

        if let Ok(mut space) = space_borrow_mut {
            space.add_entity(smoke.share());
        }
    }
}


pub struct Bolt {
    loc: Entity,
    shape: Circle,
    action: rc::Rc<Cast>,
}

// deletes bolt, and casts action
pub struct BoltLandEvent {
    target: Link<Bolt>
}

impl events::Event for BoltLandEvent {
    fn invoke(self: Box<Self>, time: &mut events::EventQueue) {
        if let Ok(bolt) = self.target.try_borrow_mut() {
            {
                // remove it from space
                let loc = links::RefMut::map(|bolt| &mut bolt.loc);
                remove_entity(&mut loc);
            }

            // cast the next action
            let bolt = self.target.borrow();  // since we just succeeded
            bolt.action.cast(
                time,
                bolt.loc.space,
                bolt.loc.body,
                units::ZERO_VEC,
            );
        }
    }
}

