use amethyst::{
    prelude::*,
    ecs::{Join, WriteStorage, Component},
    core::{
        transform::Transform,
        math::{Vector3, Point3},
    },
    tiles::{TileMap, MortonEncoder, Map},
};

use crate::resources::LocalMap;
use crate::entities::ExampleTile;


pub fn get_pos(world: &mut World, x: u32, y: u32) -> Option<Vector3<f32>> {
    let ts    = world.read_storage::<Transform>();
    let tmaps = world.read_storage::<TileMap::<ExampleTile, MortonEncoder>>();
    let map   = world.read_resource::<LocalMap>();

    if let Some((transform, map)) = (&ts, &tmaps).join().get(map.current, &world.entities()){

        return Some(map.to_world( &Point3::new(x, y, 1), Some(&transform))); 
    }
    else{
        return None
    }
}

pub fn get_pos_wo<T: Component>(transforms: &mut WriteStorage<T>) {

    println!("hola");
}
