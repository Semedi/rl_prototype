use amethyst::{
    ecs::{Read, ReadStorage, System},
    input::{InputHandler, StringBindings},
    tiles::TileMap,
};

use crate::rl::ExampleTile;

pub struct PlayerMovement;
impl Default for PlayerMovement {
    fn default() -> Self {
        Self {}
    }
}

impl<'s> System<'s> for PlayerMovement {
    type SystemData = (
        ReadStorage<'s, TileMap<ExampleTile>>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (tilemaps, input): Self::SystemData) {
        if input.action_is_down("move_left").unwrap() {
            println!("move_left");
        }
    }
}
