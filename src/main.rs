
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
    QCircuitFactory::series(1, FilterType::UpDown)
        .assert_compare(&particle_source_a, &particle_source_b, repetitions, error);

    QCircuitFactory::series(1, FilterType::LeftRight)
        .assert_compare(&particle_source_a, &particle_source_b, repetitions, error);

    QCircuitFactory::series(3, FilterType::UpDown)
        .assert_compare(&particle_source_a, &particle_source_b, repetitions, error);

    QCircuitFactory::series(3, FilterType::LeftRight)
        .assert_compare(&particle_source_a, &particle_source_b, repetitions, error);

    QCircuitFactory::tree(2, FilterType::UpDown)
        .assert_compare(&particle_source_a, &particle_source_b, repetitions, error);

    QCircuitFactory::tree(3, FilterType::LeftRight)
        .assert_compare(&particle_source_a, &particle_source_b, repetitions, error);

    QCircuitFactory::tree(4, FilterType::UpDown)
        .assert_compare(&particle_source_a, &particle_source_b, repetitions, error);

    QCircuitFactory::tree(5, FilterType::UpDown)
        .assert_compare(&particle_source_a, &particle_source_b, repetitions, error);

    QCircuitFactory::tree(6, FilterType::UpDown)
        .assert_compare(&particle_source_a, &particle_source_b, repetitions, error);
}

fn main() {

    // println!("Compare Enum and Angle hypotheses (both non deterministic hypothesis):\n");
    // compare_print_hypothesis(EnumParticleSource{}, AngleParticleSource{}, 100000);

    // println!("Compare deterministic Angle hypotheses with random and controlled source (both deterministic hypotheses):\n");
    // compare_print_hypothesis(DetAngleParticleSource{}, DetAngleParticleSourceDebug{}, MAX_ANGLE as u32 * MAX_ANGLE as u32);

    // println!("Compare deterministic Angle hypotheses with random and controlled source (both deterministic hypotheses):\n");
    // compare_print_hypothesis(DetTwoAngleParticleSource{}, DetTwoAngleParticleSourceDebug{}, MAX_ANGLE as u32 * MAX_ANGLE as u32);

    println!("Compare Enum and deterministic 6 bits hypotheses:\n");
    compare_print_hypothesis(EnumParticleSource{}, DetBitsParticleSource::new(6), 100000);
}
