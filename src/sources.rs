
use rand::Rng;

use crate::angle::{MAX_ANGLE};
use crate::hypotheses::det_2angs::DetTwoAngleParticle;
use crate::hypotheses::det_ang::DetAngleParticle;
use crate::hypotheses::det_bits::DetBitParticle;
use crate::hypotheses::rand_ang::AngleParticle;
use crate::hypotheses::rand_enum::{EnumParticle, ParticleState};
use crate::circuits::{
    Particle,
    ParticleFactory,
};

#[derive(Copy, Clone)]
pub struct EnumParticleSource;

impl ParticleFactory for EnumParticleSource {
    fn get_particle(&mut self) -> Box<dyn Particle> {
        let rand_enum = rand::thread_rng().gen_range(0, 4);
        if rand_enum == 0 {
            Box::new(EnumParticle::new(ParticleState::Up))
        } else if rand_enum == 1 {
            Box::new(EnumParticle::new(ParticleState::Down))
        } else if rand_enum == 2 {
            Box::new(EnumParticle::new(ParticleState::Left))
        } else {
            Box::new(EnumParticle::new(ParticleState::Right))
        }
    }
}

#[derive(Copy, Clone)]
pub struct AngleParticleSource;

impl ParticleFactory for AngleParticleSource {
    fn get_particle(&mut self) -> Box<dyn Particle> {
        Box::new(AngleParticle::new(rand::thread_rng().gen_range(0, MAX_ANGLE)))
    }
}

#[derive(Copy, Clone)]
pub struct DetAngleParticleSource;

impl ParticleFactory for DetAngleParticleSource {
    fn get_particle(&mut self) -> Box<dyn Particle> {
        Box::new(DetAngleParticle::new(rand::thread_rng().gen_range(0, MAX_ANGLE)))
    }
}

#[derive(Copy, Clone)]
pub struct DetAngleParticleSourceDebug {
    angle: u16,
}

impl DetAngleParticleSourceDebug {
    pub const fn new() -> DetAngleParticleSourceDebug {
        DetAngleParticleSourceDebug { angle: 0 }
    }
}

impl ParticleFactory for DetAngleParticleSourceDebug {
    fn get_particle(&mut self) -> Box<dyn Particle> {
        let p = DetAngleParticle::new(self.angle);
        if self.angle == MAX_ANGLE - 1 {
            self.angle = 0;
        } else {
            self.angle = self.angle + 1;
        }
        Box::new(p)
    }
}

#[derive(Copy, Clone)]
pub struct DetTwoAngleParticleSource;

impl ParticleFactory for DetTwoAngleParticleSource {
    fn get_particle(&mut self) -> Box<dyn Particle> {
        Box::new(DetTwoAngleParticle::new(rand::thread_rng().gen_range(0, MAX_ANGLE), rand::thread_rng().gen_range(0, MAX_ANGLE)))
    }
}

#[derive(Copy, Clone)]
pub struct DetTwoAngleParticleSourceDebug {
    angle_a: u16,
    angle_b: u16,
}

impl DetTwoAngleParticleSourceDebug {
    pub const fn new() -> DetTwoAngleParticleSourceDebug {
        DetTwoAngleParticleSourceDebug { angle_a: 0, angle_b: 0 }
    }
}

impl ParticleFactory for DetTwoAngleParticleSourceDebug {
    fn get_particle(&mut self) -> Box<dyn Particle> {
        let p = DetTwoAngleParticle::new(self.angle_a, self.angle_b);
        if self.angle_a == MAX_ANGLE - 1 {
            self.angle_a = 0;
            if self.angle_b == MAX_ANGLE - 1 {
                self.angle_b = 0;
            } else {
                self.angle_b = self.angle_b + 1;
            }
        } else {
            self.angle_a = self.angle_a + 1;
        }
        Box::new(p)
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

impl ParticleFactory for DetBitsParticleSource {
    fn get_particle(&mut self) -> Box<dyn Particle> {
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
        Box::new(DetBitParticle::new(v, is_updown))
    }
}
