use amethyst::{
    assets::{Handle},
    ecs::prelude::{NullStorage, Component},
    ecs::Entity,
    prelude::*,
    core::{
        transform::Transform,
    },
    renderer::{
        transparent::Transparent,
        SpriteRender, SpriteSheet,
    }
};

#[derive(Default)]
pub struct Player;

impl Component for Player {
    type Storage = NullStorage<Self>;
}

pub fn init_player(world: &mut World, sprite_sheet: &Handle<SpriteSheet>) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 0.1);
    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 32,
    };
    world
        .create_entity()
        .with(transform)
        .with(Player)
        .with(sprite)
        .with(Transparent)
        .named("player")
        .build()
}
