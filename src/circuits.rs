//! The circuit module contains the structs and methods to create and run the circuits

#[derive(Copy, Clone)]
pub enum FilterType {UpDown, LeftRight}

/// A particle can be observed by a Filter or counted by a Detector.
pub trait Particle {
    // if up returns true, false otherwise
    fn observe_updown(&mut self) -> bool;
    // if left returns true, false otherwise
    fn observe_leftright(&mut self) -> bool;
}

/// Filters send particles to either descenand_a or descenand_b
#[derive(Clone)]
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
        if let &mut Some(ref mut x) = &mut self.descenand_a {
            x.receive_particle(particle);
        } else {
            self.particle_counter_a += 1;
        }
    }

    fn transfer_to_b(&mut self, particle: &mut dyn Particle) {
        if let &mut Some(ref mut x) = &mut self.descenand_b {
            x.receive_particle(particle);
        } else {
            self.particle_counter_b += 1;
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

    pub fn print(&self) {
        print!("Source--{}\n", self.get_string(&String::from("        ")));
    }
}

/// A particle source can emit particles towards a CircuitNode (Filter or Detector)
pub trait ParticleSource {
    fn emit_particles(&self, filter: &Filter, particles: u32) -> Filter;
}

pub struct QCircuit {
    initial_node: Filter
}

impl QCircuit {

    pub fn new(initial_node: Filter) -> QCircuit {
        QCircuit{initial_node}
    }

    fn compare(&mut self, hypothesis_a: &impl ParticleSource, hypothesis_b: &impl ParticleSource, particles: u32) -> Vec<f32> {

        let filter_a = hypothesis_a.emit_particles(&self.initial_node, particles);
        let results_a = filter_a.get_results();

        let filter_b = hypothesis_b.emit_particles(&self.initial_node, particles);
        let results_b = filter_b.get_results();

        let mut difference = Vec::new();
        let mut percentage_a = Vec::new();
        let mut percentage_b = Vec::new();
        let mut percentage_difference = Vec::new();
        assert!(results_a.len() == results_b.len());
        for i in 0..results_a.len() {
            percentage_a.push((results_a[i] as f32) * 100.0 / particles as f32);
            percentage_b.push((results_b[i] as f32) * 100.0 / particles as f32);
            if results_a[i] > results_b[i] {
                difference.push(results_a[i] - results_b[i]);
                percentage_difference.push(percentage_a[i] - percentage_b[i]);
            } else {
                difference.push(results_b[i] - results_a[i]);
                percentage_difference.push(percentage_b[i] - percentage_a[i]);
            }
        }

        filter_b.print();
        println!("Results a:             {:?}", results_a);
        println!("Results b:             {:?}", results_b);
        println!("percentage_a:          {:?}", percentage_a);
        println!("percentage_b:          {:?}", percentage_b);
        println!("Difference:            {:?}", difference);
        println!("Percentage difference: {:?}\n", percentage_difference);
        percentage_difference
    }

    pub fn assert_compare(&mut self, hypothesis_a: &impl ParticleSource, hypothesis_b: &impl ParticleSource, particles: u32, error: f32) {
        let percentage_difference = self.compare(hypothesis_a, hypothesis_b, particles);
        for perc_diff in percentage_difference {
            assert!(perc_diff < error);
        }
    }
}
