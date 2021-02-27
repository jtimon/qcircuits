//! The circuit module contains the structs and methods to create and run the circuits

use rand::Rng;

use crate::angle::Angle;

/// A particle can be observed by a Filter or counted by a Detector.
pub trait Particle {
    // if up returns true, false otherwise
    fn observe_updown(&mut self) -> bool;
    // if left returns true, false otherwise
    fn observe_leftright(&mut self) -> bool;
}

pub enum FilterType {UpDown, LeftRight}

const UP: u16 = 0;
const RIGHT: u16 = 90;
const DOWN: u16 = 180;
const LEFT: u16 = 270;
const MAX: u16 = 360;
const ACCEPTANCE_ANGLE: u16 = 45;

pub struct AngleParticle {
    state: Angle
}

impl AngleParticle {

    pub fn new() -> AngleParticle {
        AngleParticle {
            state: Angle::random_angle(),
        }
    }

    fn is_up(&self) -> bool {
        (self.state.angle < UP + ACCEPTANCE_ANGLE) ||
            (self.state.angle >= UP + MAX - ACCEPTANCE_ANGLE)
    }

    fn is_down(&self) -> bool {
        (self.state.angle < DOWN + ACCEPTANCE_ANGLE) &&
            (self.state.angle >= DOWN - ACCEPTANCE_ANGLE)
    }

    fn is_left(&self) -> bool {
        (self.state.angle < LEFT + ACCEPTANCE_ANGLE) &&
            (self.state.angle >= LEFT - ACCEPTANCE_ANGLE)
    }

    fn is_right(&self) -> bool {
        (self.state.angle < RIGHT + ACCEPTANCE_ANGLE) &&
            (self.state.angle >= RIGHT - ACCEPTANCE_ANGLE)
    }
}

impl Particle for AngleParticle {

    fn observe_updown(&mut self) -> bool {
        if self.is_up() {
            return true;
        } else if self.is_down() {
            return false;
        }
        // 50% chance of going either way
        let mut a = rand::thread_rng().gen_range(0, ACCEPTANCE_ANGLE * 4);
        if a < ACCEPTANCE_ANGLE {
            a = a + LEFT + ACCEPTANCE_ANGLE;
        } else if a < ACCEPTANCE_ANGLE * 2 {
            a = a - ACCEPTANCE_ANGLE;
        } else if a >= ACCEPTANCE_ANGLE * 2 {
            a = a + ACCEPTANCE_ANGLE;
        }
        self.state.angle = a;
        assert!(self.state.angle < MAX);
        self.is_up()
    }

    fn observe_leftright(&mut self) -> bool {
        if self.is_left() {
            return true;
        } else if self.is_right() {
            return false;
        }
        // 50% chance of going either way
        let mut a = rand::thread_rng().gen_range(0, ACCEPTANCE_ANGLE * 4);
        if a >= ACCEPTANCE_ANGLE * 2 {
            a = a + ACCEPTANCE_ANGLE * 2;
        }
        a = a + ACCEPTANCE_ANGLE;
        self.state.angle = a;
        assert!(self.state.angle < MAX);
        self.is_left()
    }
}

/// CircuitNode process particles, they can be filters or detectors
pub trait CircuitNode {
    fn receive_particle(&mut self, particle: &mut dyn Particle);
    fn get_string(&self, prefix: &String) -> String;
}

// Detectors are terminal nodes with no descendants
pub struct Detector {
    particle_counter: u32
}

impl Detector {

    pub fn new() -> Detector {
        Detector{particle_counter: 0}
    }

    pub fn reset(&mut self) {
        self.particle_counter = 0;
    }
}

impl CircuitNode for Detector {
    fn receive_particle(&mut self, _particle: &mut dyn Particle) {
        self.particle_counter = self.particle_counter + 1;
    }

    fn get_string(&self, _prefix: &String) -> String {
        format!("Detector: {} particles\n", self.particle_counter)
    }
}

// Filters send particles to either descenand_a or descenand_b
pub struct Filter<CNA: CircuitNode, CNB: CircuitNode> {
    f_type: FilterType,
    descenand_a: CNA,
    descenand_b: CNB
}

impl<CNA, CNB> Filter<CNA, CNB> where CNA: CircuitNode, CNB: CircuitNode {

    pub fn new(f_type: FilterType, descenand_a: CNA, descenand_b: CNB) -> Filter<CNA, CNB> {
        Filter{f_type, descenand_a, descenand_b}
    }
}

impl<CNA, CNB> CircuitNode for Filter<CNA, CNB> where CNA: CircuitNode, CNB: CircuitNode {

    fn receive_particle(&mut self, particle: &mut dyn Particle) {
        match self.f_type {
            FilterType::UpDown => {
                if particle.observe_updown() {
                    self.descenand_a.receive_particle(particle);
                } else {
                    self.descenand_b.receive_particle(particle);
                }
            },

            FilterType::LeftRight => {
                if particle.observe_leftright() {
                    self.descenand_a.receive_particle(particle);
                } else {
                    self.descenand_b.receive_particle(particle);
                }
            },
        }
    }

    fn get_string(&self, prefix: &String) -> String {
        let to_return;
        match self.f_type {
            FilterType::UpDown => {
                to_return = format!("|UpDown|--Up----{}{}       |--Down--{}",
                                    self.descenand_a.get_string(&format!("{}       |        ", prefix)),
                                    prefix,
                                    self.descenand_b.get_string(&format!("{}                ", prefix))
                );
            },
            FilterType::LeftRight => {
                to_return = format!("|LeftRight|--Left---{}{}          |--Right--{}",
                                    self.descenand_a.get_string(&format!("{}          |         ", prefix)),
                                    prefix,
                                    self.descenand_b.get_string(&format!("{}                    ", prefix))
                );
            }
        }
        to_return
    }
}

pub struct QCircuit<CN: CircuitNode> {
    initial_node: CN
}

impl<CN> QCircuit<CN> where CN: CircuitNode {

    pub fn new(initial_node: CN) -> QCircuit<CN> {
        QCircuit{initial_node}
    }

    pub fn run(&mut self, particles: u32) {
        for _ in 0..particles {
            let mut p = AngleParticle::new();
            self.initial_node.receive_particle(&mut p);
        }
    }

    pub fn print(&self) {
        print!("Source--{}\n", self.initial_node.get_string(&String::from("        ")));
    }
}
