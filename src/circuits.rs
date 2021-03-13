//! The circuit module contains the structs and methods to create and run the circuits

use std::sync::Arc;
use std::thread;

pub const BATCH_PARTICLE: usize = 10000;

#[derive(Copy, Clone)]
pub enum FilterType {UpDown, LeftRight}

/// A particle can be observed by a Filter or counted by a Detector.
#[derive(Clone)]
pub trait Particle : Send + Sync {
    // if up returns true, false otherwise
    fn observe_updown(&mut self) -> bool;
    // if left returns true, false otherwise
    fn observe_leftright(&mut self) -> bool;
}

/// A particle source can emit particles
pub trait ParticleSource : Copy + Clone + Send {
    fn get_particle(&mut self) -> Box<dyn Particle>;
}

/// Filters send particles to either descenand_a or descenand_b
// REM Deriving Copy is dangerous for Filter
// #[derive(Copy)]
pub struct Filter {
    f_type: FilterType,
    descenand_a: Option<Box<Filter>>,
    descenand_b: Option<Box<Filter>>,
    particle_counter_a: usize,
    particle_counter_b: usize
}

impl Clone for Filter {
    fn clone(&self) -> Filter {
        Filter {
            f_type: self.f_type,
            descenand_a: self.descenand_a.clone(),
            descenand_b: self.descenand_b.clone(),
            particle_counter_a: self.particle_counter_a,
            particle_counter_b: self.particle_counter_b,
        }
    }
}

impl Filter {

    pub fn new(f_type: FilterType, descenand_a: Option<Box<Filter>>, descenand_b: Option<Box<Filter>>) -> Filter {
        Filter{f_type, descenand_a, descenand_b, particle_counter_a: 0, particle_counter_b: 0}
    }

    pub fn get_results(&self) -> Vec<usize> {
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

    pub fn receive_particles_recu(&self, particles: Vec<Box<dyn Particle>>) -> Filter {
        let mut filter = self.clone();
        let mut observed_a : Vec<Box<dyn Particle>> = vec![];
        let mut observed_b : Vec<Box<dyn Particle>> = vec![];

        match filter.f_type {
            FilterType::UpDown => {
                for mut p in particles {
                    if p.observe_updown() {
                        observed_a.push(p);
                    } else {
                        observed_b.push(p);
                    }
                }
            },

            FilterType::LeftRight => {
                for mut p in particles {
                    if p.observe_leftright() {
                        observed_a.push(p);
                    } else {
                        observed_b.push(p);
                    }
                }
            }
        }

        let afa = Arc::new(filter.descenand_a.clone());
        let oba = Arc::new(&observed_a);
        let handle_a = thread::spawn(move || {
            if let &Some(ref x) = &*afa {
                Some(Box::new(x.receive_particles_recu(*oba)))
            } else {
                None
            }
        });

        let afb = Arc::new(filter.descenand_b.clone());
        let handle_b = thread::spawn(move || {
            if let &Some(ref x) = &*afb {
                Some(Box::new(x.receive_particles_recu(observed_b)))
            } else {
                None
            }
        });

        if filter.descenand_a.is_none() {
            filter.particle_counter_a += observed_a.len();
        }
        if filter.descenand_b.is_none() {
            filter.particle_counter_b += observed_b.len();
        }
        filter.descenand_a = handle_a.join().unwrap();
        filter.descenand_b = handle_b.join().unwrap();
        filter
    }

    fn receive_particles(&self, source: &mut (impl ParticleSource + 'static), n_particles: u32) -> Filter {
        let mut filter = self.clone();
        let mut n_particles = n_particles as usize;
        while n_particles > 0 {
            let iter = std::cmp::min(BATCH_PARTICLE, n_particles);
            n_particles -= iter;
            let mut particles : Vec<Box<dyn Particle>> = vec![];
            for _ in 0..iter {
                let p = source.get_particle();
                particles.push(p);
            }
            filter = filter.receive_particles_recu(particles);
        }
        filter
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

pub struct QCircuit {
    initial_node: Filter
}

impl QCircuit {

    pub fn new(initial_node: Filter) -> QCircuit {
        QCircuit{initial_node}
    }

    fn compare(&self,
               hypothesis_a: impl ParticleSource + 'static,
               hypothesis_b: impl ParticleSource + 'static,
               particles: u32) -> Vec<f32> {

        let filter_a = self.initial_node.clone();
        let handle_a = thread::spawn(move || {
            filter_a.receive_particles(&mut hypothesis_a.clone(), particles)
        });
        let filter_b = self.initial_node.clone();
        let handle_b = thread::spawn(move || {
            filter_b.receive_particles(&mut hypothesis_b.clone(), particles)
        });

        let filter_a = handle_a.join().unwrap();
        let results_a = filter_a.get_results();

        let filter_b = handle_b.join().unwrap();
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

    pub fn assert_compare(&self,
                          hypothesis_a: impl ParticleSource + 'static,
                          hypothesis_b: impl ParticleSource + 'static,
                          particles: u32, error: f32) {

        let percentage_difference = self.compare(hypothesis_a, hypothesis_b, particles);
        for perc_diff in percentage_difference {
            assert!(perc_diff < error);
        }
    }
}
