//! A QCircuit factory

use crate::circuits::{
    Filter,
    FilterType,
    QCircuit,
};

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

    pub fn tree(depth: u8, filtertype: FilterType) -> QCircuit {
        let mut filter = Filter::new(filtertype, None, None);
        assert!(depth > 0);
        let mut depth_count = depth - 1;
        let mut ft = filtertype;
        while depth_count > 0 {
            match ft {
                FilterType::UpDown => ft = FilterType::LeftRight,
                FilterType::LeftRight => ft = FilterType::UpDown
            }
            filter = Filter::new(ft, Some(Box::new(filter.clone())), Some(Box::new(filter)));
            depth_count -= 1;
        }
        QCircuit::new(filter)
    }
}
