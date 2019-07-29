// use std::collections::HashSet;
use amethyst::{core::timing::Time, ecs::prelude::*};

use crate::resources::{Context, MessageChannel, Msg};

#[derive(Debug)]
pub struct TickSystem {
    last_tick: f64,
}

impl Default for TickSystem {
    fn default() -> Self {
        TickSystem { last_tick: 0.0 }
    }
}

impl<'s> System<'s> for TickSystem {
    type SystemData = (
        Read<'s, Time>,
        ReadExpect<'s, Context>,
        Write<'s, MessageChannel>,
    );

    fn run(&mut self, (time, ctx, mut messages): Self::SystemData) {
        let now = time.absolute_real_time_seconds();
        let tick_duration = ctx.tick_duration;
        let new_tick = self.last_tick + tick_duration;
        if now > new_tick {
            self.last_tick = new_tick;
            messages.single_write(Msg::Tick(new_tick));
        }
    }
}
