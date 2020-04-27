use amethyst::{
    prelude::*,
    ecs::Entity,
    tiles::{MortonEncoder, Tile, TileMap},
    assets::{Handle},
    core::{
        math::{Point3, Vector3},
        Transform,
    },
    renderer::{
        SpriteSheet,
    },
};


#[derive(Default, Clone)]
pub struct ExampleTile;

impl Tile for ExampleTile {
    fn sprite(&self, _: Point3<u32>, _: &World) -> Option<usize> {
        Some(1)
    }
}
pub struct LocalMap {
    pub current: Entity
}

pub fn init_map(world: &mut World, sprite_sheet: Handle<SpriteSheet>) -> Entity {
    let map = TileMap::<ExampleTile, MortonEncoder>::new(
        Vector3::new(48, 48, 1),
        Vector3::new(20, 20, 1),
        Some(sprite_sheet),
    );

     let entity = world
        .create_entity()
        .with(map)
        .with(Transform::default())
        .build();

    return entity;
}
