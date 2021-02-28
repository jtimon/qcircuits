
/// Compare different hypotheses

use qcircuits::circuits::{
    Filter,
    FilterType,
    QCircuit,
};

use qcircuits::sources::{
    AngleParticleSource,
    EnumParticleSource,
};

#[test]
fn test_enum_angle_single() {
    let repetitions = 100000;
    let error = 0.7;
    let mut c_updown_single = QCircuit::new(Filter::new(FilterType::UpDown, None, None));
    c_updown_single.assert_compare(&EnumParticleSource{}, &AngleParticleSource{}, repetitions, error);

    let mut c_leftright_single = QCircuit::new(Filter::new(FilterType::LeftRight, None, None));
    c_leftright_single.assert_compare(&EnumParticleSource{}, &AngleParticleSource{}, repetitions, error);
}

#[test]
fn test_enum_angle_series() {
    let repetitions = 100000;
    let error = 0.7;
    let mut c_updown_series = QCircuit::new(
        Filter::new(FilterType::UpDown,
                    Some(Box::new(Filter::new(FilterType::UpDown,
                                              Some(Box::new(Filter::new(FilterType::UpDown,
                                                                        None,
                                                                        None))),
                                              None))),
                    None));
    c_updown_series.assert_compare(&EnumParticleSource{}, &AngleParticleSource{}, repetitions, error);

    let mut c_leftright_series = QCircuit::new(
        Filter::new(FilterType::LeftRight,
                    Some(Box::new(Filter::new(FilterType::LeftRight,
                                              Some(Box::new(Filter::new(FilterType::LeftRight,
                                                                        None,
                                                                        None))),
                                              None))),
                    None));
    c_leftright_series.assert_compare(&EnumParticleSource{}, &AngleParticleSource{}, repetitions, error);
}

#[test]
fn test_enum_angle_tree2() {
    let repetitions = 100000;
    let error = 0.7;
    let mut c_tree2_up = QCircuit::new(
        Filter::new(FilterType::UpDown,
                    Some(Box::new(Filter::new(FilterType::LeftRight,
                                              None,
                                              None))),
                    Some(Box::new(Filter::new(FilterType::LeftRight,
                                              None,
                                              None)))
        ));
    c_tree2_up.assert_compare(&EnumParticleSource{}, &AngleParticleSource{}, repetitions, error);

    let mut c_tree2_left = QCircuit::new(
        Filter::new(FilterType::LeftRight,
                    Some(Box::new(Filter::new(FilterType::UpDown,
                                              None,
                                              None))),
                    Some(Box::new(Filter::new(FilterType::UpDown,
                                              None,
                                              None)))
        ));
    c_tree2_left.assert_compare(&EnumParticleSource{}, &AngleParticleSource{}, repetitions, error);
}

#[test]
fn test_enum_angle_tree3() {
    let repetitions = 100000;
    let error = 0.7;
    let mut c_tree3_up = QCircuit::new(
        Filter::new(FilterType::UpDown,
                    Some(Box::new(Filter::new(FilterType::LeftRight,
                                              Some(Box::new(Filter::new(FilterType::UpDown,
                                                                        None,
                                                                        None))),
                                              Some(Box::new(Filter::new(FilterType::UpDown,
                                                                        None,
                                                                        None)))
                    ))),
                    Some(Box::new(Filter::new(FilterType::LeftRight,
                                              Some(Box::new(Filter::new(FilterType::UpDown,
                                                                        None,
                                                                        None))),
                                              Some(Box::new(Filter::new(FilterType::UpDown,
                                                                        None,
                                                                        None)))
                    ))),
        ));
    c_tree3_up.assert_compare(&EnumParticleSource{}, &AngleParticleSource{}, repetitions, error);

    let mut c_tree3_left = QCircuit::new(
        Filter::new(FilterType::LeftRight,
                    Some(Box::new(Filter::new(FilterType::UpDown,
                                              Some(Box::new(Filter::new(FilterType::LeftRight,
                                                                        None,
                                                                        None))),
                                              Some(Box::new(Filter::new(FilterType::LeftRight,
                                                                        None,
                                                                        None)))
                    ))),
                    Some(Box::new(Filter::new(FilterType::UpDown,
                                              Some(Box::new(Filter::new(FilterType::LeftRight,
                                                                        None,
                                                                        None))),
                                              Some(Box::new(Filter::new(FilterType::LeftRight,
                                                                        None,
                                                                        None)))
                    ))),
        ));
    c_tree3_left.assert_compare(&EnumParticleSource{}, &AngleParticleSource{}, repetitions, error);
}

