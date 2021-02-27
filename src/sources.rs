
use rand::Rng;

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
