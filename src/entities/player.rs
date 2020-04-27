use amethyst::{
    assets::Handle,
    ecs::Entity,
    ecs::Join,
    prelude::*,
    core::{
        transform::Transform,
        math::{Vector3, Point3},
    },
    renderer::{
        transparent::Transparent,
        SpriteRender, SpriteSheet,
    },
    tiles::{TileMap, Map, MortonEncoder},
};

use crate::components;
use crate::resources::LocalMap;
use crate::{
    entities::ExampleTile,
};

pub fn init_player(world: &mut World, sprite_sheet: &Handle<SpriteSheet>) -> Entity {
    let mut ptransform = Transform::default();

    {
        let ts    = world.read_storage::<Transform>();
        let tmaps = world.read_storage::<TileMap::<ExampleTile, MortonEncoder>>();

        let map   = world.read_resource::<LocalMap>();

        if let Some((transform, map)) = (&ts, &tmaps).join().get(map.current, &world.entities()){

            let v = map.to_world( &Point3::new(0, 0, 1), Some(&transform)); 

            println!("{}",v);

            ptransform.set_translation_xyz(v.x, v.y, v.z);
        }
        else{
            println!("mierda");
        }
    }

    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 32,
    };

    let player = components::Player::new();
    world
        .create_entity()
        .with(ptransform)
        .with(player)
        .with(sprite)
        .with(Transparent)
        .named("player")
        .build()
}
