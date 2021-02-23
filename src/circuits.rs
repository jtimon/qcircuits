//! The circuit module contains the structs and methods to create and run the circuits

use rand::Rng;

pub enum FilterType {UpDown, LeftRight}

struct Angle {
    angle: u16
}

impl Angle {

    pub fn random_angle() -> Angle {
        Angle {angle: rand::thread_rng().gen_range(0, 360)}
    }

    pub fn random_updown() -> Angle {
        let mut a = rand::thread_rng().gen_range(0, 180);
        if a >= 90 {
            a = a + 90;
        }
        Angle {angle: a}
    }

    pub fn random_leftright() -> Angle {
        let mut a = rand::thread_rng().gen_range(0, 180);
        if a >= 90 {
            a = a + 90;
        }
        a = a + 90;
        Angle {angle: a}
    }

    pub fn is_up(&self) -> bool {
        assert!(self.angle < 360);
        self.angle < 90
    }

    pub fn is_down(&self) -> bool {
        assert!(self.angle < 360);
        self.angle >= 180 && self.angle < 270
    }

    pub fn is_left(&self) -> bool {
        assert!(self.angle < 360);
        self.angle >= 270
    }

    pub fn is_right(&self) -> bool {
        assert!(self.angle < 360);
        self.angle >= 90 && self.angle < 180
    }
}

pub struct Particle {
    state: Angle
}

impl Particle {

    pub fn new() -> Particle {
        Particle {
            state: Angle::random_angle(),
        }
    }
}

/// CircuitNode process particles, they can be filters or detectors
pub trait CircuitNode {
    fn receive_particle(&mut self, particle: &mut Particle);
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
    fn receive_particle(&mut self, _particle: &mut Particle) {
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

    // Used when the current state cannot be used by the current filter
    fn random_state(&mut self, particle: &mut Particle) {
        match self.f_type {
            FilterType::UpDown => {
                // 50% chance of going either way
                particle.state = Angle::random_updown();
                if particle.state.is_up() {
                    self.descenand_a.receive_particle(particle);
                } else {
                    self.descenand_b.receive_particle(particle);
                }
            },

            FilterType::LeftRight => {
                // 50% chance of going either way
                particle.state = Angle::random_leftright();
                if !particle.state.is_left() {
                    self.descenand_a.receive_particle(particle);
                } else {
                    self.descenand_b.receive_particle(particle);
                }
            },
        }
    }
}

impl<CNA, CNB> CircuitNode for Filter<CNA, CNB> where CNA: CircuitNode, CNB: CircuitNode {

    fn receive_particle(&mut self, particle: &mut Particle) {
        match self.f_type {
            FilterType::UpDown => {
                if particle.state.is_up() {
                    self.descenand_a.receive_particle(particle);
                } else if particle.state.is_down() {
                    self.descenand_b.receive_particle(particle);
                } else {
                    self.random_state(particle);
                }
            },

            FilterType::LeftRight => {
                if particle.state.is_left() {
                    self.descenand_a.receive_particle(particle);
                } else if particle.state.is_right() {
                    self.descenand_b.receive_particle(particle);
                } else {
                    self.random_state(particle);
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
            let mut p = Particle::new();
            self.initial_node.receive_particle(&mut p);
        }
    }

    pub fn print(&self) {
        print!("Source--{}\n", self.initial_node.get_string(&String::from("        ")));
    }
}
