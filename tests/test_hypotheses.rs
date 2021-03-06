
/// Compare different hypotheses

use qcircuits::angle::MAX_ANGLE;
use qcircuits::cfactory::QCircuitFactory;
use qcircuits::circuits::FilterType;
use qcircuits::sources::{
    AngleParticleSource,
    DetAngleParticleSourceDebug,
    DetBitsParticleSource,
    DetTwoAngleParticleSourceDebug,
    EnumParticleSource,
};

#[test]
fn test_enum_angle_single() {
    let repetitions = 100000;
    let error = 0.7;
    let hypothesis_a = EnumParticleSource{};
    let hypothesis_b = AngleParticleSource{};

    QCircuitFactory::series(1, FilterType::LeftRight)
        .assert_compare(hypothesis_a, hypothesis_b, repetitions, error);
}

#[test]
fn test_enum_det_angle_series1() {
    let repetitions = MAX_ANGLE as u32 * MAX_ANGLE as u32;
    let error = 0.7;
    let hypothesis_a = EnumParticleSource{};
    let hypothesis_b = DetAngleParticleSourceDebug{};

    QCircuitFactory::series(1, FilterType::UpDown)
        .assert_compare(hypothesis_a, hypothesis_b, repetitions, error);

    QCircuitFactory::series(1, FilterType::LeftRight)
        .assert_compare(hypothesis_a, hypothesis_b, repetitions, error);
}

#[test]
fn test_enum_angle_series3() {
    let repetitions = 100000;
    let error = 0.7;
    let hypothesis_a = EnumParticleSource{};
    let hypothesis_b = AngleParticleSource{};

    QCircuitFactory::series(3, FilterType::UpDown)
        .assert_compare(hypothesis_a, hypothesis_b, repetitions, error);

    QCircuitFactory::series(3, FilterType::LeftRight)
        .assert_compare(hypothesis_a, hypothesis_b, repetitions, error);
}

#[test]
fn test_enum_det_angle_series() {
    let repetitions = MAX_ANGLE as u32 * MAX_ANGLE as u32;
    let error = 0.5;
    let hypothesis_a = EnumParticleSource{};
    let hypothesis_b = DetAngleParticleSourceDebug{};

    QCircuitFactory::series(3, FilterType::UpDown)
        .assert_compare(hypothesis_a, hypothesis_b, repetitions, error);

    QCircuitFactory::series(3, FilterType::LeftRight)
        .assert_compare(hypothesis_a, hypothesis_b, repetitions, error);
}

#[test]
fn test_enum_angle_tree2() {
    let repetitions = 100000;
    let error = 0.7;
    let hypothesis_a = EnumParticleSource{};
    let hypothesis_b = AngleParticleSource{};

    QCircuitFactory::tree(2, FilterType::UpDown)
        .assert_compare(hypothesis_a, hypothesis_b, repetitions, error);

    QCircuitFactory::tree(2, FilterType::LeftRight)
        .assert_compare(hypothesis_a, hypothesis_b, repetitions, error);
}

#[test]
fn test_enum_det_angle_tree2() {
    let repetitions = MAX_ANGLE as u32 * MAX_ANGLE as u32;
    let error = 0.6;
    let hypothesis_a = EnumParticleSource{};
    let hypothesis_b = DetAngleParticleSourceDebug{};

    QCircuitFactory::tree(2, FilterType::UpDown)
        .assert_compare(hypothesis_a, hypothesis_b, repetitions, error);

    QCircuitFactory::tree(2, FilterType::LeftRight)
        .assert_compare(hypothesis_a, hypothesis_b, repetitions, error);
}

#[test]
fn test_enum_angle_tree3() {
    let repetitions = 100000;
    let error = 0.7;
    let hypothesis_a = EnumParticleSource{};
    let hypothesis_b = AngleParticleSource{};

    QCircuitFactory::tree(3, FilterType::UpDown)
        .assert_compare(hypothesis_a, hypothesis_b, repetitions, error);

    QCircuitFactory::tree(3, FilterType::LeftRight)
        .assert_compare(hypothesis_a, hypothesis_b, repetitions, error);
}

#[test]
fn test_enum_det_angle_tree3() {
    let repetitions = MAX_ANGLE as u32 * MAX_ANGLE as u32;
    let error = 0.7;
    let hypothesis_a = EnumParticleSource{};
    let hypothesis_b = DetAngleParticleSourceDebug{};

    QCircuitFactory::tree(3, FilterType::UpDown)
        .assert_compare(hypothesis_a, hypothesis_b, repetitions, error);

    QCircuitFactory::tree(3, FilterType::LeftRight)
        .assert_compare(hypothesis_a, hypothesis_b, repetitions, error);
}

#[test]
fn test_enum_angle_tree4() {
    let repetitions = 100000;
    let error = 0.7;
    let hypothesis_a = EnumParticleSource{};
    let hypothesis_b = AngleParticleSource{};

    QCircuitFactory::tree(4, FilterType::UpDown)
        .assert_compare(hypothesis_a, hypothesis_b, repetitions, error);

    QCircuitFactory::tree(4, FilterType::LeftRight)
        .assert_compare(hypothesis_a, hypothesis_b, repetitions, error);
}

