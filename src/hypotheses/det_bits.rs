//! Implement the hypothesis that particles have a binary string as a state
//! Is this "cheating"? It feels like it.

use crate::circuits::Particle;

pub struct DetBitParticle {
    bitvec: Vec<bool>,
    is_updown: bool
}

impl DetBitParticle {

    pub fn new(bitvec: Vec<bool>, is_updown: bool) -> DetBitParticle {
        DetBitParticle { bitvec, is_updown }
    }
}

impl Particle for DetBitParticle {

    fn observe_updown(&mut self) -> bool {
        if !self.is_updown {
            let front = self.bitvec.remove(0);
            self.bitvec.push(front);
            self.is_updown = true;
        }
        self.bitvec[0]
    }

    fn observe_leftright(&mut self) -> bool {
        if self.is_updown {
            let front = self.bitvec.remove(0);
            self.bitvec.push(front);
            self.is_updown = false;
        }
        self.bitvec[0]
    }
}
