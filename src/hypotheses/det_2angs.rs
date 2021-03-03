//! Implement the hypothesis that particles have two angles as a state in a deterministic way

use crate::angle::{Angle, MAX_ANGLE, UP, LEFT};
use crate::circuits::Particle;

const ACCEPTANCE_ANGLE: u16 = 90;
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
        // self.angle_leftright = self.angle_leftright.clone() + self.angle_updown.clone() + TRANSFORM_ANGLE;
        self.angle_leftright = self.angle_leftright.clone() + TRANSFORM_ANGLE;
        self.is_up()
    }

    fn observe_leftright(&mut self) -> bool {
        // self.angle_updown = self.angle_updown.clone() + self.angle_leftright.clone() + TRANSFORM_ANGLE;
        self.angle_updown = self.angle_updown.clone() + TRANSFORM_ANGLE;
        self.is_left()
    }
}
