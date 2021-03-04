
// use qcircuits::angle::MAX_ANGLE;
use qcircuits::cfactory::QCircuitFactory;
use qcircuits::circuits::{
    FilterType,
    ParticleSource,
};
use qcircuits::sources::{
    // AngleParticleSource,
    EnumParticleSource,
    // DetAngleParticleSource,
    // DetAngleParticleSourceDebug,
    DetBitsParticleSource,
    // DetTwoAngleParticleSource,
    // DetTwoAngleParticleSourceDebug,
};

fn compare_print_hypothesis<PSA: ParticleSource, PSB: ParticleSource>(particle_source_a: PSA, particle_source_b: PSB, repetitions: u32) {

    let error = 0.8;
    let mut c_updown_single = QCircuitFactory::series(1, FilterType::UpDown);
    c_updown_single.assert_compare(&particle_source_a, &particle_source_b, repetitions, error);

    let mut c_leftright_single = QCircuitFactory::series(1, FilterType::LeftRight);
    c_leftright_single.assert_compare(&particle_source_a, &particle_source_b, repetitions, error);

    let mut c_updown_series = QCircuitFactory::series(3, FilterType::UpDown);
    c_updown_series.assert_compare(&particle_source_a, &particle_source_b, repetitions, error);

    let mut c_leftright_series = QCircuitFactory::series(3, FilterType::LeftRight);
    c_leftright_series.assert_compare(&particle_source_a, &particle_source_b, repetitions, error);

    let mut c_tree2 = QCircuitFactory::tree(2, FilterType::UpDown);
    c_tree2.assert_compare(&particle_source_a, &particle_source_b, repetitions, error);

    let mut c_tree3 = QCircuitFactory::tree(3, FilterType::LeftRight);
    c_tree3.assert_compare(&particle_source_a, &particle_source_b, repetitions, error);

    let mut c_tree4 = QCircuitFactory::tree(4, FilterType::UpDown);
    c_tree4.assert_compare(&particle_source_a, &particle_source_b, repetitions, error);

    let mut c_tree5 = QCircuitFactory::tree(5, FilterType::UpDown);
    c_tree5.assert_compare(&particle_source_a, &particle_source_b, repetitions, error);

    let mut c_tree6 = QCircuitFactory::tree(6, FilterType::UpDown);
    c_tree6.assert_compare(&particle_source_a, &particle_source_b, repetitions, error);
}

fn main() {

    println!("\nContemplated hypotheses:\n");
    println!("- Enum hypothesis (simplest non deterministic hypothesis)");
    println!("- Random angles hypothesis (non deterministic hypothesis)");
    println!("");

    // println!("Compare Enum and Angle hypotheses (both non deterministic hypothesis):\n");
    // compare_print_hypothesis(EnumParticleSource{}, AngleParticleSource{}, 100000);

    // println!("Compare deterministic Angle hypotheses with random and controlled source (both deterministic hypotheses):\n");
    // compare_print_hypothesis(DetAngleParticleSource{}, DetAngleParticleSourceDebug{}, MAX_ANGLE as u32 * MAX_ANGLE as u32);

    // println!("Compare deterministic Angle hypotheses with random and controlled source (both deterministic hypotheses):\n");
    // compare_print_hypothesis(DetTwoAngleParticleSource{}, DetTwoAngleParticleSourceDebug{}, MAX_ANGLE as u32 * MAX_ANGLE as u32);

    println!("Compare Enum and deterministic 6 bits hypotheses:\n");
    compare_print_hypothesis(EnumParticleSource{}, DetBitsParticleSource::new(6), 100000);
}
