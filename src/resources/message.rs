use crate::resources::MovingDirection;

use amethyst::shrev::EventChannel;

#[derive(Debug)]
pub enum Msg {
    // input
    Move(MovingDirection),

    //
    Hit,
}

pub type MessageChannel = EventChannel<Msg>;
