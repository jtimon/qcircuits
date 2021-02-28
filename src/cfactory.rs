//! A QCircuit factory

use crate::circuits::{
    Filter,
    FilterType,
    QCircuit,
};

fn opposite_filtertype(filtertype: &FilterType) -> FilterType {
    match filtertype {
        FilterType::UpDown => FilterType::LeftRight,
        FilterType::LeftRight => FilterType::UpDown,
    }
}

pub struct QCircuitFactory;

impl QCircuitFactory {

    pub fn single(filtertype: FilterType) -> QCircuit {
        QCircuit::new(Filter::new(filtertype, None, None))
    }

    pub fn series(filtertype: FilterType) -> QCircuit {
        QCircuit::new(
            Filter::new(filtertype,
                        Some(Box::new(Filter::new(filtertype,
                                                  Some(Box::new(Filter::new(filtertype,
                                                                            None,
                                                                            None))),
                                                  None))),
                        None))
    }

    // TODO generalize tree factory with depth argument
    pub fn tree2(filtertype: FilterType) -> QCircuit {
        let opposite_filtertype = opposite_filtertype(&filtertype);
        QCircuit::new(
            Filter::new(filtertype,
                        Some(Box::new(Filter::new(opposite_filtertype,
                                                  None,
                                                  None))),
                        Some(Box::new(Filter::new(opposite_filtertype,
                                                  None,
                                                  None)))
            ))
    }

    pub fn tree3(filtertype: FilterType) -> QCircuit {
        let opposite_filtertype = opposite_filtertype(&filtertype);
        QCircuit::new(
            Filter::new(filtertype,
                        Some(Box::new(Filter::new(opposite_filtertype,
                                                  Some(Box::new(Filter::new(filtertype,
                                                                            None,
                                                                            None))),
                                                  Some(Box::new(Filter::new(filtertype,
                                                                            None,
                                                                            None)))
                        ))),
                        Some(Box::new(Filter::new(opposite_filtertype,
                                                  Some(Box::new(Filter::new(filtertype,
                                                                            None,
                                                                            None))),
                                                  Some(Box::new(Filter::new(filtertype,
                                                                            None,
                                                                            None)))
                        ))),
            ))
    }

    pub fn tree4(filtertype: FilterType) -> QCircuit {
        let opposite_filtertype = opposite_filtertype(&filtertype);
        QCircuit::new(
            Filter::new(filtertype,
                        Some(Box::new(Filter::new(opposite_filtertype,
                                                  Some(Box::new(Filter::new(filtertype,
                                                                            Some(Box::new(Filter::new(opposite_filtertype,
                                                                                                      None,
                                                                                                      None))),
                                                                            Some(Box::new(Filter::new(opposite_filtertype,
                                                                                                      None,
                                                                                                      None)))))),
                                                  Some(Box::new(Filter::new(filtertype,
                                                                            Some(Box::new(Filter::new(opposite_filtertype,
                                                                                                      None,
                                                                                                      None))),
                                                                            Some(Box::new(Filter::new(opposite_filtertype,
                                                                                                      None,
                                                                                                      None))))))
                        ))),
                        Some(Box::new(Filter::new(opposite_filtertype,
                                                  Some(Box::new(Filter::new(filtertype,
                                                                            Some(Box::new(Filter::new(opposite_filtertype,
                                                                                                      None,
                                                                                                      None))),
                                                                            Some(Box::new(Filter::new(opposite_filtertype,
                                                                                                      None,
                                                                                                      None)))))),
                                                  Some(Box::new(Filter::new(filtertype,
                                                                            Some(Box::new(Filter::new(opposite_filtertype,
                                                                                                      None,
                                                                                                      None))),
                                                                            Some(Box::new(Filter::new(opposite_filtertype,
                                                                                                      None,
                                                                                                      None))))))
                        ))),
            ))
    }
}
