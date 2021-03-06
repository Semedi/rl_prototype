use amethyst::{
    core::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderDebugLines, RenderingBundle,
    },
    tiles::{MortonEncoder, RenderTiles2D},
    utils::application_root_dir,
};

pub mod rl;
pub mod tiles;
pub mod systems;
pub mod entities;
pub mod components;
pub mod resources;

use crate::rl::{Game, InputState};
use crate::entities::ExampleTile;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    let assets_dir = app_root.join("assets");

    // We'll put the rest of the code here.
    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file("config/input.ron")?,
        )?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderDebugLines::default())
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderTiles2D::<ExampleTile, MortonEncoder>::default()),
        )?
        .with(
            systems::MapMovementSystem::default(),
            "MapMovementSystem",
            &[],
        )
        .with(
            systems::CameraSwitchSystem::default(),
            "camera_switch",
            &[],
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
        );

    let mut game = Application::build(assets_dir, InputState::default())?
        .with_resource(Game::default())
        .build(game_data)?;

    //let mut game = Application::new(assets_dir, Rl, game_data)?;
    game.run();

    Ok(())
}
