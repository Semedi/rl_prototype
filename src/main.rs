use amethyst::{
    prelude::*,
    core::TransformBundle,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle, 
    },
    tiles::{RenderTiles2D, MortonEncoder},
    utils::application_root_dir,
};

mod rl;
use crate::rl::{Rl, ExampleTile};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    let assets_dir = app_root.join("assets");

    // We'll put the rest of the code here.
    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderTiles2D::<ExampleTile, MortonEncoder>::default())
        )?
        .with_bundle(TransformBundle::new())?;

    let mut game = Application::new(assets_dir, Rl, game_data)?;
    game.run();

    Ok(())
}
