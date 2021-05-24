#![allow(unused_imports)]
use std::path::Path;

use crate::{TspBuilder, TspKind, WeightKind};

// dantzig42 -> lower diag row

#[test]
fn test_read_str() {
    let s = "
    NAME: test
    TYPE: TSP
    COMMENT: Test
    DIMENSION: 3
    EDGE_WEIGHT_TYPE: GEO
    DISPLAY_DATA_TYPE: COORD_DISPLAY
    NODE_COORD_SECTION
    1 38.24 20.42
    2 39.57 26.15
    3 40.56 25.32
    EOF
    ";
    let result = TspBuilder::parse_str(s);
    assert!(result.is_ok());
    let tsp = result.unwrap();

    assert_eq!(3, tsp.dim());
    assert_eq!(TspKind::Tsp, tsp.kind());
    assert_eq!(WeightKind::Geo, tsp.weight_kind());
}
