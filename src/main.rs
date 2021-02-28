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

fn test_print_hypothesis<PS: ParticleSource>(particle_source: PS, repetitions: u32) {
    let mut c_updown_single = QCircuit::new(
        Filter::new(FilterType::UpDown,
                    None,
                    None));
    c_updown_single.run(&particle_source, repetitions);
    c_updown_single.print();

    let mut c_leftright_single = QCircuit::new(
        Filter::new(FilterType::LeftRight,
                    None,
                    None));
    c_leftright_single.run(&particle_source, repetitions);
    c_leftright_single.print();

    let mut c_updown_series = QCircuit::new(
        Filter::new(FilterType::UpDown,
                    Some(Box::new(Filter::new(FilterType::UpDown,
                                              Some(Box::new(Filter::new(FilterType::UpDown,
                                                                        None,
                                                                        None))),
                                              None))),
                    None));
    c_updown_series.run(&particle_source, repetitions);
    c_updown_series.print();

    let mut c_leftright_series = QCircuit::new(
        Filter::new(FilterType::LeftRight,
                    Some(Box::new(Filter::new(FilterType::LeftRight,
                                              Some(Box::new(Filter::new(FilterType::LeftRight,
                                                                        None,
                                                                        None))),
                                              None))),
                    None));
    c_leftright_series.run(&particle_source, repetitions);
    c_leftright_series.print();

    let mut c_tree2 = QCircuit::new(
        Filter::new(FilterType::LeftRight,
                    Some(Box::new(Filter::new(FilterType::UpDown,
                                              None,
                                              None))),
                    Some(Box::new(Filter::new(FilterType::UpDown,
                                              None,
                                              None)))
        ));
    c_tree2.run(&particle_source, repetitions);
    c_tree2.print();

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
    c_tree3.run(&particle_source, repetitions);
    c_tree3.print();

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
    c_tree4.run(&particle_source, repetitions);
    c_tree4.print();
}

fn main() {
    println!("Enum hypothesis (simplest non deterministic hypothesis):\n");
    test_print_hypothesis(EnumParticleSource{}, 100000);

    println!("Random angles hypothesis (non deterministic hypothesis):\n");
    test_print_hypothesis(AngleParticleSource{}, 100000);
}
