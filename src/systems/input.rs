// use std::collections::HashSet;
use amethyst::{
    core::{timing::Time, Transform},
    ecs::prelude::*,
    input::{InputHandler, StringBindings},
};

use crate::resources::{Context, MessageChannel, MovingDirection, Msg};

#[derive(Debug)]
pub struct InputSystem {
    last_tick: f64,
    pressed: Option<MovingDirection>,
}

impl Default for InputSystem {
    fn default() -> Self {
        InputSystem {
            last_tick: 0.0,
            pressed: None,
        }
    }
}

impl<'s> System<'s> for InputSystem {
    type SystemData = (
        Read<'s, Time>,
        Read<'s, InputHandler<StringBindings>>,
        ReadExpect<'s, Context>,
        Write<'s, MessageChannel>,
    );

    fn run(&mut self, (time, input, ctx, mut messages): Self::SystemData) {
        let mx = input.axis_value("move_x");
        let my = input.axis_value("move_y");
        if self.pressed.is_none() {
            if let Some(x_value) = mx {
                if let Some(input) = MovingDirection::from_axis_x(x_value) {
                    self.pressed.replace(input);
                    println!("p {:?}", self.pressed);
                }
            }
        }

        if self.pressed.is_none() {
            if let Some(y_value) = my {
                if let Some(input) = MovingDirection::from_axis_y(y_value) {
                    self.pressed.replace(input);
                    println!("p {:?}", self.pressed);
                }
            }
        }

        let now = time.absolute_real_time_seconds();
        let tick_duration = ctx.tick_duration;
        let new_tick = self.last_tick + tick_duration;
        if now > new_tick {
            self.last_tick = new_tick;
            if let Some(direction) = &self.pressed {
                messages.single_write(Msg::Move(direction.clone()));
                self.pressed = None;
            }
        }
    }
}
