use crate::assets::{T_BODY, T_HEAD};

#[derive(Debug, PartialEq, Clone)]
pub enum MovingDirection {
    None,
    Left,
    Right,
    Up,
    Down,
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
            direction: MovingDirection::None,
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
            food: 1,
            snake: Snake(vec![Cell::new(T_HEAD, MovingDirection::Right, 0)]),
        }
    }

    pub fn get_snake(&self) -> &Snake {
        &self.snake
    }

    pub fn get_food_pos(&self) -> usize {
        self.food
    }

    /// get (row, column) from idx
    pub fn idx_2_pos(&self, idx: usize) -> (usize, usize) {
        let row = idx / self.width;
        let column = idx % self.width;
        (row, column)
    }

    pub fn pos_2_idx(&self, (row, column): (usize, usize)) -> usize {
        row * self.width + column
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn idx2pos() {
        let b = Board::new(3, 4);
        assert_eq!(b.idx_2_pos(0), (0, 0));
        assert_eq!(b.idx_2_pos(11), (3, 2));
        assert_eq!(b.idx_2_pos(7), (2, 1));
    }

    #[test]
    fn pos2idx() {
        let b = Board::new(5, 5);
        assert_eq!(b.pos_2_idx((0, 0)), 0);
        assert_eq!(b.pos_2_idx((2, 2)), 12);
        assert_eq!(b.pos_2_idx((4, 4)), 24);
    }
}
