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

    pub fn series(depth: u8, filtertype: FilterType) -> QCircuit {
        let mut filter = Filter::new(filtertype, None, None);
        assert!(depth > 0);
        let mut depth_count = depth - 1;
        while depth_count > 0 {
            filter = Filter::new(filtertype, Some(Box::new(filter)), None);
            depth_count -= 1;
        }
        QCircuit::new(filter)
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
