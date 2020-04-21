use amethyst::{
    ecs::{Read, ReadStorage, System, Write},
    input::{InputHandler, StringBindings},
    tiles::TileMap,
};

use crate::rl::{ExampleTile, Game, UserAction};

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
        Write<'s, Game>,
    );

    fn run(&mut self, (tilemaps, input, mut game): Self::SystemData) {
        if input.action_is_down("move_left").unwrap() {
            println!("move_left");

            game.user_action = Some(UserAction::Turn);
        }
    }
}
