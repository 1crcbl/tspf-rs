#![allow(unused_imports, dead_code)]
use std::{ffi::OsStr, path::Path};

use crate::metric::*;
use crate::{TspBuilder, TspKind, WeightKind};

const TEST_STR_1: &str = "
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

#[test]
fn test_read_str() {
    let result = TspBuilder::parse_str(TEST_STR_1);
    assert!(result.is_ok());
    let tsp = result.unwrap();

    assert_eq!(3, tsp.dim());
    assert_eq!(TspKind::Tsp, tsp.kind());
    assert_eq!(WeightKind::Geo, tsp.weight_kind());
}

#[test]
fn test_read_str_missing_name() {
    let mut s = String::from("");
    for (idx, line) in TEST_STR_1.lines().enumerate() {
        if idx == 1 {
            continue;
        }
        s.push_str(line);
        s.push('\n');
    }

    let result = TspBuilder::parse_str(s);
    assert!(result.is_err());
}

#[test]
fn test_read_str_missing_type() {
    let mut s = String::from("");
    for (idx, line) in TEST_STR_1.lines().enumerate() {
        if idx == 2 {
            continue;
        }
        s.push_str(line);
        s.push('\n');
    }

    let result = TspBuilder::parse_str(s);
    assert!(result.is_err());
}

#[test]
fn test_read_str_missing_dim() {
    let mut s = String::from("");
    for (idx, line) in TEST_STR_1.lines().enumerate() {
        if idx == 4 {
            continue;
        }
        s.push_str(line);
        s.push('\n');
    }

    let result = TspBuilder::parse_str(s);
    assert!(result.is_err());
}

#[test]
fn test_read_str_missing_wtype() {
    let mut s = String::from("");
    for (idx, line) in TEST_STR_1.lines().enumerate() {
        if idx == 5 {
            continue;
        }
        s.push_str(line);
        s.push('\n');
    }

    let result = TspBuilder::parse_str(s);
    assert!(result.is_err());
}

#[test]
fn test_metric_fn() {
    assert_eq!(5., euc_2d(6., 0., 3., 4.), "Test euc_2d");
    assert_eq!(
        5. * (2 as f64).sqrt(),
        euc_3d(6., 0., -2., 3., 4., 3.),
        "Test euc_3d"
    );
    assert_eq!(7., man_2d(6., 0., 3., 4.), "Test man_2d");
    assert_eq!(12., man_3d(6., 0., -2., 3., 4., 3.), "Test man_3d");
    assert_eq!(4., max_2d(6., 0., 3., 4.), "Test max_2d");
    assert_eq!(5., max_3d(6., 0., -2., 3., 4., 3.), "Test max_3d");

    let eps = geo(89.6, -74.6, -29.6, -14.6) - 13359.864588;
    assert!(eps.abs() < 1e-6, "Test geo");
    // 13359.864588
    assert_eq!(
        18000.,
        xray1(360., 75., -55., 180., -45., 22.),
        "Test xray1"
    );
    assert_eq!(
        14400.,
        xray2(360., 75., -55., 180., -45., 22.),
        "Test xray2"
    );
}
