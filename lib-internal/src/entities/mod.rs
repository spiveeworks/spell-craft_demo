use std::collections::VecDeque;

use links::{Owned, Link};
use links;

mod grenade;

pub use self::grenade::*;

pub trait Container<Obj> {
    fn add_entity(&mut self, Owned<Obj>);
    fn remove_entity(&mut self, &links::Ref<Obj>) -> Option<Owned<Obj>>;

    fn add_value(&mut self, toad: Obj) -> Link<Obj> {
        let toad = Owned::new(toad);
        let result = Owned::share(&toad);
        self.add_entity(toad);
        result
    }
}

pub struct Space {
    nades: VecDeque<Owned<Grenade>>,
}

impl Container<Grenade> for Space {
    fn add_entity(&mut self, toad: Owned<Grenade>) {
        self.nades.push_back(toad);
    }
    fn remove_entity(&mut self, tore: &links::Ref<Grenade>) -> Option<Owned<Grenade>> {
        let mut result = None;

        for (i, item) in self.nades.iter().enumerate() {
            if Owned::ptr_eq(item, tore) {
                result = Some(i);
                break;
            }
        }

        result.and_then(|i| self.nades.remove(i))
    }
}

