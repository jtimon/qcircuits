//! Basic struct to handle angles

use std::ops::{Add, Sub};

pub const MAX_ANGLE: u16 = 360;

fn substract_angles(angle_a: u16, angle_b: u16) -> u16 {

    let ang;
    if angle_a > angle_b {
       ang = (angle_a - angle_b) % MAX_ANGLE;
    } else {
       ang = (angle_a + MAX_ANGLE - angle_b) % MAX_ANGLE;
    }
    ang
}

#[derive(Debug, PartialEq)]
pub struct Angle {
    pub angle: u16
}

impl Angle {

    pub const fn new(angle: u16) -> Angle {
        Angle { angle: angle % MAX_ANGLE }
    }

    pub fn between(&self, angle_a: u16, angle_b: u16) -> bool {
        if angle_a <= angle_b {
            self.angle >= angle_a && self.angle < angle_b
        } else {
            self.angle >= angle_a || self.angle < angle_b
        }
    }
}

impl Clone for Angle {
    fn clone(&self) -> Angle {
        Angle { angle: self.angle }
    }
}

impl Add<u16> for Angle {
    type Output = Angle;

    fn add(self, _rhs: u16) -> Angle {
        Angle { angle : (self.angle + _rhs) % MAX_ANGLE}
    }
}

impl Add<Angle> for Angle {
    type Output = Angle;

    fn add(self, _rhs: Angle) -> Angle {
        Angle { angle : (self.angle + _rhs.angle) % MAX_ANGLE}
    }
}

impl Sub<u16> for Angle {
    type Output = Angle;

    fn sub(self, _rhs: u16) -> Angle {
        Angle { angle : substract_angles(self.angle, _rhs)}
    }
}

impl Sub<Angle> for Angle {
    type Output = Angle;

    fn sub(self, _rhs: Angle) -> Angle {
        Angle { angle : substract_angles(self.angle, _rhs.angle)}
    }
}