#[test]
fn test_enum_angle_tree4() {
    let repetitions = 100000;
    let error = 0.7;

    let mut c_tree4_up = QCircuit::new(
        Filter::new(FilterType::UpDown,
                    Some(Box::new(Filter::new(FilterType::LeftRight,
                                              Some(Box::new(Filter::new(FilterType::UpDown,
                                                                        Some(Box::new(Filter::new(FilterType::LeftRight,
                                                                                                  None,
                                                                                                  None))),
                                                                        Some(Box::new(Filter::new(FilterType::LeftRight,
                                                                                                  None,
                                                                                                  None)))))),
                                              Some(Box::new(Filter::new(FilterType::UpDown,
                                                                        Some(Box::new(Filter::new(FilterType::LeftRight,
                                                                                                  None,
                                                                                                  None))),
                                                                        Some(Box::new(Filter::new(FilterType::LeftRight,
                                                                                                  None,
                                                                                                  None))))))
                    ))),
                    Some(Box::new(Filter::new(FilterType::LeftRight,
                                              Some(Box::new(Filter::new(FilterType::UpDown,
                                                                        Some(Box::new(Filter::new(FilterType::LeftRight,
                                                                                                  None,
                                                                                                  None))),
                                                                        Some(Box::new(Filter::new(FilterType::LeftRight,
                                                                                                  None,
                                                                                                  None)))))),
                                              Some(Box::new(Filter::new(FilterType::UpDown,
                                                                        Some(Box::new(Filter::new(FilterType::LeftRight,
                                                                                                  None,
                                                                                                  None))),
                                                                        Some(Box::new(Filter::new(FilterType::LeftRight,
                                                                                                  None,
                                                                                                  None))))))
                    ))),
        ));
    c_tree4_up.assert_compare(&EnumParticleSource{}, &AngleParticleSource{}, repetitions, error);

    let mut c_tree4_left = QCircuit::new(
        Filter::new(FilterType::LeftRight,
                    Some(Box::new(Filter::new(FilterType::UpDown,
                                              Some(Box::new(Filter::new(FilterType::LeftRight,
                                                                        Some(Box::new(Filter::new(FilterType::UpDown,
                                                                                                  None,
                                                                                                  None))),
                                                                        Some(Box::new(Filter::new(FilterType::UpDown,
                                                                                                  None,
                                                                                                  None)))))),
                                              Some(Box::new(Filter::new(FilterType::LeftRight,
                                                                        Some(Box::new(Filter::new(FilterType::UpDown,
                                                                                                  None,
                                                                                                  None))),
                                                                        Some(Box::new(Filter::new(FilterType::UpDown,
                                                                                                  None,
                                                                                                  None))))))
                    ))),
                    Some(Box::new(Filter::new(FilterType::UpDown,
                                              Some(Box::new(Filter::new(FilterType::LeftRight,
                                                                        Some(Box::new(Filter::new(FilterType::UpDown,
                                                                                                  None,
                                                                                                  None))),
                                                                        Some(Box::new(Filter::new(FilterType::UpDown,
                                                                                                  None,
                                                                                                  None)))))),
                                              Some(Box::new(Filter::new(FilterType::LeftRight,
                                                                        Some(Box::new(Filter::new(FilterType::UpDown,
                                                                                                  None,
                                                                                                  None))),
                                                                        Some(Box::new(Filter::new(FilterType::UpDown,
                                                                                                  None,
                                                                                                  None))))))
                    ))),
        ));
    c_tree4_left.assert_compare(&EnumParticleSource{}, &AngleParticleSource{}, repetitions, error);
}
