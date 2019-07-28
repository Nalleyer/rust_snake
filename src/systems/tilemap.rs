use amethyst::{core::Transform, ecs::prelude::*, renderer::sprite::SpriteRender};
use std::f64::consts::PI;

use crate::{
    assets::{T_EMPTY, T_FOOD},
    components::TileMap,
    resources::{Board, MovingDirection},
};

#[derive(Debug)]
pub struct TileSystem;

impl<'s> System<'s> for TileSystem {
    type SystemData = (
        ReadExpect<'s, Board>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, TileMap>,
    );

    fn run(&mut self, (board, mut sprite_renders, mut transforms, mut tilemaps): Self::SystemData) {
        for (tilemap) in (&mut tilemaps).join() {
            for idx in (0..tilemap.len()) {
                let mut updated = false;
                let snake = board.get_snake();
                for (idx_snake, cell) in snake.0.iter().enumerate() {
                    if idx_snake == idx {
                        let entity = tilemap.entities[idx_snake];
                        if let Some(sprite_render) = sprite_renders.get_mut(entity) {
                            sprite_render.sprite_number = cell.ttype as usize;
                        }

                        if let Some(trans) = transforms.get_mut(entity) {
                            trans.set_rotation_z_axis(match cell.direction {
                                MovingDirection::Up => 0.0,
                                MovingDirection::Down => PI,
                                MovingDirection::Right => -PI * 0.5,
                                MovingDirection::Left => PI * 0.5,
                            });
                        }

                        updated = true;
                        break;
                    }
                }
                if updated {
                    continue;
                }

                let food_entity = tilemap.entities[board.get_food_pos()];
                if let Some(sprite_render) = sprite_renders.get_mut(food_entity) {
                    sprite_render.sprite_number = T_FOOD as usize;
                    updated = true;
                }
                if updated {
                    continue;
                }

                if let Some(sprite_render) = sprite_renders.get_mut(food_entity) {
                    sprite_render.sprite_number = T_EMPTY as usize;
                }
            }
        }
    }
}
