use std::sync::Arc;
use crate::coordinates::{Boundary, Vec2d};

pub trait TrackSection {
    fn is_within(&self, pos: &Vec2d) -> bool;
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
}

pub struct Track {
    pub start: Vec2d,
    pub finish_line: Boundary,
    pub sections: Vec<Box<dyn TrackSection>>,
}

impl Track {
    pub fn is_within_track(&self, point: &Vec2d) -> bool {
        self.sections.iter().any(|section| section.is_within(point))
    }

    pub fn is_finished(&self, point: &Vec2d) -> bool {
        !self.finish_line.point_within(point)
    }
}