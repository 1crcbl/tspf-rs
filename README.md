# Tspf

[![Crates.io](https://img.shields.io/crates/v/tspf)](https://crates.io/crates/tspf) [![Documentation](https://docs.rs/tspf/badge.svg)](https://docs.rs/tspf) [![Build](https://github.com/1crcbl/tspf-rs/actions/workflows/main.yml/badge.svg)](https://github.com/1crcbl/tspf-rs/actions/workflows/main.yml)

Tspf is a library for parsing the TSPLIB file format, a text-based file format often used in the research field of travelling salesman problem (TSP), vehicle routing problem and other related problems. Some well-known TSP solvers (e.g. [Concorde](http://www.math.uwaterloo.ca/tsp/concorde/index.html) or [LKH](http://webhotel4.ruc.dk/~keld/research/LKH/)) work mainly with this file format as inputs.

The parser is implemented based on the [documentation](http://comopt.ifi.uni-heidelberg.de/software/TSPLIB95/) from the Department of Discrete and Combinatorial Optimization at Ruprecht-Karls-Universität Heidelberg.

The library can fully parse all problem datasets hosted on TSPLIB's [website](http://comopt.ifi.uni-heidelberg.de/software/TSPLIB95/):
- [X] TSP - symmetric travelling salesman problem  
- [X] HCP - Hamiltonian cycle problem  
- [X] ATSP - asymmetric travelling salesman problem  
- [X] SOP - sequential ordering problem  
- [X] CVRP - capacitated vehicle routing problem  
- [X] Tour - a collection of tours 

Moreover, it also provides common metric functions to calculate edge weights (or cost/distance) between nodes in public interface. Among them are:
- [X] 2D- and 3D-Euclidean distance
- [X] 2D- and 3D-Manhattan distance
- [X] Geographical distance

This is a sister project from [cykl-rs](https://github.com/1crcbl/cykl-rs), a heuristic solver for TSP, which is still under-development.

## Usage
To parse an input string, we use the function ```parse_str``` from the struct ```TspBuilder```:
```rust
use tspf;

match TspBuilder::parse_str("some input string") {
    Ok(tsp) => {
        // tsp is an instance of struct Tsp.
        // From tsp, one can access all data.
    }
    Err(e) => eprint!("{:?}", e),
}
```

On the other hand, the function ```parse_path``` handles the parsing from files:
```rust
use std::path::Path;
use tspf;

let path = ;
match TspBuilder::parse_path(Path::new("path/to/some_file.tsp")) {
    Ok(tsp) => {
        // tsp is an instance of struct Tsp.
        // From tsp, one can access all data.
    }
    Err(e) => eprint!("{:?}", e),
}
``` 

**NOTES**:
- The files ```si175.tsp```, ```si535.tsp```, ```si1032.tsp``` from the TSP dataset require a small change: the type entry in the second line ```TYPE: TSP (M.~Hofmeister)``` is wrong according to the format definition. Instead, that line should simply be ```TYPE: TSP```.
- For the HCP dataset, the file ```alb4000.hcp``` has a wrong entry in line ```8005```. The line should reads ```FIXED_EDGES_SECTION```, instead ```FIXED_EDGES```.

## Datasets
The following list doesn't contain all test instance datasets used in academia. If you have any interesting datasets that are not included in the list, please let me know:

|Name |Links  | Note|
--- | --- | ---
|TSPLIB Travelling salesman problem | [[Website]](http://comopt.ifi.uni-heidelberg.de/software/TSPLIB95/tsp/)| Optimal solution: [[website]](http://comopt.ifi.uni-heidelberg.de/software/TSPLIB95/STSP.html)|
|TSPLIB Asymmetric travelling salesman problem | [[Website]](http://comopt.ifi.uni-heidelberg.de/software/TSPLIB95/atsp/)| Optimal solution: [[website]](http://comopt.ifi.uni-heidelberg.de/software/TSPLIB95/ATSP.html)|
|TSPLIB Hamiltonian cycle problem |[[Website]](http://comopt.ifi.uni-heidelberg.de/software/TSPLIB95/hcp/)||
|FHCP Hamiltonian cycle challenge set |[[Website]](https://sites.flinders.edu.au/flinders-hamiltonian-cycle-project/fhcp-challenge-set/)||
|TSPLIB Sequential ordering problem |[[Website]](http://comopt.ifi.uni-heidelberg.de/software/TSPLIB95/sop/)| Optimal solution: [[website]](http://comopt.ifi.uni-heidelberg.de/software/TSPLIB95/SOP.html)|
|TSPLIB Capacitated vehicle routing problem |[[Website]](http://comopt.ifi.uni-heidelberg.de/software/TSPLIB95/vrp/)||
|Shortest tour to nearly every pub in UK| [[Website]](http://www.math.uwaterloo.ca/tsp/uk/index.html) [[Download]](http://www.math.uwaterloo.ca/tsp/uk/files/uk49687_geom.tsp.txt) | Distance function not yet implemented |
|Orienteering problem | [[Github]](https://github.com/bcamath-ds/OPLib/tree/master/instances) | Parser not yet implemented |

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

