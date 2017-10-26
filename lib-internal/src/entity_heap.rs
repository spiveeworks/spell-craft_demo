use std::collections;

use forms::effects;

pub trait AsEntity where Self: Sized {
    fn as_entity(Self) -> Entity;
    fn downcast(Entity) -> Result<Self, Entity>;
    fn downcast_ref(&Entity) -> Option<&Self>;
    fn downcast_mut(&mut Entity) -> Option<&mut Self>;
}

macro_rules! as_entity {
    ($var: ident => $typ: ty) => {
        impl AsEntity for $typ {
            fn as_entity(matter: Self) -> Entity {
                Entity::$var(matter)
            }
            fn downcast(ent: Entity) -> Result<Self, Entity> {
                if let Entity::$var(matter) = ent {
                    Ok(matter)
                } else {
                    Err(ent)
                }
            }
            fn downcast_ref(ent: &Entity) -> Option<&Self> {
                if let Entity::$var(ref matter) = *ent {
                    Some(matter)
                } else {
                    None
                }
            }
            fn downcast_mut(ent: &mut Entity) -> Option<&mut Self> {
                if let Entity::$var(ref mut matter) = *ent {
                    Some(matter)
                } else {
                    None
                }
            }
        }
    }
}

macro_rules! entity_definition {
    {$($var: ident($typ: ty)),+,} => {
        pub enum Entity {
            $($var($typ)),+
        }

        $( as_entity!($var => $typ); )+
    }
}

// TODO maybe use a C-enum and downcasting instead?
// that could be better in a number of ways
//  - less coupling between this and the entities' definitions
//  - pointers will definitely point to the right type
//  - polymorphic idiom could emerge
//  - compatible with dynamic entity definitions/modding API (future projects)
entity_definition! {
    Bolt(effects::Bolt),
    Smoke(effects::Smoke),
}


impl Entity {
    pub fn expect<T>(self, err: &str) -> T
        where T: AsEntity
    {
        if let Ok(val) = AsEntity::downcast(self) {
            val
        } else {
            panic!("Downcasted bad entity UID: {}", err);
        }
    }
}

// TODO rand
pub fn new_entity<T>(space: &mut EntityHeap, matter: T) -> UID
    where T: AsEntity
{
    let mut uid = space.len() as UID;
    while space.contains_key(&uid) {
        uid += 1;
    }
    let ent = AsEntity::as_entity(matter);
    space.insert(uid, ent);
    uid
}


pub type UID = u64;

// heap in the memory sense not the queue sense
pub type EntityHeap = collections::HashMap<UID, Entity>;


