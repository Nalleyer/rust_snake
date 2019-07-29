use crate::assets::{T_BODY, T_HEAD};

#[derive(Debug, PartialEq, Clone)]
pub enum MovingDirection {
    Left,
    Right,
    Up,
    Down,
}

impl MovingDirection {
    pub fn from_axis_x(x: f64) -> Option<Self> {
        if x > 0.8 {
            Some(MovingDirection::Right)
        } else if x < -0.8 {
            Some(MovingDirection::Left)
        } else {
            None
        }
    }

    pub fn from_axis_y(y: f64) -> Option<Self> {
        if y > 0.8 {
            Some(MovingDirection::Up)
        } else if y < -0.8 {
            Some(MovingDirection::Down)
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
        let len = self.snake.0.len();
        for idx in (1..len).rev() {
            self.snake.0[idx] = self.snake.0[idx - 1].clone();
        }

        if len > 1 {
            self.snake.0[1].ttype = T_BODY;
            self.snake.0[1].direction = direction.clone();
        }
        self.snake.0[0] = new_cell;
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
