//! Implement the hypothesis that particles have an angle as a state

use rand::Rng;

use crate::angle::{Angle, MAX_ANGLE};
use crate::circuits::Particle;

const UP: u16 = 0;
const RIGHT: u16 = 90;
const DOWN: u16 = 180;
const LEFT: u16 = 270;
const ACCEPTANCE_ANGLE: u16 = 45;

pub struct AngleParticle {
    state: Angle
}

impl AngleParticle {

    pub fn new(angle: u16) -> AngleParticle {
        AngleParticle { state: Angle::new(angle) }
    }

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
}

impl Particle for AngleParticle {

    fn observe_updown(&mut self) -> bool {
        if self.is_up() {
            return true;
        } else if self.is_down() {
            return false;
        }
        // 50% chance of going either way
        let mut a = rand::thread_rng().gen_range(0, ACCEPTANCE_ANGLE * 4);
        if a < ACCEPTANCE_ANGLE {
            a = a + LEFT + ACCEPTANCE_ANGLE;
        } else if a < ACCEPTANCE_ANGLE * 2 {
            a = a - ACCEPTANCE_ANGLE;
        } else if a >= ACCEPTANCE_ANGLE * 2 {
            a = a + ACCEPTANCE_ANGLE;
        }
        self.state.angle = a;
        assert!(self.state.angle < MAX_ANGLE);
        self.is_up()
    }

    fn observe_leftright(&mut self) -> bool {
        if self.is_left() {
            return true;
        } else if self.is_right() {
            return false;
        }
        // 50% chance of going either way
        let mut a = rand::thread_rng().gen_range(0, ACCEPTANCE_ANGLE * 4);
        if a >= ACCEPTANCE_ANGLE * 2 {
            a = a + ACCEPTANCE_ANGLE * 2;
        }
        a = a + ACCEPTANCE_ANGLE;
        self.state.angle = a;
        assert!(self.state.angle < MAX_ANGLE);
        self.is_left()
    }
}
