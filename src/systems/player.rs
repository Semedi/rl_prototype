use amethyst::{
    prelude::*,
    core::{
        Transform, 
        timing::Time
    },
    ecs::{Component, Join, Read, System, Write, WriteStorage, ReadStorage, ReadExpect, Entities},
    input::{InputHandler, StringBindings},
    tiles::{TileMap, MortonEncoder, Map},
};

use crate::rl::{Game, UserAction,};
use crate::components::Player;
use crate::resources::LocalMap;
use crate::entities::ExampleTile;

pub struct PlayerInput;
impl Default for PlayerInput {
    fn default() -> Self {
        Self {}
    }
}

impl<'s> System<'s> for PlayerInput {
    type SystemData = (
        Read<'s, Time>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
        ReadStorage<'s, TileMap::<ExampleTile, MortonEncoder>>,
        ReadExpect<'s, LocalMap>,
        Entities<'s>,
        Write<'s, Game>,
    );


    fn run(&mut self, (t, mut transforms, mut players, input, tilemaps, local_map, entities, mut game): Self::SystemData) {
        let moves: [&'static str; 4] = ["move_left", "move_right", "move_down", "move_up"];


        let (tmap, tilemap) = {
            if let Some((t, tilemap)) = (&mut transforms, &tilemaps).join().get(local_map.current, &entities){
                (Some(t), Some(tilemap))
            }else {
                (None, None)
            }
        };

        for (entity, player) in (&entities, &mut players).join() {
            let can_move = player.can_move(t.delta_seconds());

            for &m in moves.iter() {
                let movement = input.action_is_down(m).unwrap() && can_move;

                if movement {
                    let (tmap, tilemap) = {
                        if let Some((t, tilemap)) = (&mut transforms, &tilemaps).join().get(local_map.current, &entities){
                            (Some(t), Some(tilemap))
                        }else {
                            (None, None)
                        }
                    };
                    game.user_action = Some(UserAction::Turn);
                    player.movement();

                    println!("{}", tmap.unwrap().translation());

                    println!("{}", m);
                }
            }
        }
    }
}

        //let map = {
        //    if let Some((map_tiles, map_transform)) = (&mut transforms, &tilemap).join().get(region.current, &entities){

        //        Some((map_tiles, map_transform))
        //    } else {
        //        None
        //    }
        //};
        //if let Some((map_tiles, map_transform)) = map {
        //    println!("is working!");
        //}



        //if let Some(map) = tilemap.get(region.current) {
        //        println!("{}", map.origin());
        //    }
        //}

