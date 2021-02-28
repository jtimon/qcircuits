
/// Compare different hypotheses

use qcircuits::cfactory::QCircuitFactory;
use qcircuits::sources::{
    AngleParticleSource,
    EnumParticleSource,
};

#[test]
fn test_enum_angle_single() {
    let repetitions = 100000;
    let error = 0.7;
    let mut c_updown_single = QCircuitFactory::single_updown();
    c_updown_single.assert_compare(&EnumParticleSource{}, &AngleParticleSource{}, repetitions, error);

    let mut c_leftright_single = QCircuitFactory::single_leftright();
    c_leftright_single.assert_compare(&EnumParticleSource{}, &AngleParticleSource{}, repetitions, error);
}

#[test]
fn test_enum_angle_series() {
    let repetitions = 100000;
    let error = 0.7;
    let mut c_updown_series = QCircuitFactory::series_updown();
    c_updown_series.assert_compare(&EnumParticleSource{}, &AngleParticleSource{}, repetitions, error);

    let mut c_leftright_series = QCircuitFactory::series_leftright();
    c_leftright_series.assert_compare(&EnumParticleSource{}, &AngleParticleSource{}, repetitions, error);
}

#[test]
fn test_enum_angle_tree2() {
    let repetitions = 100000;
    let error = 0.7;
    let mut c_tree_up = QCircuitFactory::tree2_updown();
    c_tree_up.assert_compare(&EnumParticleSource{}, &AngleParticleSource{}, repetitions, error);

    let mut c_tree_left = QCircuitFactory::tree2_leftright();
    c_tree_left.assert_compare(&EnumParticleSource{}, &AngleParticleSource{}, repetitions, error);
}

#[test]
fn test_enum_angle_tree3() {
    let repetitions = 100000;
    let error = 0.7;
    let mut c_tree_up = QCircuitFactory::tree3_updown();
    c_tree_up.assert_compare(&EnumParticleSource{}, &AngleParticleSource{}, repetitions, error);

    let mut c_tree_left = QCircuitFactory::tree3_leftright();
    c_tree_left.assert_compare(&EnumParticleSource{}, &AngleParticleSource{}, repetitions, error);
}

#[test]
fn test_enum_angle_tree4() {
    let repetitions = 100000;
    let error = 0.7;

    let mut c_tree_up = QCircuitFactory::tree4_updown();
    c_tree_up.assert_compare(&EnumParticleSource{}, &AngleParticleSource{}, repetitions, error);

    let mut c_tree_left = QCircuitFactory::tree4_leftright();
    c_tree_left.assert_compare(&EnumParticleSource{}, &AngleParticleSource{}, repetitions, error);
}
