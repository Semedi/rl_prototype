use amethyst::{
    assets::Handle,
    ecs::Entity,
    prelude::*,
    core::{
        transform::Transform,
    },
    renderer::{
        transparent::Transparent,
        SpriteRender, SpriteSheet,
    },
};

use crate::components;
use crate::tiles;

pub fn init_player(world: &mut World, sprite_sheet: &Handle<SpriteSheet>) -> Entity {
    let mut ptransform = Transform::default();

    let pos = tiles::get_pos(world, 10, 22).unwrap();
    ptransform.set_translation_xyz(pos.x, pos.y, pos.z);

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
