
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
          let ts    = world.read_storage::<Transform>();
          let tmaps = world.read_storage::<TileMap<ExampleTile>>();

          if let Some((transform, map)) = (&ts, &tmaps).join().get(self.current, &world.entities()){

            return Some(
                map.to_world(
                    &Point3::new(x, y, 1), 
                    Some(&transform))
            );
          }

          None
    }
}




pub fn create_map_resource(world: &mut World, e: Entity) {
    world.insert(LocalMap{
        current: e,
    });
}

