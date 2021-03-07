
use rand::Rng;

use crate::angle::{Angle, MAX_ANGLE};
use crate::hypotheses::det_2angs::DetTwoAngleParticle;
use crate::hypotheses::det_ang::DetAngleParticle;
use crate::hypotheses::det_bits::DetBitParticle;
use crate::hypotheses::rand_ang::AngleParticle;
use crate::hypotheses::rand_enum::{EnumParticle, ParticleState};
use crate::circuits::{
    Filter,
    ParticleSource,
};

#[derive(Copy, Clone)]
pub struct EnumParticleSource;

impl ParticleSource for EnumParticleSource {
    fn emit_particles(&self, filter: &Filter, particles: u32) -> Filter {
        let mut filter = filter.clone();
        for _ in 0..particles {
            let rand_enum = rand::thread_rng().gen_range(0, 4);
            let mut p;
            if rand_enum == 0 {
                p = EnumParticle::new(ParticleState::Up);
            } else if rand_enum == 1 {
                p = EnumParticle::new(ParticleState::Down);
            } else if rand_enum == 2 {
                p = EnumParticle::new(ParticleState::Left);
            } else {
                p = EnumParticle::new(ParticleState::Right);
            }
            filter.receive_particle(&mut p);
        }
        filter
    }
}

#[derive(Copy, Clone)]
pub struct AngleParticleSource;

impl ParticleSource for AngleParticleSource {
    fn emit_particles(&self, filter: &Filter, particles: u32) -> Filter {
        let mut filter = filter.clone();
        for _ in 0..particles {
            let mut p = AngleParticle::new(rand::thread_rng().gen_range(0, MAX_ANGLE));
            filter.receive_particle(&mut p);
        }
        filter
    }
}

#[derive(Copy, Clone)]
pub struct DetAngleParticleSource;

impl ParticleSource for DetAngleParticleSource {
    fn emit_particles(&self, filter: &Filter, particles: u32) -> Filter {
        let mut filter = filter.clone();
        for _ in 0..particles {
            let mut p = DetAngleParticle::new(rand::thread_rng().gen_range(0, MAX_ANGLE));
            filter.receive_particle(&mut p);
        }
        filter
    }
}

#[derive(Copy, Clone)]
pub struct DetAngleParticleSourceDebug;

impl ParticleSource for DetAngleParticleSourceDebug {
    fn emit_particles(&self, filter: &Filter, particles: u32) -> Filter {
        let mut angle = Angle::new(0);
        let mut filter = filter.clone();
        for _ in 0..particles {
            let mut p = DetAngleParticle::new(angle.angle);
            filter.receive_particle(&mut p);
            angle = angle + 1;
        }
        filter
    }
}

#[derive(Copy, Clone)]
pub struct DetTwoAngleParticleSource;

impl ParticleSource for DetTwoAngleParticleSource {
    fn emit_particles(&self, filter: &Filter, particles: u32) -> Filter {
        let mut filter = filter.clone();
        for _ in 0..particles {
            let mut p = DetTwoAngleParticle::new(
                rand::thread_rng().gen_range(0, MAX_ANGLE),
                rand::thread_rng().gen_range(0, MAX_ANGLE));
            filter.receive_particle(&mut p);
        }
        filter
    }
}

#[derive(Copy, Clone)]
pub struct DetTwoAngleParticleSourceDebug;

impl ParticleSource for DetTwoAngleParticleSourceDebug {
    fn emit_particles(&self, filter: &Filter, particles: u32) -> Filter {
        let mut angle_a : u16 = 0;
        let mut angle_b : u16 = 0;
        let mut filter = filter.clone();
        for _ in 0..particles {
            let mut p = DetTwoAngleParticle::new(angle_a, angle_b);
            filter.receive_particle(&mut p);
            if angle_a == 359 {
                angle_a = 0;
                if angle_b == 359 {
                    angle_b = 0;
                } else {
                    angle_b = angle_b + 1;
                }
            } else {
                angle_a = angle_a + 1;
            }
        }
        filter
    }
}

#[derive(Copy, Clone)]
pub struct DetBitsParticleSource
{
    bit_count: usize
}

impl DetBitsParticleSource {

    pub fn new(bit_count: usize) -> DetBitsParticleSource {
        DetBitsParticleSource { bit_count }
    }
}

impl ParticleSource for DetBitsParticleSource {
    fn emit_particles(&self, filter: &Filter, particles: u32) -> Filter {
        let mut filter = filter.clone();
        for _ in 0..particles {
            let mut v: Vec<bool> = Vec::new();
            for _ in 0..self.bit_count + 1 {
                v.push(
                    if rand::thread_rng().gen_range(0, 2) > 0 {
                        true
                    } else {
                        false
                    }
                );
            }
            let is_updown = v.remove(v.len() - 1);
            let mut p = DetBitParticle::new(v, is_updown);
            filter.receive_particle(&mut p);
        }
        filter
    }
}
