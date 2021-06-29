use std::path::Path;

use tspf::{TspBuilder, TspKind, WeightKind, metric::MetricPoint};

#[test]
fn parse_vrp() {
    let tsp = TspBuilder::parse_path(Path::new("./tests/data/eil22.vrp")).unwrap();
    assert_eq!(TspKind::Cvrp, tsp.kind());
    assert_eq!(22, tsp.dim());
    assert_eq!(WeightKind::Euc2d, tsp.weight_kind());
    assert_eq!(6000, tsp.capacity());
    assert_eq!(1, tsp.depots().len());
    assert_eq!(1, *tsp.depots().first().unwrap());
    let pt = tsp.node_coords().get(&21).unwrap();
    assert_eq!(155_f64, pt.x());
    assert_eq!(185_f64, pt.y());
    assert_eq!(0_f64, pt.z());
    assert_eq!(900_f64, *tsp.demands().get(&16).unwrap());
}

#[test]
fn parse_tsp() {
    let tsp = TspBuilder::parse_path(Path::new("./tests/data/berlin52.tsp")).unwrap();
    assert_eq!(TspKind::Tsp, tsp.kind());
    assert_eq!(52, tsp.dim());
    assert_eq!(WeightKind::Euc2d, tsp.weight_kind());
    assert_eq!(0, tsp.depots().len());
    let pt = tsp.node_coords().get(&52).unwrap();
    assert_eq!(1740_f64, pt.x());
    assert_eq!(245_f64, pt.y());
    assert_eq!(0_f64, pt.z());
}