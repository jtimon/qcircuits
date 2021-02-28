use qcircuits::circuits::{
    Filter,
    FilterType,
    ParticleSource,
    QCircuit,
};

use qcircuits::sources::{
    AngleParticleSource,
    EnumParticleSource,
};

fn compare_print_hypothesis<PSA: ParticleSource, PSB: ParticleSource>(particle_source_a: PSA, particle_source_b: PSB, repetitions: u32) {

    let error = 0.7;
    let mut c_updown_single = QCircuit::new(Filter::new(FilterType::UpDown, None, None));
    c_updown_single.assert_compare(&particle_source_a, &particle_source_b, repetitions, error);

    let mut c_leftright_single = QCircuit::new(Filter::new(FilterType::LeftRight, None, None));
    c_leftright_single.assert_compare(&particle_source_a, &particle_source_b, repetitions, error);

    let mut c_updown_series = QCircuit::new(
        Filter::new(FilterType::UpDown,
                    Some(Box::new(Filter::new(FilterType::UpDown,
                                              Some(Box::new(Filter::new(FilterType::UpDown,
                                                                        None,
                                                                        None))),
                                              None))),
                    None));
    c_updown_series.assert_compare(&particle_source_a, &particle_source_b, repetitions, error);

    let mut c_leftright_series = QCircuit::new(
        Filter::new(FilterType::LeftRight,
                    Some(Box::new(Filter::new(FilterType::LeftRight,
                                              Some(Box::new(Filter::new(FilterType::LeftRight,
                                                                        None,
                                                                        None))),
                                              None))),
                    None));
    c_leftright_series.assert_compare(&particle_source_a, &particle_source_b, repetitions, error);

    let mut c_tree2 = QCircuit::new(
        Filter::new(FilterType::LeftRight,
                    Some(Box::new(Filter::new(FilterType::UpDown,
                                              None,
                                              None))),
                    Some(Box::new(Filter::new(FilterType::UpDown,
                                              None,
                                              None)))
        ));
    c_tree2.assert_compare(&particle_source_a, &particle_source_b, repetitions, error);

    let mut c_tree3 = QCircuit::new(
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
    c_tree3.assert_compare(&particle_source_a, &particle_source_b, repetitions, error);

    let mut c_tree4 = QCircuit::new(
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
