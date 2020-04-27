
use amethyst::{
    ecs::{Entity, Join},
    prelude::*,
    core::{
        transform::Transform,
        math::{Vector3, Point3},
    },
    tiles::{TileMap, Map},
};

use crate::{
    entities::ExampleTile,
};

pub struct LocalMap {
    pub current: Entity
}

impl LocalMap {
    pub fn get_pos(&mut self, world: &World, x: u32, y: u32) -> Option<Vector3<f32>> {
          None
    }
}




pub fn create_map_resource(world: &mut World, e: Entity) {
    world.insert(LocalMap{
        current: e,
    });
}

