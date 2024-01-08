use std::sync::Arc;
use crate::coordinates::{Boundary, Vec2d};

pub trait TrackSection {
    fn is_within(&self, pos: &Vec2d) -> bool;

    fn edges(&self) -> Vec<(Vec2d, Vec2d)>;
}

pub struct ParallelRectSection {
    pub left_x: f32,
    pub right_x: f32,
    pub top_y: f32,
    pub bottom_y: f32,
}

impl TrackSection for ParallelRectSection {
    fn is_within(&self, pos: &Vec2d) -> bool {
        (pos.x > self.left_x) && (pos.x < self.right_x) && (pos.y > self.bottom_y) && (pos.y < self.top_y)
    }
    fn edges(&self) -> Vec<(Vec2d, Vec2d)> {
        let mut vec = Vec::with_capacity(4);
        let bot_left = Vec2d {x: self.left_x, y: self.bottom_y};
        let top_left = Vec2d {x: self.left_x, y: self.top_y};
        let top_right = Vec2d {x: self.right_x, y: self.top_y};
        let bot_right = Vec2d{ x: self.right_x, y: self.bottom_y};
        vec.push((bot_left, top_left));
        vec.push((top_left, top_right));
        vec.push((top_right, bot_right));
        vec.push((bot_right, bot_left));
        vec
    }
}

pub struct Track {
    pub start: Vec2d,
    pub finish_line: Boundary,
    pub sections: Vec<Box<dyn TrackSection + Send + Sync>>,
}

impl Track {
    pub fn is_within_track(&self, point: &Vec2d) -> bool {
        self.sections.iter().any(|section| section.is_within(point))
    }

    pub fn is_finished(&self, point: &Vec2d) -> bool {
        !self.finish_line.point_within(point)
    }
}