use std::rc;

use events;
use units;
use physics;
use entities::spaces;


pub trait Cast {
    fn cast(
        self: &Self,
        time: &mut events::EventQueue,
        space: Link<spaces::Space>,
        ref_frame: physics::Body,
        target: units::Position,
    );
}

pub trait Effect {
    fn color(self: &Self) -> [u8; 4];
}




// maybe store coords in the space itself?
pub struct Location {
    pub space: Link<spaces::Space>,
    pub body: physics::Body,
}

// TODO use Link<spaces::Container<T>> instead
impl Location {
    fn remove<T>(
        loc_mut: &mut links::RefMut<Location, T>  // does this really need to be borrowed?
    ) -> Option<Owned<T>>
        where spaces::Space: spaces::Container<T>
    {
        use entities::spaces::Container;

        let as_owned = loc_mut
            .space
            .try_borrow_mut()
            .ok()
            .and_then(|mut space| {
                let compare = links::RefMut::compare_source(loc_mut);
                space.remove_entity(compare)
            });
        if as_owned.is_some() {
            loc_mut.space = Link::new();
        }
        as_owned
    }

    // construct an object out of an Location object, and link it to the space in the Location
    fn create_entity<T, F>(
        space: Link<spaces::Space>,
        body: physics::Body,
        f: F,
    ) -> Result<Link<T>, Owned<T>>
        where F: FnOnce(Location) -> T,
              spaces::Space: spaces::Container<T>,
    {
        use entities::spaces::Container;

        let space2 = space.clone();
        let loc = Location { space, body };
        let obj = f(loc);
        {
            let space_borrow = space2.try_borrow_mut();
            if let Ok(mut space) = space_borrow {
                Ok(space.add_value(obj))
            } else {
                Err(Owned::new(obj))
            }
        }
    }
}


#[derive(Clone)]
pub struct Circle {
    pub color: rc::Rc<Effect>,
    pub radius: units::Scalar,
}

pub struct Smoke {
    pub loc: Location,
    pub shape: Circle,
}

pub struct SmokeClearEvent {
    target: Link<Smoke>
}

impl events::Event for SmokeClearEvent {
    fn invoke(self: Box<Self>, _time: &mut events::EventQueue) {
        let target = &self.target;
        if let Ok(smoke) = target.try_borrow_mut() {
            let mut loc = links::RefMut::map(smoke, |smoke_data| &mut smoke_data.loc);
            Location::remove(&mut loc);
        }
    }
}

pub struct SmokeCast {
    pub shape: Circle,
    pub duration: units::Duration,
}

impl Cast for SmokeCast {
    fn cast(
        self: &Self,
        time: &mut events::EventQueue,
        space: Link<spaces::Space>,
        ref_frame: physics::Body,
        _target: units::Position,
    ) {
        let body = physics::Body::new_frozen(ref_frame.position(time.now()));
        let shape = self.shape.clone();
        let smoke_fn = move |loc| Smoke { loc, shape };
        let smoke = Location::create_entity(space, body, smoke_fn);

        if let Ok(target) = smoke {
            time.enqueue(
                SmokeClearEvent { target },
                self.duration,
            );
        }
    }
}


pub struct Bolt {
    pub loc: Location,
    pub shape: Circle,
    action: rc::Rc<Cast>,
}

// deletes bolt, and casts action
pub struct BoltLandEvent {
    target: Link<Bolt>
}

impl events::Event for BoltLandEvent {
    fn invoke(self: Box<Self>, time: &mut events::EventQueue) {
        if let Ok(bolt) = self.target.try_borrow_mut() {
            let bolt_owned = {
                // remove it from space
                let mut loc = links::RefMut::map(bolt, |bolt| &mut bolt.loc);
                Location::remove(&mut loc)
                    .expect("Bolt with bad space pointer")
            };

            // cast the next action
            let bolt = bolt_owned.try_borrow().ok().unwrap();  // TODO use borrow() instead (nyi)
            bolt.action.cast(
                time,
                bolt.loc.space.clone(),
                bolt.loc.body.clone(),
                units::ZERO_VEC,
            );
        }
    }
}

pub struct BoltCast {
    pub shape: Circle,
    pub duration: units::Duration,
    pub action: rc::Rc<Cast>,
}


impl Cast for BoltCast {
    fn cast(
        self: &Self,
        time: &mut events::EventQueue,
        space: Link<spaces::Space>,
        ref_frame: physics::Body,
        target: units::Position,
    ) {
        let body = physics::Body::with_end_point(
            ref_frame.position(time.now()),
            target,
            time.now(),
            self.duration,
        );
        let shape = self.shape.clone();
        let action = rc::Rc::clone(&self.action);
        let bolt_fn = move |loc| Bolt { loc, shape, action };
        let bolt = Location::create_entity(space, body, bolt_fn);

        if let Ok(target) = bolt {
            time.enqueue(
                BoltLandEvent { target },
                self.duration,
            );
        }
    }
}
