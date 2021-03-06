//! The circuit module contains the structs and methods to create and run the circuits

use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::thread;

#[derive(Copy, Clone)]
pub enum FilterType {UpDown, LeftRight}

pub const MAX_THREADS: u32 = 100;

/// Filters send particles to either descenand_a or descenand_b
pub struct Filter {
    f_type: FilterType,
    descenand_a: Option<Box<Filter>>,
    descenand_b: Option<Box<Filter>>,
    particle_counter_a: AtomicU32,
    particle_counter_b: AtomicU32
}

impl Clone for Filter {
    fn clone(&self) -> Filter {
        Filter {
            f_type: self.f_type,
            descenand_a: self.descenand_a.clone(),
            descenand_b: self.descenand_b.clone(),
            particle_counter_a: AtomicU32::new(self.particle_counter_a.load(Ordering::Relaxed)),
            particle_counter_b: AtomicU32::new(self.particle_counter_b.load(Ordering::Relaxed))
        }
    }
}

impl Filter {

    pub fn new(f_type: FilterType, descenand_a: Option<Box<Filter>>, descenand_b: Option<Box<Filter>>) -> Filter {
        Filter{f_type, descenand_a, descenand_b, particle_counter_a: AtomicU32::new(0), particle_counter_b: AtomicU32::new(0)}
    }

    pub fn reset_counters(&mut self) {
        self.particle_counter_a = AtomicU32::new(0);
        self.particle_counter_b = AtomicU32::new(0);
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
            None => vec.push(self.particle_counter_a.load(Ordering::Relaxed)),
        }
        match &self.descenand_b {
            Some(x) => vec.append(&mut x.get_results()),
            None => vec.push(self.particle_counter_b.load(Ordering::Relaxed)),
        }
        vec
    }

    fn get_descenand_a_string(&self, prefix: &String) -> String {
        let descenand_a_string;
        match &self.descenand_a {
            Some(x) => descenand_a_string = x.get_string(prefix),
            None => descenand_a_string = format!("Detector: {} particles\n", self.particle_counter_a.load(Ordering::Relaxed))
        }
        descenand_a_string
    }

    fn get_descenand_b_string(&self, prefix: &String) -> String {
        let descenand_b_string;
        match &self.descenand_b {
            Some(x) => descenand_b_string = x.get_string(prefix),
            None => descenand_b_string = format!("Detector: {} particles\n", self.particle_counter_b.load(Ordering::Relaxed))
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

/// A particle can be observed by a Filter or counted by a Detector.
pub trait Particle {
    // if up returns true, false otherwise
    fn observe_updown(&mut self) -> bool;
    // if left returns true, false otherwise
    fn observe_leftright(&mut self) -> bool;

    fn transfer_to_a(&mut self, filter: &Filter) {
        if let &Some(ref x) = &filter.descenand_a {
            self.pass_filter(x);
        } else {
            filter.particle_counter_a.fetch_add(1, Ordering::Relaxed);
        }
    }

    fn transfer_to_b(&mut self, filter: &Filter) {
        if let &Some(ref x) = &filter.descenand_b {
            self.pass_filter(x);
        } else {
            filter.particle_counter_b.fetch_add(1, Ordering::Relaxed);
        }
    }

    fn pass_filter(&mut self, filter: &Filter) {

        match filter.f_type {
            FilterType::UpDown => {
                if self.observe_updown() {
                    self.transfer_to_a(filter);
                } else {
                    self.transfer_to_b(filter);
                }
            },

            FilterType::LeftRight => {
                if self.observe_leftright() {
                    self.transfer_to_a(filter);
                } else {
                    self.transfer_to_b(filter);
                }
            },
        }
    }
}

/// A particle source can emit particles towards a CircuitNode (Filter or Detector)
pub trait ParticleSource {
    fn get_particle(&mut self) -> Box<dyn Particle + Send>;

    fn emit_particles(&mut self, filter: &Arc<Filter>, particles: u32) {
        let mut particles = particles;
        while particles > 0 {
            let iter = std::cmp::min(particles, MAX_THREADS);
            particles -= iter;

            let mut handles = vec![];
            for _ in 0..iter {
                let fc = Arc::clone(&filter);
                let mut p = self.get_particle();
                handles.push(thread::spawn(move || {
                    p.pass_filter(&*fc);
                }));
            }
            for handle in handles {
                handle.join().unwrap();
            }
        }
    }
}

pub struct QCircuit {
    initial_node: Arc<Filter>
}

impl QCircuit {

    pub fn new(initial_node: Filter) -> QCircuit {
        QCircuit{initial_node: Arc::new(initial_node)}
    }

    pub fn run(&mut self, particle_source: &mut impl ParticleSource, particles: u32) {
        particle_source.emit_particles(&self.initial_node, particles);
    }

    fn compare(&mut self, hypothesis_a: &mut impl ParticleSource, hypothesis_b: &mut impl ParticleSource, particles: u32, error: f32) -> bool {

        hypothesis_a.emit_particles(&mut self.initial_node, particles);
        let results_a = self.initial_node.get_results();
        Arc::get_mut(&mut self.initial_node).unwrap().reset_counters();

        hypothesis_b.emit_particles(&mut self.initial_node, particles);
        let results_b = self.initial_node.get_results();
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
        println!("Results a:             {:?}", results_a);
        println!("Results b:             {:?}", results_b);
        println!("Difference:            {:?}\n", difference);
        println!("percentage_a:          {:?}", percentage_a);
        println!("percentage_b:          {:?}", percentage_b);
        println!("Percentage difference: {:?}\n", percentage_difference);
        self.print();
        for perc_diff in percentage_difference {
            if perc_diff > error {
                return false
            }
        }
        true
    }

    pub fn assert_compare(&mut self, hypothesis_a: &mut impl ParticleSource, hypothesis_b: &mut impl ParticleSource, particles: u32, error: f32) {
        assert!(self.compare(hypothesis_a, hypothesis_b, particles, error));
    }

    pub fn print(&self) {
        print!("Source--{}\n", self.initial_node.get_string(&String::from("        ")));
    }
}
