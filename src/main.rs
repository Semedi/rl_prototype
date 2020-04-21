use amethyst::{
    prelude::*,
    core::TransformBundle,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle, RenderDebugLines
    },
    input::{InputBundle, StringBindings},
    tiles::{RenderTiles2D, MortonEncoder},
    utils::application_root_dir,
};

mod systems;
mod rl;
use crate::rl::{Rl, ExampleTile};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    let assets_dir = app_root.join("assets");

    // We'll put the rest of the code here.
    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new()
                .with_bindings_from_file("config/input.ron")?,
        )?
        .with(
            systems::MapMovementSystem::default(),
            "MapMovementSystem",
            &["input_system"],
        )
        .with(
            systems::CameraSwitchSystem::default(),
            "camera_switch",
            &["input_system"],
        )
        .with(
            systems::PlayerMovement::default(),
            "PlayerMovement",
            &["input_system"],
        )
        .with(
            systems::CameraMovementSystem::default(),
            "movement",
            &["camera_switch"],
        )
        .with(
            systems::DrawSelectionSystem::default(),
            "DrawSelectionSystem",
            &["camera_switch"],
        )
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderDebugLines::default())
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderTiles2D::<ExampleTile, MortonEncoder>::default())
        )?;

    let mut game = Application::new(assets_dir, Rl, game_data)?;
    game.run();

    Ok(())
}
