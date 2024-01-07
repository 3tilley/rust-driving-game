
#[derive(Copy, Clone, Debug, Default)]
pub struct Vec2d {
    pub x: f32,
    pub y: f32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Axis {
    X,
    Y,
}

pub enum LineType {
    Horizontal(f32),
    Vertical(f32),
    // y = mx + c
    // Args are m, c
    Diagonal(f32, f32)
}

pub struct Boundary {
    pub line_type: LineType,
    pub positive_inf_within: bool,
}

impl Boundary {
    pub fn vertical(x: f32, left_within: bool) -> Boundary {
        Boundary {
            line_type: LineType::Vertical(x),
            positive_inf_within: !left_within,
        }
    }

    pub fn horizontal(y: f32, below_within: bool) -> Boundary {
        Boundary {
            line_type: LineType::Horizontal(y),
            positive_inf_within: !below_within,
        }
    }

    pub fn point_within(&self, point: &Vec2d) -> bool {
        // point > intercept    pos_inf_within  |   is_within
        //                                      |
        // 1                    1               |   1
        // 1                    0               |   0
        // 0                    1               |   0
        // 0                    0               |   1
        match self.line_type {
            LineType::Horizontal(y_intercept) => !((point.y > y_intercept) ^ self.positive_inf_within),
            LineType::Vertical(x_intercept) => !((point.x > x_intercept) ^ self.positive_inf_within),
            LineType::Diagonal(m, c_y) => {
                let y_line = m * point.x + c_y;
                !((point.y > y_line) ^ self.positive_inf_within)
            }
        }
    }
}
