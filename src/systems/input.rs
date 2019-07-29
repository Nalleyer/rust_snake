// use std::collections::HashSet;
use amethyst::{
    ecs::prelude::*,
    input::{InputHandler, StringBindings},
};

use crate::resources::{Board, MessageChannel, MovingDirection, Msg};

#[derive(Debug)]
pub struct InputSystem {
    message_reader: Option<ReaderId<Msg>>,
    pressed: Option<MovingDirection>,
}

impl Default for InputSystem {
    fn default() -> Self {
        InputSystem {
            message_reader: None,
            pressed: None,
        }
    }
}

impl<'s> System<'s> for InputSystem {
    type SystemData = (
        Read<'s, InputHandler<StringBindings>>,
        Write<'s, MessageChannel>,
        ReadExpect<'s, Board>,
    );

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.message_reader = Some(res.fetch_mut::<MessageChannel>().register_reader());
    }

    fn run(&mut self, (input, mut messages, board): Self::SystemData) {
        let mx = input.axis_value("move_x");
        let my = input.axis_value("move_y");

        if self.pressed.is_none() {
            if let Some(x_value) = mx {
                if let Some(input) = MovingDirection::from_axis_x(x_value) {
                    self.pressed.replace(input);
                }
            }
        }

        if self.pressed.is_none() {
            if let Some(y_value) = my {
                if let Some(input) = MovingDirection::from_axis_y(y_value) {
                    self.pressed.replace(input);
                }
            }
        }

        let mut is_tick = false;
        for message in messages.read(self.message_reader.as_mut().unwrap()) {
            if let Msg::Tick(_) = message {
                is_tick = true;
            }
        }
        if is_tick {
            if let Some(direction) = &self.pressed {
                if board.input_valid(direction) {
                    messages.single_write(Msg::Move(direction.clone()));
                } else {
                    messages.single_write(Msg::Move(board.current_direction().clone()));
                }
                self.pressed = None;
            } else {
                messages.single_write(Msg::Move(board.current_direction().clone()));
            }
        }
    }
}
