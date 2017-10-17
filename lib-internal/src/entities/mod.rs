use std::collections::VecDeque;

use prelude::*;
use links;

mod grenade;

pub use self::grenade::*;

// TODO better names for things? space is a subset of container?
pub trait Container<Obj> {
    fn add_entity(&mut self, Owned<Obj>);
    fn remove_entity(&mut self, links::PtrCompare<Grenade>) -> Option<Owned<Obj>>;

    fn add_value(&mut self, toad: Obj) -> Link<Obj> {
        let toad = Owned::new(toad);
        let result = toad.share();
        self.add_entity(toad);
        result
    }
}

pub struct Space {
    // TODO make a proper interface for this
    pub nades: VecDeque<Owned<Grenade>>,
}

impl Container<Grenade> for Space {
    fn add_entity(&mut self, toad: Owned<Grenade>) {
        self.nades.push_back(toad);
    }
    fn remove_entity(&mut self, tore: links::PtrCompare<Grenade>) -> Option<Owned<Grenade>> {
        let mut result = None;

        for (i, item) in self.nades.iter().enumerate() {
            if item.compare() == tore {
                result = Some(i);
                break;
            }
        }

        result.and_then(|i| self.nades.remove(i))
    }
}

impl Space {
    pub fn new() -> Self {
        Space {
            nades: VecDeque::new(),
        }
    }
}

