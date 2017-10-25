use std::collections::VecDeque;

use links;
use entities::effects;

use prelude::*;


// TODO better names for things? space is a subset of container?
pub trait Container<Obj> {
    fn add_entity(&mut self, Owned<Obj>);
    fn remove_entity(&mut self, links::PtrCompare<Obj>) -> Option<Owned<Obj>>;

    fn add_value(&mut self, toad: Obj) -> Link<Obj> {
        let toad = Owned::new(toad);
        let result = toad.share();
        self.add_entity(toad);
        result
    }
}


pub trait AsEntity where Self: Sized {
    fn as_entity(owned: Owned<Self>) -> Entity;
    fn downcast(Entity) -> Result<Owned<Self>, Entity>;
    fn downcast_ref(&Entity) -> Option<&Owned<Self>>;
}

macro_rules! as_entity {
    ($var: ident => $typ: ty) => {
        impl AsEntity for $typ {
            fn as_entity(owned: Owned<Self>) -> Entity {
                Entity::$var(owned)
            }
            fn downcast(ent: Entity) -> Result<Owned<Self>, Entity> {
                if let Entity::$var(owned) = ent {
                    Ok(owned)
                } else {
                    Err(ent)
                }
            }
            fn downcast_ref(ent: &Entity) -> Option<&Owned<Self>> {
                if let Entity::$var(ref owned_ref) = *ent {
                    Some(owned_ref)
                } else {
                    None
                }
            }
        }
    }
}

macro_rules! entity_definition {
    {$($var: ident => $typ: ty),+,} => {
        pub enum Entity {
            $($var(Owned<$typ>)),+
        }

        $( as_entity!($var => $typ); )+
    }
}

entity_definition! {
    Bolt => effects::Bolt,
    Smoke => effects::Smoke,
}



pub struct Space {
    // TODO make a proper interface for this
    pub ents: VecDeque<Entity>,
}

impl<Obj> Container<Obj> for Space
    where Obj: AsEntity
{
    fn add_entity(&mut self, toad: Owned<Obj>) {
        let ent = AsEntity::as_entity(toad);
        self.ents.push_back(ent);
    }
    fn remove_entity(&mut self, tore: links::PtrCompare<Obj>) -> Option<Owned<Obj>> {
        let mut result = None;

        for (i, ent) in self.ents.iter().enumerate() {
            if let Some(item) = AsEntity::downcast_ref(ent) {
                if item.compare() == tore {
                    result = Some(i);
                    break;
                }
            }
        }

        if let Some(i) = result {
            let ent = self.ents
                          .remove(i)
                          .expect("Entity found in invalid index?");
            let item = AsEntity::downcast(ent)
                .ok()  // dont bother with printing the entity
                .expect("Valid entity found but invalid entity removed?");
            Some(item)
        } else {
            None
        }
    }
}

impl Space {
    pub fn new() -> Self {
        Space {
            ents: VecDeque::new(),
        }
    }
}

