use crate::car::TerminationCondition;
use crate::coordinates::Boundary;
use crate::track::{ParallelRectSection, Track};

pub fn make_track(// world: &mut World
) -> Track {
    let track_sect = ParallelRectSection {
        left_x: -50.0,
        right_x: 50.0,
        top_y: 380.0,
        bottom_y: -10.0,
    };
    Track {
        start: Default::default(),
        finish_line: Boundary::horizontal(350.0, true),
        sections: vec![Box::new(track_sect)],

        termination_condition: TerminationCondition::Seconds(30.0),
    }
}
