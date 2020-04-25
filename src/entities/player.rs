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

pub fn init_player(world: &mut World, sprite_sheet: &Handle<SpriteSheet>) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 0.1);
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
