mod pong;

mod systems;

use crate::pong::Pong;

use amethyst::{
    prelude::*,
    core::transform::TransformBundle,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    input::{InputBundle, StringBindings},
    utils::application_root_dir,
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(app_root.join("config").join("display.ron"))?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(
            TransformBundle::new(),
        )?
        .with_bundle(
            InputBundle::<StringBindings>::new()
            .with_bindings_from_file(
                app_root.join("config").join("bindings.ron"),
            )?,
        )?
        .with(
            systems::PaddleSystem,
            "paddle_system",
            &["input_system"]
        );

    let mut game = Application::new(
        app_root.join("assets"),
        Pong,
        game_data
    )?;

    game.run();

    Ok(())
}
