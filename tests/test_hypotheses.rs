
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
    let mut c_updown_single = QCircuitFactory::series(1, FilterType::UpDown);
    c_updown_single.assert_compare(&mut EnumParticleSource{}, &mut AngleParticleSource{}, repetitions, error);

    let mut c_leftright_single = QCircuitFactory::series(1, FilterType::LeftRight);
    c_leftright_single.assert_compare(&mut EnumParticleSource{}, &mut AngleParticleSource{}, repetitions, error);
}

#[test]
fn test_enum_det_angle_single() {
    let repetitions = MAX_ANGLE as u32 * MAX_ANGLE as u32;
    let error = 0.7;
    let mut c_updown_single = QCircuitFactory::series(1, FilterType::UpDown);
    c_updown_single.assert_compare(&mut EnumParticleSource{}, &mut DetAngleParticleSourceDebug::new(), repetitions, error);

    let mut c_leftright_single = QCircuitFactory::series(1, FilterType::LeftRight);
    c_leftright_single.assert_compare(&mut EnumParticleSource{}, &mut DetAngleParticleSourceDebug::new(), repetitions, error);
}

#[test]
fn test_enum_angle_series() {
    let repetitions = 100000;
    let error = 0.5;
    let mut c_updown_series = QCircuitFactory::series(3, FilterType::UpDown);
    c_updown_series.assert_compare(&mut EnumParticleSource{}, &mut AngleParticleSource{}, repetitions, error);

    let mut c_leftright_series = QCircuitFactory::series(3, FilterType::LeftRight);
    c_leftright_series.assert_compare(&mut EnumParticleSource{}, &mut AngleParticleSource{}, repetitions, error);
}

#[test]
fn test_enum_det_angle_series() {
    let repetitions = MAX_ANGLE as u32 * MAX_ANGLE as u32;
    let error = 0.5;
    let mut c_updown_series = QCircuitFactory::series(3, FilterType::UpDown);
    c_updown_series.assert_compare(&mut EnumParticleSource{}, &mut DetAngleParticleSourceDebug::new(), repetitions, error);

    let mut c_leftright_series = QCircuitFactory::series(3, FilterType::LeftRight);
    c_leftright_series.assert_compare(&mut EnumParticleSource{}, &mut DetAngleParticleSourceDebug::new(), repetitions, error);
}

#[test]
fn test_enum_angle_tree2() {
    let repetitions = 100000;
    let error = 0.6;
    let mut c_tree_up = QCircuitFactory::tree(2, FilterType::UpDown);
    c_tree_up.assert_compare(&mut EnumParticleSource{}, &mut AngleParticleSource{}, repetitions, error);

    let mut c_tree_left = QCircuitFactory::tree(2, FilterType::LeftRight);
    c_tree_left.assert_compare(&mut EnumParticleSource{}, &mut AngleParticleSource{}, repetitions, error);
}

#[test]
fn test_enum_det_angle_tree2() {
    let repetitions = MAX_ANGLE as u32 * MAX_ANGLE as u32;
    let error = 0.6;
    let mut c_tree_up = QCircuitFactory::tree(2, FilterType::UpDown);
    c_tree_up.assert_compare(&mut EnumParticleSource{}, &mut DetAngleParticleSourceDebug::new(), repetitions, error);

    let mut c_tree_left = QCircuitFactory::tree(2, FilterType::LeftRight);
    c_tree_left.assert_compare(&mut EnumParticleSource{}, &mut DetAngleParticleSourceDebug::new(), repetitions, error);
}

#[test]
fn test_enum_angle_tree3() {
    let repetitions = 100000;
    let error = 0.5;
    let mut c_tree_up = QCircuitFactory::tree(3, FilterType::UpDown);
    c_tree_up.assert_compare(&mut EnumParticleSource{}, &mut AngleParticleSource{}, repetitions, error);

    let mut c_tree_left = QCircuitFactory::tree(3, FilterType::LeftRight);
    c_tree_left.assert_compare(&mut EnumParticleSource{}, &mut AngleParticleSource{}, repetitions, error);
}

#[test]
fn test_enum_det_angle_tree3() {
    let repetitions = MAX_ANGLE as u32 * MAX_ANGLE as u32;
    let error = 0.7;
    let mut c_tree_up = QCircuitFactory::tree(3, FilterType::UpDown);
    c_tree_up.assert_compare(&mut EnumParticleSource{}, &mut DetAngleParticleSourceDebug::new(), repetitions, error);

    let mut c_tree_left = QCircuitFactory::tree(3, FilterType::LeftRight);
    c_tree_left.assert_compare(&mut EnumParticleSource{}, &mut DetAngleParticleSourceDebug::new(), repetitions, error);
}

#[test]
fn test_enum_angle_tree4() {
    let repetitions = 100000;
    let error = 0.4;

    let mut c_tree_up = QCircuitFactory::tree(4, FilterType::UpDown);
    c_tree_up.assert_compare(&mut EnumParticleSource{}, &mut AngleParticleSource{}, repetitions, error);

    let mut c_tree_left = QCircuitFactory::tree(4, FilterType::LeftRight);
    c_tree_left.assert_compare(&mut EnumParticleSource{}, &mut AngleParticleSource{}, repetitions, error);
}

