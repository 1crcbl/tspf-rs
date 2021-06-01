# Tspf

[![Crates.io](https://img.shields.io/crates/v/tspf)](https://crates.io/crates/tspf) [![Documentation](https://docs.rs/tspf/badge.svg)](https://docs.rs/tspf) [![Build](https://github.com/1crcbl/tspf-rs/actions/workflows/main.yml/badge.svg)](https://github.com/1crcbl/tspf-rs/actions/workflows/main.yml)

Tspf is a small library for reading the TSPLIB file format. TSPLIB is a text-based file format often used in the research field of travelling salesman problem, vehicle routing problem and related problems. Some well-known TSP solvers (e.g. [Concorde](http://www.math.uwaterloo.ca/tsp/concorde/index.html) or [LKH](http://webhotel4.ruc.dk/~keld/research/LKH/)) work mainly with this file format as inputs. The department of Discrete and Combinatorial Optimization at Ruprecht-Karls-Universität Heidelberg maintains a detailed [documentation](http://comopt.ifi.uni-heidelberg.de/software/TSPLIB95/) for TSPLIB format.

## What's new?
The version ```0.2.0``` has the following new features:
- Parse HCP files
- Iterator for node coordinates, display coordinates, fixed edges and edge weights

## Status
At the moment I'm focusing on implementing an LKH solver (also in Rust). Thus, many features of the parser are still missing, but will be gradually added.

The library can currently parse the following problems from TSPLIB:  
- [X] TSP - symmetric travelling salesman problem  
- [X] HCP - Hamiltonian cycle problem  
- [X] ATSP - asymmetric travelling salesman problem  
- [ ] SOP - sequential ordering problem  
- [X] CVRP - capacitated vehicle routing problem  
- [ ] Tour - a collection of tours  

**NOTES**:
- The files ```si175.tsp```, ```si535.tsp```, ```si1032.tsp``` from the TSP dataset require a small change: the type entry in the second line ```TYPE: TSP (M.~Hofmeister)``` is wrong according to the format definition. Instead, that line should simply be ```TYPE: TSP```.
- For the HCP dataset, the file ```alb4000.hcp``` has a wrong entry in line ```8005```. The line should reads ```FIXED_EDGES_SECTION```, instead ```FIXED_EDGES```.

## Examples
To parse an input string, we use the function ```parse_str``` from the struct ```TspBuilder```:
```rust
use tspf;

let input_str = "
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

match tspf::TspBuilder::parse_str(input_str) {
    Ok(tsp) => {
        // Iterates over the node coordinates.
        for node in tsp.node_coords_itr() {
            println!("{:?}", node);
        }
    }
    Err(e) => {
        eprintln!("{}", e)
    }
}
```

On the other hand, the function ```parse_path``` handles the parsing from file.
```rust
use std::path::Path;
use tspf;

// The problem file can be downloaded from TSPLIB website.
// See: http://comopt.ifi.uni-heidelberg.de/software/TSPLIB95/tsp/
let path = Path::new("path/to/bays29.tsp");

match tspf::TspBuilder::parse_path(path) {
    Ok(tsp) => {
        // Iterates over the edge weights matrix.
        for v in tsp.edge_weights_itr() {
            println!("{:?}", v);
        }
    }
    Err(e) => {
        eprintln!("{}", e)
    }
}
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

