use crate::{point::Point, direction::Direction};

#[derive(Debug, Eq, PartialEq)]
pub struct Snake {
    body: Vec<Point>,
    direction: Direction,
    digesting: bool,
}

impl Snake {
    pub fn new(start: Point, length: u16, direction: Direction) -> Self {
        let opposite = direction.opposite();

        let mut body = Vec::new();

        for i in 0..length {
            body.push(start.transform(opposite, i));
        }

        Self {
            body,
            direction,
            digesting: false,
        }
    }

    pub fn get_head(&self) -> &Point {
        self.body.first().unwrap()
    }

    pub fn get_body_points(&self) -> &Vec<Point> {
        &self.body
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }

    pub fn contains_point(&self, point: &Point) -> bool {
        self.body.contains(point)
    }

    pub fn slither(&mut self) {
        self.body
            .insert(0, self.get_head().transform(self.direction, 1));

        if !self.digesting {
            self.body.pop();
        } else {
            self.digesting = false;
        }
    }

    pub fn turn(&mut self, direction: Direction) {
        if self.direction.opposite() != direction {
            self.direction = direction;
        }
    }

    pub fn digest(&mut self) {
        self.digesting = true;
    }
}
