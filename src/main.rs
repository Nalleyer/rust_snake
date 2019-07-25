#[macro_use]
extern crate specs_derive;

mod assets;
mod graph_creator;
mod states;
mod tilemap;
mod resource;

use states::MyState;

use amethyst::{
    assets::Processor,
    core::transform::TransformBundle,
    prelude::*,
    renderer::{sprite::SpriteSheet, types::DefaultBackend, RenderingSystem},
    utils::application_root_dir,
    window::WindowBundle,
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources_dir = app_root.join("resources");
    let display_config_path = resources_dir.join("display_config.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(WindowBundle::from_config_path(display_config_path))?
        .with_bundle(TransformBundle::new())?
        .with(
            Processor::<SpriteSheet>::new(),
            "sprite_sheet_processor",
            &[],
        )
        .with_thread_local(RenderingSystem::<DefaultBackend, _>::new(
            graph_creator::RenderingGraph::default(),
        ));

    let mut game = Application::new(resources_dir, MyState, game_data)?;
    game.run();

    Ok(())
}
