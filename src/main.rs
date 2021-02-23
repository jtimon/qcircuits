use qcircuits::circuits::{
    QCircuit,
    Detector,
    Filter,
    FilterType,
};

fn main() {
    let mut c_detector_only = QCircuit::new(Detector::new());
    c_detector_only.run(1000);
    c_detector_only.print();

    let mut c_updown_series = QCircuit::new(
        Filter::new(FilterType::UpDown,
                    Filter::new(FilterType::UpDown,
                                Filter::new(FilterType::UpDown,
                                            Detector::new(),
                                            Detector::new()),
                                Detector::new()),
                    Detector::new()));
    c_updown_series.run(100000);
    c_updown_series.print();

    let mut c_tree2 = QCircuit::new(
        Filter::new(FilterType::LeftRight,
                    Filter::new(FilterType::UpDown,
                                Detector::new(),
                                Detector::new()),
                    Filter::new(FilterType::UpDown,
                                Detector::new(),
                                Detector::new())
        ));
    c_tree2.run(100000);
    c_tree2.print();

    let mut c_tree3 = QCircuit::new(
        Filter::new(FilterType::LeftRight,
                    Filter::new(FilterType::UpDown,
                                Filter::new(FilterType::LeftRight,
                                            Detector::new(),
                                            Detector::new()),
                                Filter::new(FilterType::LeftRight,
                                            Detector::new(),
                                            Detector::new())
                    ),
                    Filter::new(FilterType::UpDown,
                                Filter::new(FilterType::LeftRight,
                                            Detector::new(),
                                            Detector::new()),
                                Filter::new(FilterType::LeftRight,
                                            Detector::new(),
                                            Detector::new())
                    ),
        ));
    c_tree3.run(100000);
    c_tree3.print();

    let mut c_tree4 = QCircuit::new(
        Filter::new(FilterType::LeftRight,
                    Filter::new(FilterType::UpDown,
                                Filter::new(FilterType::LeftRight,
                                            Filter::new(FilterType::UpDown,
                                                        Detector::new(),
                                                        Detector::new()),
                                            Filter::new(FilterType::UpDown,
                                                        Detector::new(),
                                                        Detector::new())),
                                Filter::new(FilterType::LeftRight,
                                            Filter::new(FilterType::UpDown,
                                                        Detector::new(),
                                                        Detector::new()),
                                            Filter::new(FilterType::UpDown,
                                                        Detector::new(),
                                                        Detector::new()))
                    ),
                    Filter::new(FilterType::UpDown,
                                Filter::new(FilterType::LeftRight,
                                            Filter::new(FilterType::UpDown,
                                                        Detector::new(),
                                                        Detector::new()),
                                            Filter::new(FilterType::UpDown,
                                                        Detector::new(),
                                                        Detector::new())),
                                Filter::new(FilterType::LeftRight,
                                            Filter::new(FilterType::UpDown,
                                                        Detector::new(),
                                                        Detector::new()),
                                            Filter::new(FilterType::UpDown,
                                                        Detector::new(),
                                                        Detector::new()))
                    ),
        ));
    c_tree4.run(100000);
    c_tree4.print();
}
