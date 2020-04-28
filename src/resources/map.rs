
use amethyst::{
    ecs::Entity,
    prelude::*,
};

pub struct LocalMap {
    pub current: Entity
}

pub fn create_map_resource(world: &mut World, e: Entity) {
    world.insert(LocalMap{
        current: e,
    });
}

