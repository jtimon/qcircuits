use qcircuits::circuits::{
    QCircuit,
    Filter,
    FilterType,
};

use qcircuits::sources::{
    AngleParticleSource,
};

fn main() {
    let angle_particle_source = AngleParticleSource{};
    let mut c_updown_single = QCircuit::new(
        Filter::new(FilterType::UpDown,
                    None,
                    None));
    c_updown_single.run(&angle_particle_source, 100000);
    c_updown_single.print();

    let mut c_leftright_single = QCircuit::new(
        Filter::new(FilterType::LeftRight,
                    None,
                    None));
    c_leftright_single.run(&angle_particle_source, 100000);
    c_leftright_single.print();

    let mut c_updown_series = QCircuit::new(
        Filter::new(FilterType::UpDown,
                    Some(Box::new(Filter::new(FilterType::UpDown,
                                              Some(Box::new(Filter::new(FilterType::UpDown,
                                                                        None,
                                                                        None))),
                                              None))),
                    None));
    c_updown_series.run(&angle_particle_source, 100000);
    c_updown_series.print();

    let mut c_leftright_series = QCircuit::new(
        Filter::new(FilterType::LeftRight,
                    Some(Box::new(Filter::new(FilterType::LeftRight,
                                              Some(Box::new(Filter::new(FilterType::LeftRight,
                                                                        None,
                                                                        None))),
                                              None))),
                    None));
    c_leftright_series.run(&angle_particle_source, 100000);
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
    c_tree2.run(&angle_particle_source, 100000);
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
    c_tree3.run(&angle_particle_source, 100000);
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
    c_tree4.run(&angle_particle_source, 100000);
    c_tree4.print();
}
