//! Basic struct to handle angles

use rand::Rng;

const MAX_ANGLE: u16 = 360;

pub struct Angle {
    pub angle: u16
}

impl Angle {

    pub fn random_angle() -> Angle {
        Angle { angle: rand::thread_rng().gen_range(0, MAX_ANGLE) }
    }
}
