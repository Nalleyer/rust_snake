use amethyst::{prelude::*, window::ScreenDimensions};

use crate::resources::Context;

pub fn get_screen_size(world: &World) -> (f32, f32) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let dimensions = world.read_resource::<ScreenDimensions>();

    let width = dimensions.width();
    let height = dimensions.height();
    (width, height)
}

pub fn get_scale(world: &World) -> f32 {
    let ctx = world.read_resource::<Context>();
    ctx.scale
}

pub fn get_tick_duration(world: &World) -> f64 {
    let ctx = world.read_resource::<Context>();
    ctx.tick_duration
}
