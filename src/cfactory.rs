//! A QCircuit factory

use crate::circuits::{
    Filter,
    FilterType,
    QCircuit,
};

pub struct QCircuitFactory;

impl QCircuitFactory {

    // TODO generalize updown/leftright factory with FilterType argument
    pub fn single_updown() -> QCircuit {
        QCircuit::new(Filter::new(FilterType::UpDown, None, None))
    }

    pub fn single_leftright() -> QCircuit {
        QCircuit::new(Filter::new(FilterType::LeftRight, None, None))
    }

    pub fn series_updown() -> QCircuit {
        QCircuit::new(
            Filter::new(FilterType::UpDown,
                        Some(Box::new(Filter::new(FilterType::UpDown,
                                                  Some(Box::new(Filter::new(FilterType::UpDown,
                                                                            None,
                                                                            None))),
                                                  None))),
                        None))
    }

    pub fn series_leftright() -> QCircuit {
        QCircuit::new(
            Filter::new(FilterType::LeftRight,
                        Some(Box::new(Filter::new(FilterType::LeftRight,
                                                  Some(Box::new(Filter::new(FilterType::LeftRight,
                                                                            None,
                                                                            None))),
                                                  None))),
                        None))
    }

    // TODO generalize tree factory with depth argument
    pub fn tree2_updown() -> QCircuit {
        QCircuit::new(
            Filter::new(FilterType::UpDown,
                        Some(Box::new(Filter::new(FilterType::LeftRight,
                                                  None,
                                                  None))),
                        Some(Box::new(Filter::new(FilterType::LeftRight,
                                                  None,
                                                  None)))
            ))
    }

    pub fn tree2_leftright() -> QCircuit {
        QCircuit::new(
            Filter::new(FilterType::LeftRight,
                        Some(Box::new(Filter::new(FilterType::UpDown,
                                                  None,
                                                  None))),
                        Some(Box::new(Filter::new(FilterType::UpDown,
                                                  None,
                                                  None)))
            ))
    }

    pub fn tree3_updown() -> QCircuit {
        QCircuit::new(
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
            ))
    }

    pub fn tree3_leftright() -> QCircuit {
        QCircuit::new(
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
            ))
    }

    pub fn tree4_updown() -> QCircuit {
        QCircuit::new(
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
            ))
    }

    pub fn tree4_leftright() -> QCircuit {
        QCircuit::new(
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
            ))
    }
}
