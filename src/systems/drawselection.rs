use amethyst::{
    core::{
        math::{Point3, Vector2},
        Transform,
    },
    ecs::{
        Read, WriteStorage, ReadExpect, ReadStorage, System,
        Entities, Join
    },
    input::{InputHandler, StringBindings},
    renderer::{
        camera::{Camera, ActiveCamera},
        palette::Srgba,
        debug_drawing::DebugLinesComponent,
    },
    window::ScreenDimensions,

};


#[derive(Default)]
pub struct DrawSelectionSystem {
    start_coordinate: Option<Point3<f32>>,
}

impl<'s> System<'s> for DrawSelectionSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, ActiveCamera>,
        ReadExpect<'s, ScreenDimensions>,
        ReadStorage<'s, Camera>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, DebugLinesComponent>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(
        &mut self,
        (entities, active_camera, dimensions, cameras, transforms, mut debug_lines, input): Self::SystemData,
    ) {
        if let Some(lines) = (&mut debug_lines).join().next() {
            lines.clear();

            if let Some(mouse_position) = input.mouse_position() {
                let mut camera_join = (&cameras, &transforms).join();
                if let Some((camera, camera_transform)) = active_camera
                    .entity
                    .and_then(|a| camera_join.get(a, &entities))
                    .or_else(|| camera_join.next())
                {
                    let action_down = input
                        .action_is_down("select")
                        .expect("selection action missing");
                    if action_down && self.start_coordinate.is_none() {
                        // Starting a new selection
                        self.start_coordinate = Some(Point3::new(
                            mouse_position.0,
                            mouse_position.1,
                            camera_transform.translation().z,
                        ));
                    } else if action_down && self.start_coordinate.is_some() {
                        // Active drag
                        let screen_dimensions =
                            Vector2::new(dimensions.width(), dimensions.height());
                        let end_coordinate = Point3::new(
                            mouse_position.0,
                            mouse_position.1,
                            camera_transform.translation().z,
                        );

                        let mut start_world = camera.projection().screen_to_world_point(
                            self.start_coordinate.expect("Wut?"),
                            screen_dimensions,
                            camera_transform,
                        );
                        let mut end_world = camera.projection().screen_to_world_point(
                            end_coordinate,
                            screen_dimensions,
                            camera_transform,
                        );
                        start_world.z = 0.9;
                        end_world.z = 0.9;

                        lines.add_box(start_world, end_world, Srgba::new(0.5, 0.05, 0.65, 1.0));
                    } else if !action_down && self.start_coordinate.is_some() {
                        // End drag, remove
                        self.start_coordinate = None;
                    }
                }
            }
        }
    }
}

