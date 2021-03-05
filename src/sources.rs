
use rand::Rng;

use crate::angle::{MAX_ANGLE};
use crate::hypotheses::det_2angs::DetTwoAngleParticle;
use crate::hypotheses::det_ang::DetAngleParticle;
use crate::hypotheses::det_bits::DetBitParticle;
use crate::hypotheses::rand_ang::AngleParticle;
use crate::hypotheses::rand_enum::{EnumParticle, ParticleState};
use crate::circuits::{
    Particle,
    ParticleSource,
};

pub struct EnumParticleSource;

impl ParticleSource for EnumParticleSource {
    fn get_particle(&mut self) -> Box<dyn Particle + Send> {
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

pub struct AngleParticleSource;

impl ParticleSource for AngleParticleSource {
    fn get_particle(&mut self) -> Box<dyn Particle + Send> {
        Box::new(AngleParticle::new(rand::thread_rng().gen_range(0, MAX_ANGLE)))
    }
}

pub struct DetAngleParticleSource;

impl ParticleSource for DetAngleParticleSource {
    fn get_particle(&mut self) -> Box<dyn Particle + Send> {
        Box::new(DetAngleParticle::new(rand::thread_rng().gen_range(0, MAX_ANGLE)))
    }
}

pub struct DetAngleParticleSourceDebug {
    angle: u16,
}

impl DetAngleParticleSourceDebug {
    pub const fn new() -> DetAngleParticleSourceDebug {
        DetAngleParticleSourceDebug { angle: 0 }
    }
}

impl ParticleSource for DetAngleParticleSourceDebug {
    fn get_particle(&mut self) -> Box<dyn Particle + Send> {
        let p = DetAngleParticle::new(self.angle);
        if self.angle == MAX_ANGLE - 1 {
            self.angle = 0;
        } else {
            self.angle = self.angle + 1;
        }
        Box::new(p)
    }
}

pub struct DetTwoAngleParticleSource;

impl ParticleSource for DetTwoAngleParticleSource {
    fn get_particle(&mut self) -> Box<dyn Particle + Send> {
        Box::new(DetTwoAngleParticle::new(rand::thread_rng().gen_range(0, MAX_ANGLE), rand::thread_rng().gen_range(0, MAX_ANGLE)))
    }
}

pub struct DetTwoAngleParticleSourceDebug {
    angle_a: u16,
    angle_b: u16,
}

impl DetTwoAngleParticleSourceDebug {
    pub const fn new() -> DetTwoAngleParticleSourceDebug {
        DetTwoAngleParticleSourceDebug { angle_a: 0, angle_b: 0 }
    }
}

impl ParticleSource for DetTwoAngleParticleSourceDebug {
    fn get_particle(&mut self) -> Box<dyn Particle + Send> {
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
    fn get_particle(&mut self) -> Box<dyn Particle + Send> {
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
