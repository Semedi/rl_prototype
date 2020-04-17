use amethyst::{
    ecs::{
        Entities, Read, ReadStorage, ReadExpect, System, WriteStorage, Join,
        LazyUpdate,
    },
    core::{
        math::{Vector3},
        Transform, Parent,
    },
    input::{InputHandler, StringBindings},
    renderer::{
        camera::{ActiveCamera, Camera, Projection,},
    },
    window::ScreenDimensions,
    prelude::WorldExt,
};


pub struct CameraSwitchSystem {
    pressed: bool,
}
impl Default for CameraSwitchSystem {
    fn default() -> Self {
        Self { pressed: false }
    }
}
impl<'s> System<'s> for CameraSwitchSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, LazyUpdate>,
        Read<'s, ActiveCamera>,
        ReadExpect<'s, ScreenDimensions>,
        ReadStorage<'s, Camera>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Parent>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(
        &mut self,
        (entities, lazy, active_camera, dimensions, cameras, transforms, parents, input): Self::SystemData,
    ) {
        if input.action_is_down("camera_switch").unwrap() {
            self.pressed = true;
        }
        if self.pressed && !input.action_is_down("camera_switch").unwrap() {
            self.pressed = false;

            // Lazily delete the old camera
            let mut camera_join = (&entities, &cameras, &transforms, &parents).join();
            let (old_camera_entity, old_camera, _, old_parent) = active_camera
                .entity
                .and_then(|a| camera_join.get(a, &entities))
                .or_else(|| camera_join.next())
                .unwrap();
            let old_camera_entity = old_camera_entity;

            let new_parent = old_parent.entity;
            let (new_camera, new_position) = match old_camera.projection() {
                Projection::Orthographic(_) => (
                    Camera::standard_3d(dimensions.width(), dimensions.height()),
                    Vector3::new(0.0, 0.0, 500.1),
                ),
                Projection::Perspective(_) => (
                    Camera::standard_2d(dimensions.width(), dimensions.height()),
                    Vector3::new(0.0, 0.0, 1.1),
                ),
                Projection::CustomMatrix(_) => unimplemented!(),
            };

            lazy.exec_mut(move |w| {
                let new_camera =
                    init_camera(w, new_parent, Transform::from(new_position), new_camera);

                w.fetch_mut::<ActiveCamera>().entity = Some(new_camera);

                w.delete_entity(old_camera_entity).unwrap();
            });
        }
    }
}

use crate::rl::init_camera;

#[derive(Default)]
pub struct CameraMovementSystem;
impl<'s> System<'s> for CameraMovementSystem {
    type SystemData = (
        Read<'s, ActiveCamera>,
        Entities<'s>,
        ReadStorage<'s, Camera>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (active_camera, entities, cameras, mut transforms, input): Self::SystemData) {
        let x_move = input.axis_value("camera_x").unwrap();
        let y_move = input.axis_value("camera_y").unwrap();
        let z_move = input.axis_value("camera_z").unwrap();
        let z_move_scale = input.axis_value("camera_scale").unwrap();

        if x_move != 0.0 || y_move != 0.0 || z_move != 0.0 || z_move_scale != 0.0 {
            let mut camera_join = (&cameras, &mut transforms).join();
            if let Some((_, camera_transform)) = active_camera
                .entity
                .and_then(|a| camera_join.get(a, &entities))
                .or_else(|| camera_join.next())
            {
                camera_transform.prepend_translation_x(x_move * 5.0);
                camera_transform.prepend_translation_y(y_move * 5.0);
                camera_transform.prepend_translation_z(z_move);

                let z_scale = 0.01 * z_move_scale;
                let scale = camera_transform.scale();
                let scale = Vector3::new(scale.x + z_scale, scale.y + z_scale, scale.z + z_scale);
                camera_transform.set_scale(scale);
            }
        }
    }
}

