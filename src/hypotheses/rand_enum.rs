//! Implement the hypothesis that particles have an angle as a state

use rand::Rng;

use crate::circuits::Particle;

#[derive(PartialEq)]
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

    fn is_up(&self) -> bool {
        self.state == ParticleState::Up
    }

    fn is_down(&self) -> bool {
        self.state == ParticleState::Down
    }

    fn is_left(&self) -> bool {
        self.state == ParticleState::Left
    }

    fn is_right(&self) -> bool {
        self.state == ParticleState::Right
    }

    fn observe_updown(&mut self) {
        if self.state != ParticleState::Up && self.state != ParticleState::Down {
            self.set_random_updown();
        }
    }

    fn observe_leftright(&mut self) {
        if self.state != ParticleState::Left && self.state != ParticleState::Right {
            self.set_random_leftright();
        }
    }
}