#[test]
fn test_enum_angle_tree5() {
    let repetitions = 100000;
    let error = 0.3;

    let mut c_tree_up = QCircuitFactory::tree(5, FilterType::UpDown);
    c_tree_up.assert_compare(&mut EnumParticleSource{}, &mut AngleParticleSource{}, repetitions, error);

    let mut c_tree_left = QCircuitFactory::tree(5, FilterType::LeftRight);
    c_tree_left.assert_compare(&mut EnumParticleSource{}, &mut AngleParticleSource{}, repetitions, error);
}

#[test]
fn test_enum_angle_tree10() {
    let repetitions = 100000;
    let error = 0.1;

    let mut c_tree_up = QCircuitFactory::tree(10, FilterType::UpDown);
    c_tree_up.assert_compare(&mut EnumParticleSource{}, &mut AngleParticleSource{}, repetitions, error);

    let mut c_tree_left = QCircuitFactory::tree(10, FilterType::LeftRight);
    c_tree_left.assert_compare(&mut EnumParticleSource{}, &mut AngleParticleSource{}, repetitions, error);
}

#[test]
fn test_enum_det_2angle_tree4() {
    let repetitions = MAX_ANGLE as u32 * MAX_ANGLE as u32;
    let error = 0.5;

    let mut c_tree_up = QCircuitFactory::tree(4, FilterType::UpDown);
    c_tree_up.assert_compare(&mut EnumParticleSource{}, &mut DetTwoAngleParticleSourceDebug::new(), repetitions, error);

    let mut c_tree_left = QCircuitFactory::tree(4, FilterType::LeftRight);
    c_tree_left.assert_compare(&mut EnumParticleSource{}, &mut DetTwoAngleParticleSourceDebug::new(), repetitions, error);
}

#[test]
fn test_enum_det_2angle_tree5() {
    let repetitions = MAX_ANGLE as u32 * MAX_ANGLE as u32;
    let error = 0.5;

    let mut c_tree_up = QCircuitFactory::tree(5, FilterType::UpDown);
    c_tree_up.assert_compare(&mut EnumParticleSource{}, &mut DetTwoAngleParticleSourceDebug::new(), repetitions, error);

    let mut c_tree_left = QCircuitFactory::tree(5, FilterType::LeftRight);
    c_tree_left.assert_compare(&mut EnumParticleSource{}, &mut DetTwoAngleParticleSourceDebug::new(), repetitions, error);
}

#[test]
fn test_enum_det_2angle_tree6() {
    let repetitions = MAX_ANGLE as u32 * MAX_ANGLE as u32;
    let error = 0.5;

    let mut c_tree_up = QCircuitFactory::tree(6, FilterType::UpDown);
    c_tree_up.assert_compare(&mut EnumParticleSource{}, &mut DetTwoAngleParticleSourceDebug::new(), repetitions, error);

    let mut c_tree_left = QCircuitFactory::tree(6, FilterType::LeftRight);
    c_tree_left.assert_compare(&mut EnumParticleSource{}, &mut DetTwoAngleParticleSourceDebug::new(), repetitions, error);
}

#[test]
fn test_enum_bits6_tree6() {
    let repetitions = 100000;
    let error = 0.2;

    let mut c_tree_up = QCircuitFactory::tree(6, FilterType::UpDown);
    c_tree_up.assert_compare(&mut EnumParticleSource{}, &mut DetBitsParticleSource::new(6), repetitions, error);

    let mut c_tree_left = QCircuitFactory::tree(6, FilterType::LeftRight);
    c_tree_left.assert_compare(&mut EnumParticleSource{}, &mut DetBitsParticleSource::new(6), repetitions, error);
}

#[test]
fn test_enum_bits10_tree10() {
    let repetitions = 100000;
    let error = 0.2;

    let mut c_tree_up = QCircuitFactory::tree(10, FilterType::UpDown);
    c_tree_up.assert_compare(&mut EnumParticleSource{}, &mut DetBitsParticleSource::new(10), repetitions, error);

    let mut c_tree_left = QCircuitFactory::tree(10, FilterType::LeftRight);
    c_tree_left.assert_compare(&mut EnumParticleSource{}, &mut DetBitsParticleSource::new(10), repetitions, error);
}

#[test]
fn test_enum_bits15_series() {
    let repetitions = 100000;
    let error = 0.7;
    let mut c_updown_series = QCircuitFactory::series(3, FilterType::UpDown);
    c_updown_series.assert_compare(&mut EnumParticleSource{}, &mut DetBitsParticleSource::new(15), repetitions, error);

    let mut c_leftright_series = QCircuitFactory::series(3, FilterType::LeftRight);
    c_leftright_series.assert_compare(&mut EnumParticleSource{}, &mut DetBitsParticleSource::new(15), repetitions, error);
}

#[test]
fn test_enum_bits15_tree15() {
    let repetitions = 100000;
    let error = 0.2;

    let mut c_tree_up = QCircuitFactory::tree(15, FilterType::UpDown);
    c_tree_up.assert_compare(&mut EnumParticleSource{}, &mut DetBitsParticleSource::new(15), repetitions, error);

    let mut c_tree_left = QCircuitFactory::tree(15, FilterType::LeftRight);
    c_tree_left.assert_compare(&mut EnumParticleSource{}, &mut DetBitsParticleSource::new(15), repetitions, error);
}
