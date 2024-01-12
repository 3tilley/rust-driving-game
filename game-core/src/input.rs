use std::time::Duration;

#[derive(Debug, Copy, Clone)]
pub enum Accelerator {
    Accelerate,
    Brake,
}

impl Accelerator {
    pub fn from_up_down(up_pressed: bool, down_pressed: bool) -> Option<Accelerator> {
        if up_pressed && down_pressed {
            None
        } else if up_pressed {
            Some(Accelerator::Accelerate)
        } else if down_pressed {
            Some(Accelerator::Brake)
        } else {
            None
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Left,
    Right,
}

impl Direction {
    pub fn from_left_right(left_pressed: bool, right_pressed: bool) -> Option<Direction> {
        if left_pressed && right_pressed {
            None
        } else if left_pressed {
            Some(Direction::Left)
        } else if right_pressed {
            Some(Direction::Right)
        } else {
            None
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct KeyInput {
    pub acceleration: Option<Accelerator>,
    pub direction: Option<Direction>,
}

impl KeyInput {
    pub fn new(acceleration: Option<Accelerator>, direction: Option<Direction>) -> KeyInput {
        KeyInput {
            acceleration,
            direction,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.acceleration.is_none() && self.direction.is_none()
    }
}

pub trait Input {
    fn get_input() -> KeyInput;
}

pub struct TerminalInput {}

// impl Input for TerminalInput {
//
//     fn get_input() -> KeyInput {
//         // let stream = crossterm::event::E
//
//         if poll(Duration::from_micros(10)).unwrap() {
//             // It's guaranteed that `read` won't block, because `poll` returned
//             // `Ok(true)`.
//             let event = read().unwrap();
//             if let Event::Key(key) = event {
//                 match key {
//                     KeyEvent { code, modifiers, kind, state } => {
//                         if kind != KeyEventKind::Release {
//                             match code {
//                                 KeyCode::Left => return KeyInput::new(None, Some(Direction::Left), Some(kind)),
//                                 KeyCode::Right => return KeyInput::new(None, Some(Direction::Right), Some(kind)),
//                                 KeyCode::Up => return KeyInput::new(Some(Accelerate), None, Some(kind)),
//                                 KeyCode::Down => return KeyInput::new(Some(Brake), None, Some(kind)),
//                                 _ => {}
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//         KeyInput::default()
//     }
// }
