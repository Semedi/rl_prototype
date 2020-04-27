use amethyst::{
    ecs::prelude::{DenseVecStorage, Component},
};

#[derive(Default)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Component for Position {
    type Storage = DenseVecStorage<Self>;
}
