use crate::direction::Direction;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    pub fn transform(self, direction: Direction, times: u16) -> Self {
        let times = times as i16;

        let transform = match direction {
            Direction::Up => (0, -times),
            Direction::Down => (0, times),
            Direction::Left => (-times, 0),
            Direction::Right => (times, 0),
        };

        Self::new(
            (self.x as i16 + transform.0) as u16,
            (self.y as i16 + transform.1) as u16,
        )

    }
}
