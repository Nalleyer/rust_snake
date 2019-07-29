#[macro_use]
extern crate specs_derive;

mod assets;
mod components;
mod graph_creator;
mod resources;
mod states;
mod systems;

use states::MyState;

use amethyst::{
    assets::Processor,
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{sprite::SpriteSheet, types::DefaultBackend, RenderingSystem},
    utils::application_root_dir,
    window::WindowBundle,
};

use crate::systems::{InputSystem, SnakeSystem, TickSystem, TileSystem};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources_dir = app_root.join("resources");
    let display_config_path = resources_dir.join("display_config.ron");

    let bindings_config_path = resources_dir.join("key_config.ron");
    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(bindings_config_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(WindowBundle::from_config_path(display_config_path))?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(
            Processor::<SpriteSheet>::new(),
            "sprite_sheet_processor",
            &[],
        )
        // systems
        .with(TileSystem, "tile_system", &[])
        .with(TickSystem::default(), "tick_system", &[])
        .with(
            InputSystem::default(),
            "my_input_system",
            &["input_system", "tick_system"],
        )
        .with(SnakeSystem::default(), "snake_system", &[])
        .with_thread_local(RenderingSystem::<DefaultBackend, _>::new(
            graph_creator::RenderingGraph::default(),
        ));

    let mut game = Application::new(resources_dir, MyState, game_data)?;
    game.run();

    Ok(())
}
