
use rand::Rng;

use crate::angle::Angle;
use crate::hypotheses::ran_ang::AngleParticle;
use crate::circuits::{
    Filter,
    ParticleSource,
};

const MAX_ANGLE: u16 = 360;

pub struct AngleParticleSource;

impl ParticleSource for AngleParticleSource {
    fn emit_particles(&self, filter: &mut Filter, particles: u32){
        for _ in 0..particles {
            let mut p = AngleParticle::new(rand::thread_rng().gen_range(0, MAX_ANGLE));
            filter.receive_particle(&mut p);
        }
    }
}

pub struct AngleParticleSourceDebug;

impl ParticleSource for AngleParticleSourceDebug {
    fn emit_particles(&self, filter: &mut Filter, particles: u32){
        let mut angle = Angle::new(0);
        for _ in 0..particles {
            let mut p = AngleParticle::new(angle.angle);
            filter.receive_particle(&mut p);
            angle = angle + 1;
        }
    }
}
