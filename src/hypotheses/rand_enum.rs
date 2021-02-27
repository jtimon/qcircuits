//! Implement the hypothesis that particles have an angle as a state

use rand::Rng;

use crate::circuits::Particle;

pub enum ParticleState { Up, Down, Left, Right }

pub struct EnumParticle {
    state: ParticleState
}

impl EnumParticle {

    pub fn new(state: ParticleState) -> EnumParticle {
        EnumParticle { state }
    }

    fn set_random_updown(&mut self) -> bool {
        if rand::thread_rng().gen_range(0, 2) > 0 {
            self.state = ParticleState::Up;
            return true
        } else {
            self.state = ParticleState::Down;
        }
        false
    }

    fn set_random_leftright(&mut self) -> bool {
        if rand::thread_rng().gen_range(0, 2) > 0 {
            self.state = ParticleState::Left;
            return true
        } else {
            self.state = ParticleState::Right;
        }
        false
    }
}

impl Particle for EnumParticle {

    fn observe_updown(&mut self) -> bool {
        match self.state {
            ParticleState::Up => true,
            ParticleState::Down => false,
            _ => self.set_random_updown()
        }
    }

    fn observe_leftright(&mut self) -> bool {
        match self.state {
            ParticleState::Left => true,
            ParticleState::Right => false,
            _ => self.set_random_leftright()
        }
    }
}
