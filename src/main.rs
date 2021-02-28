
use qcircuits::cfactory::QCircuitFactory;
use qcircuits::circuits::{
    ParticleSource,
};
use qcircuits::sources::{
    AngleParticleSource,
    EnumParticleSource,
};

fn compare_print_hypothesis<PSA: ParticleSource, PSB: ParticleSource>(particle_source_a: PSA, particle_source_b: PSB, repetitions: u32) {

    let error = 0.8;
    let mut c_updown_single = QCircuitFactory::single_updown();
    c_updown_single.assert_compare(&particle_source_a, &particle_source_b, repetitions, error);

    let mut c_leftright_single = QCircuitFactory::single_leftright();
    c_leftright_single.assert_compare(&particle_source_a, &particle_source_b, repetitions, error);

    let mut c_updown_series = QCircuitFactory::series_updown();
    c_updown_series.assert_compare(&particle_source_a, &particle_source_b, repetitions, error);

    let mut c_leftright_series = QCircuitFactory::series_leftright();
    c_leftright_series.assert_compare(&particle_source_a, &particle_source_b, repetitions, error);

    let mut c_tree2 = QCircuitFactory::tree2_updown();
    c_tree2.assert_compare(&particle_source_a, &particle_source_b, repetitions, error);

    let mut c_tree3 = QCircuitFactory::tree3_leftright();
    c_tree3.assert_compare(&particle_source_a, &particle_source_b, repetitions, error);

    let mut c_tree4 = QCircuitFactory::tree4_updown();
    c_tree4.assert_compare(&particle_source_a, &particle_source_b, repetitions, error);
}

fn main() {

    println!("\nContemplated hypotheses:\n");
    println!("- Enum hypothesis (simplest non deterministic hypothesis)");
    println!("- Random angles hypothesis (non deterministic hypothesis)");
    println!("");

    println!("Compare Enum and Angle hypotheses (both non deterministic hypothesis):\n");
    compare_print_hypothesis(EnumParticleSource{}, AngleParticleSource{}, 100000);
}
