# Tspf

Tspf is a small library for reading the TSPLIB file format. TSPLIB is a text-based file format often used in the research field of travelling salesman problem, vehicle routing problem and related problems. Some well-known TSP solvers (e.g. [Concorde](http://www.math.uwaterloo.ca/tsp/concorde/index.html) or [LKH](http://webhotel4.ruc.dk/~keld/research/LKH/)) work mainly with this file format as inputs. The department of Discrete and Combinatorial Optimization at Ruprecht-Karls-Universität Heidelberg maintains a detailed [documentation](http://comopt.ifi.uni-heidelberg.de/software/TSPLIB95/) for TSPLIB format.

## Status
Currently I'm focusing on implementing an LKH solver (also in Rust). Thus, many features of the parser are still missing, but will be gradually added.

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

