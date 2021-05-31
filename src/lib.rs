//! A library for parsing TSPLIB file formats.
//!
//! The original [documentation](http://comopt.ifi.uni-heidelberg.de/software/TSPLIB95/) for TSPLIB
//! can be found in the website of Ruprecht-Karls-Universität Heidelberg.
mod error;
pub use error::ParseTspError;

pub mod metric;

mod tsp;
pub use tsp::CoordKind;
pub use tsp::DisplayKind;
pub use tsp::EdgeFormat;
pub use tsp::Point;
pub use tsp::Tsp;
pub use tsp::TspBuilder;
pub use tsp::TspKind;
pub use tsp::WeightFormat;
pub use tsp::WeightKind;

mod tests;
