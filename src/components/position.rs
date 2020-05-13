use amethyst::{
    ecs::prelude::{DenseVecStorage, Component},
};

#[derive(Default)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

impl Component for Position {
    type Storage = DenseVecStorage<Self>;
}
