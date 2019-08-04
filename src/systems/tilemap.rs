use amethyst::{core::Transform, ecs::prelude::*, renderer::sprite::SpriteRender};
use std::f64::consts::PI;

use crate::{
    assets::{T_CORNER, T_EMPTY, T_FOOD},
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
        for tilemap in (&mut tilemaps).join() {
            for idx in 0..tilemap.len() {
                let mut updated = false;
                let snake = board.get_snake();
                for cell in snake.0.iter() {
                    if cell.pos == idx {
                        let entity = tilemap.entities[idx];
                        if let Some(sprite_render) = sprite_renders.get_mut(entity) {
                            sprite_render.sprite_number = cell.ttype as usize;
                        }

                        if let Some(trans) = transforms.get_mut(entity) {
                            let ref_direction = if cell.ttype == T_CORNER {
                                &cell.corner_directon
                            } else {
                                &cell.direction
                            };
                            trans.set_rotation_z_axis(match ref_direction {
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

                let food_pos = board.get_food_pos();
                if idx == food_pos {
                    let food_entity = tilemap.entities[idx];
                    if let Some(sprite_render) = sprite_renders.get_mut(food_entity) {
                        sprite_render.sprite_number = T_FOOD as usize;
                        updated = true;
                    }
                }

                if updated {
                    continue;
                }

                if let Some(sprite_render) = sprite_renders.get_mut(tilemap.entities[idx]) {
                    sprite_render.sprite_number = T_EMPTY as usize;
                }
            }
        }
    }
}
