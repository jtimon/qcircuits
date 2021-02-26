//! Basic struct to handle angles

use rand::Rng;

use std::ops::{Add, Sub};

pub const MAX_ANGLE: u16 = 360;

fn reduce_angle(angle: u16) -> u16 {
    let mut ang = angle;
    while ang >= MAX_ANGLE {
        ang -= MAX_ANGLE;
    }
    ang
}

fn substract_angles(angle_a: u16, angle_b: u16) -> u16 {

    let ang;
    if angle_a > angle_b {
       ang = reduce_angle(angle_a - angle_b);
    } else {
       ang = reduce_angle(angle_a + MAX_ANGLE - angle_b);
    }
    ang
}

#[derive(Debug, PartialEq)]
pub struct Angle {
    pub angle: u16
}

impl Angle {

    pub fn new(angle: u16) -> Angle {
        Angle { angle: reduce_angle(angle) }
    }

    pub fn random_angle() -> Angle {
        Angle { angle: rand::thread_rng().gen_range(0, MAX_ANGLE) }
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
        Angle { angle : reduce_angle(self.angle + _rhs)}
    }
}

impl Add<Angle> for Angle {
    type Output = Angle;

    fn add(self, _rhs: Angle) -> Angle {
        Angle { angle : reduce_angle(self.angle + _rhs.angle)}
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
