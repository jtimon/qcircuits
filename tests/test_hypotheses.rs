
/// Compare different hypotheses

use qcircuits::angle::MAX_ANGLE;
use qcircuits::cfactory::QCircuitFactory;
use qcircuits::circuits::{FilterType, ParticleSource};
use qcircuits::sources::{
    AngleParticleSource,
    DetAngleParticleSourceDebug,
    DetBitsParticleSource,
    DetTwoAngleParticleSourceDebug,
    EnumParticleSource,
};

fn test_series(
    hypothesis_a: impl ParticleSource + 'static,
    hypothesis_b: impl ParticleSource + 'static,
    depth: u8, particles: u32, error: f32) {

    QCircuitFactory::series(depth, FilterType::UpDown)
        .assert_compare(hypothesis_a, hypothesis_b, particles, error);

    QCircuitFactory::series(depth, FilterType::LeftRight)
        .assert_compare(hypothesis_a, hypothesis_b, particles, error);
}

fn test_tree(
    hypothesis_a: impl ParticleSource + 'static,
    hypothesis_b: impl ParticleSource + 'static,
    depth: u8, particles: u32, error: f32) {

    QCircuitFactory::tree(depth, FilterType::UpDown)
        .assert_compare(hypothesis_a, hypothesis_b, particles, error);

    QCircuitFactory::tree(depth, FilterType::LeftRight)
        .assert_compare(hypothesis_a, hypothesis_b, particles, error);
}

#[test]
fn test_enum_angle_series1() {
    test_series(EnumParticleSource{}, AngleParticleSource{}, 1, 100000, 0.7);
}

#[test]
fn test_enum_det_angle_series1() {
    let repetitions = MAX_ANGLE as u32 * MAX_ANGLE as u32;
    test_series(EnumParticleSource{}, AngleParticleSource{}, 1, repetitions, 0.7);
}

#[test]
fn test_enum_angle_series3() {
    test_series(EnumParticleSource{}, AngleParticleSource{}, 3, 100000, 0.7);
}

#[test]
fn test_enum_det_angle_series3() {
    let repetitions = MAX_ANGLE as u32 * MAX_ANGLE as u32;
    test_series(EnumParticleSource{}, DetAngleParticleSourceDebug{}, 1, repetitions, 0.5);
}

#[test]
fn test_enum_det_2angle_series3() {
    let repetitions = MAX_ANGLE as u32 * MAX_ANGLE as u32;
    test_series(EnumParticleSource{}, DetTwoAngleParticleSourceDebug{}, 1, repetitions, 0.8);
}

#[test]
fn test_enum_bits15_series3() {
    test_series(EnumParticleSource{}, DetBitsParticleSource::new(15), 3, 100000, 0.7);
}

#[test]
fn test_enum_angle_tree2() {
    test_tree(EnumParticleSource{}, AngleParticleSource{}, 2, 100000, 0.7);
}

#[test]
fn test_enum_angle_tree3() {
    test_tree(EnumParticleSource{}, AngleParticleSource{}, 3, 100000, 0.7);
}

#[test]
fn test_enum_angle_tree4() {
    test_tree(EnumParticleSource{}, AngleParticleSource{}, 4, 100000, 0.7);
}

#[test]
fn test_enum_angle_tree5() {
    test_tree(EnumParticleSource{}, AngleParticleSource{}, 5, 100000, 0.7);
}

#[test]
fn test_enum_angle_tree10() {
    test_tree(EnumParticleSource{}, AngleParticleSource{}, 10, 100000, 0.7);
}

#[test]
fn test_enum_det_angle_tree2() {
    let repetitions = MAX_ANGLE as u32 * MAX_ANGLE as u32;
    test_tree(EnumParticleSource{}, DetAngleParticleSourceDebug{}, 2, repetitions, 0.6);
}

#[test]
fn test_enum_det_angle_tree3() {
    let repetitions = MAX_ANGLE as u32 * MAX_ANGLE as u32;
    test_tree(EnumParticleSource{}, DetAngleParticleSourceDebug{}, 3, repetitions, 0.7);
}

#[test]
fn test_enum_det_2angle_tree4() {
    let repetitions = MAX_ANGLE as u32 * MAX_ANGLE as u32;
    test_tree(EnumParticleSource{}, DetTwoAngleParticleSourceDebug{}, 4, repetitions, 0.5);
}

#[test]
fn test_enum_det_2angle_tree5() {
    let repetitions = MAX_ANGLE as u32 * MAX_ANGLE as u32;
    test_tree(EnumParticleSource{}, DetTwoAngleParticleSourceDebug{}, 5, repetitions, 0.5);
}

#[test]
fn test_enum_det_2angle_tree6() {
    let repetitions = MAX_ANGLE as u32 * MAX_ANGLE as u32;
    test_tree(EnumParticleSource{}, DetTwoAngleParticleSourceDebug{}, 6, repetitions, 0.5);
}

#[test]
fn test_enum_bits6_tree6() {
    test_tree(EnumParticleSource{}, DetBitsParticleSource::new(6), 6, 100000, 0.5);
}

#[test]
fn test_enum_bits10_tree10() {
    test_tree(EnumParticleSource{}, DetBitsParticleSource::new(10), 10, 100000, 0.5);
}

#[test]
fn test_enum_bits15_tree15() {
    test_tree(EnumParticleSource{}, DetBitsParticleSource::new(15), 15, 100000, 0.5);
}
