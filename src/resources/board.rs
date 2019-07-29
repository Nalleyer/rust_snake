extern crate rand;
use crate::assets::{T_BODY, T_HEAD};

use rand::distributions::{Distribution, Uniform};
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone)]
pub enum MovingDirection {
    Left,
    Right,
    Up,
    Down,
}

impl MovingDirection {
    pub fn from_axis(axis: &str, value: f64) -> Option<Self> {
        if value > 0.2 {
            match axis {
                "move_x" => Some(MovingDirection::Right),
                "move_y" => Some(MovingDirection::Up),
                _ => None,
            }
        } else if value < -0.2 {
            match axis {
                "move_x" => Some(MovingDirection::Left),
                "move_y" => Some(MovingDirection::Down),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn is_reverse(&self, other: &Self) -> bool {
        (*self == MovingDirection::Up && *other == MovingDirection::Down)
            || (*self == MovingDirection::Down && *other == MovingDirection::Up)
            || (*self == MovingDirection::Left && *other == MovingDirection::Right)
            || (*self == MovingDirection::Right && *other == MovingDirection::Left)
    }

    pub fn is_vertical(&self, other: &Self) -> bool {
        !(self == other || self.is_reverse(other))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Cell {
    pub ttype: u8,
    pub direction: MovingDirection,
    pub pos: usize,
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            ttype: 0,
            direction: MovingDirection::Up,
            pos: 0,
        }
    }
}

impl Cell {
    pub fn new(ttype: u8, direction: MovingDirection, pos: usize) -> Self {
        Cell {
            ttype,
            direction,
            pos,
        }
    }
}

#[derive(Debug)]
pub struct Snake(pub Vec<Cell>);

#[derive(Debug)]
pub struct Board {
    width: usize,
    height: usize,
    snake: Snake,
    food: usize,
    new_bodies: VecDeque<Cell>,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        assert!(width > 1 && height > 1);
        Board {
            width,
            height,
            food: 3,
            snake: Snake(vec![
                Cell::new(T_HEAD, MovingDirection::Right, 1),
                Cell::new(T_BODY, MovingDirection::Up, 0),
            ]),
            new_bodies: VecDeque::new(),
        }
    }

    pub fn get_snake(&self) -> &Snake {
        &self.snake
    }

    pub fn get_food_pos(&self) -> usize {
        self.food
    }

    /// get (x_axis, y_axis) from idx
    pub fn idx_2_pos(&self, idx: usize) -> (usize, usize) {
        let x = idx / self.height;
        let y = idx % self.height;
        (x, y)
    }

    pub fn pos_2_idx(&self, (x, y): (usize, usize)) -> usize {
        x * self.height + y
    }

    pub fn move_snake(&mut self, direction: &MovingDirection) {
        let new_cell = self.get_new_head(direction);

        if new_cell.pos == self.food {
            self.new_bodies.push_back(new_cell.clone());
            self.food = self.get_new_food();
        }

        let is_has_new_body = self.new_bodies.len() > 0
            && self.snake.0[self.snake.0.len() - 1].pos == self.new_bodies[0].pos;

        let len = self.snake.0.len();
        for idx in (1..len).rev() {
            self.snake.0[idx] = self.snake.0[idx - 1].clone();
        }

        if len > 1 {
            self.snake.0[1].ttype = T_BODY;
            self.snake.0[1].direction = direction.clone();
        }
        self.snake.0[0] = new_cell;

        if is_has_new_body {
            let mut new_body = self.new_bodies.pop_front().unwrap();
            new_body.ttype = T_BODY;
            self.snake.0.push(new_body);
        }
    }

    pub fn current_direction(&self) -> &MovingDirection {
        &self.snake.0[0].direction
    }

    pub fn input_valid(&self, direction: &MovingDirection) -> bool {
        self.snake.0[0].direction.is_vertical(direction)
    }

    fn get_new_head(&self, direction: &MovingDirection) -> Cell {
        let len = self.len();
        let new_pos = match direction {
            MovingDirection::Up => {
                let (x, mut y) = self.idx_2_pos(self.snake.0[0].pos);
                y = (y + 1) % self.height;
                self.pos_2_idx((x, y))
            }
            MovingDirection::Down => {
                let (x, mut y) = self.idx_2_pos(self.snake.0[0].pos);
                y = (y + self.height - 1) % self.height;
                self.pos_2_idx((x, y))
            }
            MovingDirection::Right => (self.snake.0[0].pos + self.height) % len,
            MovingDirection::Left => (self.snake.0[0].pos + len - self.height) % len,
        };

        Cell::new(T_HEAD, direction.clone(), new_pos)
    }

    fn get_new_food(&self) -> usize {
        let mut rng = rand::thread_rng();
        let die = Uniform::from(0..self.len());
        let mut new_food = die.sample(&mut rng);
        loop {
            if !self.is_in_snake(&new_food) && new_food != self.food {
                return new_food;
            }
            new_food = die.sample(&mut rng);
        }
    }

    fn is_in_snake(&self, idx: &usize) -> bool {
        self.snake
            .0
            .iter()
            .map(|cell| cell.pos)
            .collect::<Vec<usize>>()
            .contains(idx)
    }

    fn len(&self) -> usize {
        self.width * self.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn idx2pos() {
        let b = Board::new(3, 4);
        assert_eq!(b.idx_2_pos(0), (0, 0));
        assert_eq!(b.idx_2_pos(11), (2, 3));
        assert_eq!(b.idx_2_pos(7), (1, 3));
    }

    #[test]
    fn pos2idx() {
        let b = Board::new(5, 5);
        assert_eq!(b.pos_2_idx((0, 0)), 0);
        assert_eq!(b.pos_2_idx((2, 2)), 12);
        assert_eq!(b.pos_2_idx((4, 4)), 24);
    }

    #[test]
    fn dir_reverse() {
        assert_eq!(
            true,
            (MovingDirection::Up).is_reverse(&MovingDirection::Down)
        );
        assert_eq!(
            true,
            (MovingDirection::Left).is_reverse(&MovingDirection::Right)
        );
        assert_eq!(
            true,
            (MovingDirection::Down).is_reverse(&MovingDirection::Up)
        );
        assert_eq!(
            true,
            (MovingDirection::Right).is_reverse(&MovingDirection::Left)
        );
    }

    #[test]
    fn dir_vertical() {
        assert_eq!(
            true,
            (MovingDirection::Up).is_vertical(&MovingDirection::Left)
        );
        assert_eq!(
            true,
            (MovingDirection::Up).is_vertical(&MovingDirection::Right)
        );
        assert_eq!(
            true,
            (MovingDirection::Left).is_vertical(&MovingDirection::Up)
        );
        assert_eq!(
            true,
            (MovingDirection::Down).is_vertical(&MovingDirection::Right)
        );
        assert_eq!(
            true,
            (MovingDirection::Right).is_vertical(&MovingDirection::Down)
        );
    }
}
