use amethyst::{
    core::{
        Transform, 
        timing::Time
    },
    ecs::{Join, Read, ReadStorage, System, Write, WriteStorage},
    input::{InputHandler, StringBindings},
    tiles::TileMap,
};

use crate::rl::{ExampleTile, Game, UserAction};
use crate::components::Player;

pub struct PlayerMovement;
impl Default for PlayerMovement {
    fn default() -> Self {
        Self {}
    }
}

impl<'s> System<'s> for PlayerMovement {
    type SystemData = (
        ReadStorage<'s, TileMap<ExampleTile>>,
        Read<'s, Time>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
        Write<'s, Game>,
    );

    fn run(&mut self, (tilemaps, t, mut transforms, mut players, input, mut game): Self::SystemData) {

        let moves: [&'static str; 4] = ["move_left", "move_right", "move_down", "move_up"];

        for (transform, player) in (&mut transforms, &mut players).join() {
            let can_move = player.can_move(t.delta_seconds());

            for &m in moves.iter() {

                let movement = input.action_is_down(m).unwrap() && can_move;

                if movement {
                    game.user_action = Some(UserAction::Turn);
                    player.movement();
                    println!("{}", m);
                }
            }
        }
    }
}
