//! Implement the hypothesis that particles have an angle as a state in a deterministic way

use crate::angle::{Angle, MAX_ANGLE, UP, RIGHT, DOWN, LEFT};
use crate::circuits::Particle;

const ACCEPT_TRANS_ANGLE: u16 = 45;
const ACCEPTANCE_ANGLE: u16 = 90;
const TRANSFORM_ANGLE: Angle = Angle::new(135);

pub struct DetAngleParticle {
    state: Angle
}

impl DetAngleParticle {

    pub fn new(angle: u16) -> DetAngleParticle {
        DetAngleParticle { state: Angle::new(angle) }
    }
}

impl Particle for DetAngleParticle {

    fn is_up(&self) -> bool {
        self.state.between(UP + MAX_ANGLE - ACCEPTANCE_ANGLE, UP + ACCEPTANCE_ANGLE)
    }

    fn is_down(&self) -> bool {
        self.state.between(DOWN - ACCEPTANCE_ANGLE, DOWN + ACCEPTANCE_ANGLE)
    }

    fn is_left(&self) -> bool {
        self.state.between(LEFT - ACCEPTANCE_ANGLE, LEFT + ACCEPTANCE_ANGLE)
    }

    fn is_right(&self) -> bool {
        self.state.between(RIGHT - ACCEPTANCE_ANGLE, RIGHT + ACCEPTANCE_ANGLE)
    }

    fn observe_updown(&mut self) {
        // up_right_right or down_left_left
        if self.state.between(RIGHT - ACCEPT_TRANS_ANGLE, RIGHT) || self.state.between(LEFT - ACCEPT_TRANS_ANGLE, LEFT) {
            self.state = self.state.clone() - TRANSFORM_ANGLE;
        // up_left_left or down_right_right
        } else if self.state.between(LEFT, LEFT + ACCEPT_TRANS_ANGLE) || self.state.between(RIGHT, RIGHT + ACCEPT_TRANS_ANGLE) {
            self.state = self.state.clone() + TRANSFORM_ANGLE;
        }
        assert!(self.state.angle < MAX_ANGLE);
    }

    fn observe_leftright(&mut self) {
        // up_up_left or down_down_right
        if self.state.between(UP + MAX_ANGLE - ACCEPT_TRANS_ANGLE, UP) || self.state.between(DOWN - ACCEPT_TRANS_ANGLE, DOWN) {
            self.state = self.state.clone() - TRANSFORM_ANGLE;
        // up_up_right or down_down_left
        } else if self.state.between(UP, UP + ACCEPT_TRANS_ANGLE) || self.state.between(DOWN, DOWN + ACCEPT_TRANS_ANGLE) {
            self.state = self.state.clone() + TRANSFORM_ANGLE;
        }
        assert!(self.state.angle < MAX_ANGLE);
    }
}
