use amethyst::{
    assets::Handle,
    ecs::Entity,
    prelude::*,
    core::transform::Transform,
    renderer::{
        transparent::Transparent,
        SpriteRender, SpriteSheet,
    }
};

use crate::components;
use crate::resources::LocalMap;

pub fn init_player(world: &mut World, sprite_sheet: &Handle<SpriteSheet>) -> Entity {
    let mut transform = Transform::default();

    let _pos = {
        let map = world.read_resource::<LocalMap>();

        map.get_pos(world, 1, 1)
    };


    transform.set_translation_xyz(0.0, 155.0, 1.0);


    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 32,
    };

    let player = components::Player::new();
    world
        .create_entity()
        .with(transform)
        .with(player)
        .with(sprite)
        .with(Transparent)
        .named("player")
        .build()
}
