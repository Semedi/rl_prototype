use amethyst::{
    ecs::prelude::{DenseVecStorage, Component},
};

const DEFAULT_MOVEMENT_INPUT_TIME: f32 = 0.5;

#[derive(Default)]
pub struct Player {
    pub input_movement_timer: f32,
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

impl Player{

    pub fn new() -> Player {
        Player {
            input_movement_timer: DEFAULT_MOVEMENT_INPUT_TIME,
        }
    }

    pub fn can_move(&mut self, dt: f32) -> bool {
        if self.input_movement_timer > 0.0 {

            self.input_movement_timer -= dt;

            return false;
        }else{

            return true;
        }
    }

    pub fn movement(&mut self) {
        self.input_movement_timer = DEFAULT_MOVEMENT_INPUT_TIME;
    }


}
