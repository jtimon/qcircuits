//! The circuit module contains the structs and methods to create and run the circuits

pub enum FilterType {UpDown, LeftRight}

/// A particle can be observed by a Filter or counted by a Detector.
pub trait Particle {
    // if up returns true, false otherwise
    fn observe_updown(&mut self) -> bool;
    // if left returns true, false otherwise
    fn observe_leftright(&mut self) -> bool;
}

/// Filters send particles to either descenand_a or descenand_b
pub struct Filter {
    f_type: FilterType,
    descenand_a: Option<Box<Filter>>,
    descenand_b: Option<Box<Filter>>,
    particle_counter_a: u32,
    particle_counter_b: u32
}

impl Filter {

    pub fn new(f_type: FilterType, descenand_a: Option<Box<Filter>>, descenand_b: Option<Box<Filter>>) -> Filter {
        Filter{f_type, descenand_a, descenand_b, particle_counter_a: 0, particle_counter_b: 0}
    }

    pub fn reset_counters(&mut self) {
        self.particle_counter_a = 0;
        self.particle_counter_b = 0;
        if let &mut Some(ref mut x) = &mut self.descenand_a {
            x.reset_counters();
        }
        if let &mut Some(ref mut x) = &mut self.descenand_b {
            x.reset_counters();
        }
    }

    pub fn get_results(&self) -> Vec<u32> {
        let mut vec = Vec::new();
        match &self.descenand_a {
            Some(x) => vec.append(&mut x.get_results()),
            None => vec.push(self.particle_counter_a),
        }
        match &self.descenand_b {
            Some(x) => vec.append(&mut x.get_results()),
            None => vec.push(self.particle_counter_b),
        }
        vec
    }

    fn transfer_to_a(&mut self, particle: &mut dyn Particle) {
        self.particle_counter_a += 1;
        if let &mut Some(ref mut x) = &mut self.descenand_a {
            x.receive_particle(particle);
        }
    }

    fn transfer_to_b(&mut self, particle: &mut dyn Particle) {
        self.particle_counter_b += 1;
        if let &mut Some(ref mut x) = &mut self.descenand_b {
            x.receive_particle(particle);
        }
    }

    pub fn receive_particle(&mut self, particle: &mut dyn Particle) {
        match self.f_type {
            FilterType::UpDown => {
                if particle.observe_updown() {
                    self.transfer_to_a(particle);
                } else {
                    self.transfer_to_b(particle);
                }
            },

            FilterType::LeftRight => {
                if particle.observe_leftright() {
                    self.transfer_to_a(particle);
                } else {
                    self.transfer_to_b(particle);
                }
            },
        }
    }

    fn get_descenand_a_string(&self, prefix: &String) -> String {
        let descenand_a_string;
        match &self.descenand_a {
            Some(x) => descenand_a_string = x.get_string(prefix),
            None => descenand_a_string = format!("Detector: {} particles\n", self.particle_counter_a)
        }
        descenand_a_string
    }

    fn get_descenand_b_string(&self, prefix: &String) -> String {
        let descenand_b_string;
        match &self.descenand_b {
            Some(x) => descenand_b_string = x.get_string(prefix),
            None => descenand_b_string = format!("Detector: {} particles\n", self.particle_counter_b)
        }
        descenand_b_string
    }

    fn get_string(&self, prefix: &String) -> String {
        let to_return;
        match self.f_type {
            FilterType::UpDown => {
                to_return = format!("|UpDown|--Up----{}{}       |--Down--{}",
                                    self.get_descenand_a_string(&format!("{}       |        ", prefix)),
                                    prefix,
                                    self.get_descenand_b_string(&format!("{}                ", prefix))
                );
            },
            FilterType::LeftRight => {
                to_return = format!("|LeftRight|--Left---{}{}          |--Right--{}",
                                    self.get_descenand_a_string(&format!("{}          |         ", prefix)),
                                    prefix,
                                    self.get_descenand_b_string(&format!("{}                    ", prefix))
                );
            }
        }
        to_return
    }
}

/// A particle source can emit particles towards a CircuitNode (Filter or Detector)
pub trait ParticleSource {
    fn emit_particles(&self, filter: &mut Filter, particles: u32);
}

pub struct QCircuit {
    initial_node: Filter
}

impl QCircuit {

    pub fn new(initial_node: Filter) -> QCircuit {
        QCircuit{initial_node}
    }

    pub fn run(&mut self, particle_source: &impl ParticleSource, particles: u32) {
        particle_source.emit_particles(&mut self.initial_node, particles);
    }

    pub fn print(&self) {
        println!("Results: {:?}", self.initial_node.get_results());
        print!("Source--{}\n", self.initial_node.get_string(&String::from("        ")));
    }
}
