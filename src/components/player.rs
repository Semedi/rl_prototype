use amethyst::{
    ecs::prelude::{NullStorage, Component},
};

#[derive(Default)]
pub struct Player;

impl Component for Player {
    type Storage = NullStorage<Self>;
}
