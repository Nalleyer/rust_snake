use amethyst::ecs::prelude::*;

use crate::resources::{Board, MessageChannel, MovingDirection, Msg};

pub struct SnakeSystem {
    message_reader: Option<ReaderId<Msg>>,
}

impl Default for SnakeSystem {
    fn default() -> Self {
        SnakeSystem {
            message_reader: None,
        }
    }
}

impl<'s> System<'s> for SnakeSystem {
    type SystemData = (Read<'s, MessageChannel>, WriteExpect<'s, Board>);

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.message_reader = Some(res.fetch_mut::<MessageChannel>().register_reader());
    }

    fn run(&mut self, (messages, mut board): Self::SystemData) {
        for message in messages.read(self.message_reader.as_mut().unwrap()) {
            match message {
                Msg::Move(direction) => board.move_snake(direction),
                _ => (),
            }
        }
    }
}
