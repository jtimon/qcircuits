//! The circuit module contains the structs and methods to create and run the circuits

use crate::hypotheses::ran_ang::AngleParticle;
use crate::particle::Particle;

pub enum FilterType {UpDown, LeftRight}

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

/// CircuitNode process particles, they can be filters or detectors
pub trait CircuitNode {
    fn receive_particle(&mut self, particle: &mut dyn Particle);
    fn get_string(&self, prefix: &String) -> String;
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
