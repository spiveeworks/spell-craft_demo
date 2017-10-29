use std::rc;

use entity_heap;
use event_queue;
use units;
use physics;


pub trait Cast {
    fn cast(
        self: &Self,
        space: &mut entity_heap::EntityHeap,
        time: &mut event_queue::EventQueue,
        ref_frame: physics::Body,
        target: units::Position,
    );
}

pub trait Effect {
    fn color(self: &Self) -> [u8; 4];
}



#[derive(Clone)]
pub struct Circle {
    pub color: rc::Rc<Effect>,
    pub radius: units::Scalar,
}

pub struct Smoke {
    pub body: physics::Body,
    pub shape: Circle,
}

pub struct SmokeClearEvent {
    target: entity_heap::UID
}

impl event_queue::Event for SmokeClearEvent {
    fn invoke(
        self: Self,
        space: &mut entity_heap::EntityHeap,
        _time: &mut event_queue::EventQueue
    ) {
        let smoke: Smoke = space.remove(&self.target)
                                .expect("SmokeClearEvent called on nonexistent entity")
                                .expect("Smoke for SmokeClearEvent");
        drop(smoke);
    }
}

pub struct SmokeCast {
    pub shape: Circle,
    pub duration: units::Duration,
}

impl Cast for SmokeCast {
    fn cast(
        self: &Self,
        space: &mut entity_heap::EntityHeap,
        time: &mut event_queue::EventQueue,
        ref_frame: physics::Body,
        _target: units::Position,
    ) {
        let body = physics::Body::new_frozen(ref_frame.position(time.now()));
        let shape = self.shape.clone();
        let smoke = Smoke { body, shape };

        let uid = entity_heap::new_entity(space, smoke);

        let target = uid;
        let event = SmokeClearEvent { target };

        time.enqueue(
            event,
            self.duration,
        );
    }
}


pub struct Bolt {
    pub body: physics::Body,
    pub shape: Circle,
    action: rc::Rc<Cast>,
}

// deletes bolt, and casts action
pub struct BoltLandEvent {
    target: entity_heap::UID
}

impl event_queue::Event for BoltLandEvent {
    fn invoke(
        self: Self,
        space: &mut entity_heap::EntityHeap,
        time: &mut event_queue::EventQueue
    ) {
        let bolt: Bolt = space.remove(&self.target)
                              .expect("BoltLandEvent called on nonexistent entity")
                              .expect("Bolt for BoltLandEvent");
        let loc = bolt.body.position(time.now());

        bolt.action.cast(
            space,
            time,
            bolt.body,
            loc,
        );
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
        space: &mut entity_heap::EntityHeap,
        time: &mut event_queue::EventQueue,
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
        let bolt = Bolt { body, shape, action };

        let uid = entity_heap::new_entity(space, bolt);

        let target = uid;
        let event = BoltLandEvent { target };

        time.enqueue(
            event,
            self.duration,
        );
    }
}

pub struct ClusterCast {
    pub actions: Box<[(units::Displacement, rc::Rc<Cast>)]>
}

impl Cast for ClusterCast {
    fn cast(
        self: &Self,
        space: &mut entity_heap::EntityHeap,
        time: &mut event_queue::EventQueue,
        ref_frame: physics::Body,
        target: units::Position,
    ) {
        for &(loc, ref action) in self.actions.iter() {
            action.cast(
                space,
                time,
                ref_frame.clone(),
                target + loc,
            );
        }
    }
}



