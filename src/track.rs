use crate::coordinates::Vec2d;

pub trait TrackSection {
    fn is_within(&self, pos: Vec2d) -> bool;
}

pub struct ParallelRectSection {
    left_x: f32,
    right_x: f32,
    top_y: f32,
    bottom_y: f32,
}

impl TrackSection for ParallelRectSection {
    fn is_within(&self, pos: Vec2d) -> bool {
        (pos.x > self.left_x) && (pos.x < self.right_x) && (pos.y > self.bottom_y) && (pos.y < self.top_y)
    }
}

pub struct Track {
    start: Vec2d,
    finish_line:
}