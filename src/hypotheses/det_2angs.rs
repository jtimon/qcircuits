//! Implement the hypothesis that particles have two angles as a state in a deterministic way

use crate::angle::{Angle, MAX_ANGLE, UP, DOWN, LEFT, RIGHT};
use crate::circuits::Particle;

const ACCEPT_TRANS_ANGLE: u16 = 22;
const ACCEPTANCE_ANGLE: u16 = 67;
const TRANSFORM_ANGLE: Angle = Angle::new(90);

pub struct DetTwoAngleParticle {
    angle_updown: Angle,
    angle_leftright: Angle
}

impl DetTwoAngleParticle {

    pub fn new(angle_updown: u16, angle_leftright: u16) -> DetTwoAngleParticle {
        DetTwoAngleParticle { angle_updown: Angle::new(angle_updown), angle_leftright: Angle::new(angle_leftright) }
    }

    fn is_up(&self) -> bool {
        self.angle_updown.between(UP + MAX_ANGLE - ACCEPTANCE_ANGLE, UP + ACCEPTANCE_ANGLE)
    }

    fn is_left(&self) -> bool {
        self.angle_leftright.between(LEFT - ACCEPTANCE_ANGLE, LEFT + ACCEPTANCE_ANGLE)
    }
}

impl Particle for DetTwoAngleParticle {

    fn observe_updown(&mut self) -> bool {
        // up_right_right or down_left_left
        if self.angle_updown.between(RIGHT - ACCEPT_TRANS_ANGLE, RIGHT) || self.angle_updown.between(LEFT - ACCEPT_TRANS_ANGLE, LEFT) {
            self.angle_updown = self.angle_updown.clone() - TRANSFORM_ANGLE;
        // up_left_left or down_right_right
        } else if self.angle_updown.between(LEFT, LEFT + ACCEPT_TRANS_ANGLE) || self.angle_updown.between(RIGHT, RIGHT + ACCEPT_TRANS_ANGLE) {
            self.angle_updown = self.angle_updown.clone() + TRANSFORM_ANGLE;
        }

        // self.angle_leftright = self.angle_leftright.clone() + self.angle_updown.clone() + TRANSFORM_ANGLE;
        self.angle_leftright = self.angle_leftright.clone() + TRANSFORM_ANGLE;
        self.is_up()
    }

    fn observe_leftright(&mut self) -> bool {
        // up_up_left or down_down_right
        if self.angle_leftright.between(UP + MAX_ANGLE - ACCEPT_TRANS_ANGLE, UP) || self.angle_leftright.between(DOWN - ACCEPT_TRANS_ANGLE, DOWN) {
            self.angle_leftright = self.angle_leftright.clone() - TRANSFORM_ANGLE;
        // up_up_right or down_down_left
        } else if self.angle_leftright.between(UP, UP + ACCEPT_TRANS_ANGLE) || self.angle_leftright.between(DOWN, DOWN + ACCEPT_TRANS_ANGLE) {
            self.angle_leftright = self.angle_leftright.clone() + TRANSFORM_ANGLE;
        }

        // self.angle_updown = self.angle_updown.clone() + self.angle_leftright.clone() + TRANSFORM_ANGLE;
        self.angle_updown = self.angle_updown.clone() + TRANSFORM_ANGLE;
        self.is_left()
    }
}