#[test]
fn test_enum_angle_tree5() {
    let repetitions = 100000;
    let error = 0.7;
    let hypothesis_a = EnumParticleSource{};
    let hypothesis_b = AngleParticleSource{};

    QCircuitFactory::tree(5, FilterType::UpDown)
        .assert_compare(hypothesis_a, hypothesis_b, repetitions, error);

    QCircuitFactory::tree(5, FilterType::LeftRight)
        .assert_compare(hypothesis_a, hypothesis_b, repetitions, error);
}

#[test]
fn test_enum_angle_tree10() {
    let repetitions = 100000;
    let error = 0.7;
    let hypothesis_a = EnumParticleSource{};
    let hypothesis_b = AngleParticleSource{};

    QCircuitFactory::tree(10, FilterType::UpDown)
        .assert_compare(hypothesis_a, hypothesis_b, repetitions, error);

    QCircuitFactory::tree(10, FilterType::LeftRight)
        .assert_compare(hypothesis_a, hypothesis_b, repetitions, error);
}

#[test]
fn test_enum_det_2angle_tree4() {
    let repetitions = MAX_ANGLE as u32 * MAX_ANGLE as u32;
    let error = 0.5;
    let hypothesis_a = EnumParticleSource{};
    let hypothesis_b = DetTwoAngleParticleSourceDebug{};

    QCircuitFactory::tree(4, FilterType::UpDown)
        .assert_compare(hypothesis_a, hypothesis_b, repetitions, error);

    QCircuitFactory::tree(4, FilterType::LeftRight)
        .assert_compare(hypothesis_a, hypothesis_b, repetitions, error);
}

#[test]
fn test_enum_det_2angle_tree5() {
    let repetitions = MAX_ANGLE as u32 * MAX_ANGLE as u32;
    let error = 0.5;
    let hypothesis_a = EnumParticleSource{};
    let hypothesis_b = DetTwoAngleParticleSourceDebug{};

    QCircuitFactory::tree(5, FilterType::UpDown)
        .assert_compare(hypothesis_a, hypothesis_b, repetitions, error);

    QCircuitFactory::tree(5, FilterType::LeftRight)
        .assert_compare(hypothesis_a, hypothesis_b, repetitions, error);
}

#[test]
fn test_enum_det_2angle_tree6() {
    let repetitions = MAX_ANGLE as u32 * MAX_ANGLE as u32;
    let error = 0.5;
    let hypothesis_a = EnumParticleSource{};
    let hypothesis_b = DetTwoAngleParticleSourceDebug{};

    QCircuitFactory::tree(6, FilterType::UpDown)
        .assert_compare(hypothesis_a, hypothesis_b, repetitions, error);

    QCircuitFactory::tree(6, FilterType::LeftRight)
        .assert_compare(hypothesis_a, hypothesis_b, repetitions, error);
}

#[test]
fn test_enum_bits6_tree6() {
    let repetitions = 100000;
    let error = 0.5;
    let hypothesis_a = EnumParticleSource{};
    let hypothesis_b = DetBitsParticleSource::new(6);

    QCircuitFactory::tree(6, FilterType::UpDown)
        .assert_compare(hypothesis_a, hypothesis_b, repetitions, error);

    QCircuitFactory::tree(6, FilterType::LeftRight)
        .assert_compare(hypothesis_a, hypothesis_b, repetitions, error);
}

#[test]
fn test_enum_bits10_tree10() {
    let repetitions = 100000;
    let error = 0.2;
    let hypothesis_a = EnumParticleSource{};
    let hypothesis_b = DetBitsParticleSource::new(10);

    QCircuitFactory::tree(10, FilterType::UpDown)
        .assert_compare(hypothesis_a, hypothesis_b, repetitions, error);

    QCircuitFactory::tree(10, FilterType::LeftRight)
        .assert_compare(hypothesis_a, hypothesis_b, repetitions, error);
}

#[test]
fn test_enum_bits15_series() {
    let repetitions = 100000;
    let error = 0.7;
    let hypothesis_a = EnumParticleSource{};
    let hypothesis_b = DetBitsParticleSource::new(15);

    QCircuitFactory::series(3, FilterType::UpDown)
        .assert_compare(hypothesis_a, hypothesis_b, repetitions, error);

    QCircuitFactory::series(3, FilterType::LeftRight)
        .assert_compare(hypothesis_a, hypothesis_b, repetitions, error);
}

#[test]
fn test_enum_bits15_tree15() {
    let repetitions = 100000;
    let error = 0.2;
    let hypothesis_a = EnumParticleSource{};
    let hypothesis_b = DetBitsParticleSource::new(15);

    QCircuitFactory::tree(15, FilterType::UpDown)
        .assert_compare(hypothesis_a, hypothesis_b, repetitions, error);

    QCircuitFactory::tree(15, FilterType::LeftRight)
        .assert_compare(hypothesis_a, hypothesis_b, repetitions, error);
}
