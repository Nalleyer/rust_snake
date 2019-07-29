use amethyst::{
    ecs::prelude::*,
    input::{InputHandler, StringBindings},
};

use crate::resources::{Board, MessageChannel, MovingDirection, Msg};
use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
pub struct InputSystem {
    message_reader: Option<ReaderId<Msg>>,
    pressing: HashSet<String>,
    key_queue: VecDeque<MovingDirection>,
}

impl Default for InputSystem {
    fn default() -> Self {
        InputSystem {
            message_reader: None,
            pressing: HashSet::new(),
            key_queue: VecDeque::new(),
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

    fn run(&mut self, (inputs, mut messages, board): Self::SystemData) {
        for axis in inputs.bindings.axes() {
            let value = inputs.axis_value(axis).unwrap();
            let was_down = self.pressing.contains(axis.as_str());
            let maybe_direction = MovingDirection::from_axis(axis, value);
            let is_down = maybe_direction.is_some();

            if was_down && !is_down {
                self.pressing.remove(axis.as_str());
            }

            if !was_down && is_down {
                // new press
                self.pressing.insert(axis.clone());
                self.key_queue.push_back(maybe_direction.unwrap());
            }

            let mut is_tick = false;
            for message in messages.read(self.message_reader.as_mut().unwrap()) {
                if let Msg::Tick(_) = message {
                    is_tick = true;
                }
            }
            if is_tick {
                if let Some(direction) = self.key_queue.pop_front() {
                    if board.input_valid(&direction) {
                        messages.single_write(Msg::Move(direction.clone()));
                    }
                } else {
                    messages.single_write(Msg::Move(board.current_direction().clone()));
                }
            }
        }
    }
}
