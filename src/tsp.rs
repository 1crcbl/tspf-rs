use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use crate::error::ParseTspError;

// (Some) keywords for data specification part.
static K_NAME: &str = "NAME";
static K_TYPE: &str = "TYPE";
static K_DIM: &str = "DIMENSION";
static K_WEIGHT_TYPE: &str = "EDGE_WEIGHT_TYPE";
static K_WEIGHT_FORMAT: &str = "EDGE_WEIGHT_FORMAT";

// (Some) keywords for the data part.
static K_NODE_COORD_SEC: &str = "NODE_COORD_SECTION";
static K_EDGE_WEIGHT_SEC: &str = "EDGE_WEIGHT_SECTION";

/// Macro for implementing trait Display for Enums.
macro_rules! impl_disp_enum {
    ($enm:ident) => {
        impl std::fmt::Display for $enm {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
    };
}

/// Represents a parsed TSP dataset.
///
/// An instance of this struct can only be created through the use of [`TspBuilder`].
///
/// TSP format has two parts:
/// - *Specification part*: contains metadata and general information about the dataset.
/// - *Data part*: contains all data.
///
/// # Format
/// The *specification part* has the following entries:
/// - ```NAME``` (required): the name of a dataset.
/// - ```TYPE``` (required): the type specifier of a dataset. Represented by the enum [`TspKind`].
/// - ```COMMENT``` (optional): comments of a dataset.
/// - ```DIM``` (required): the dimension of a dataset.
/// - ```CAPACITY``` (required if ```TYPE``` is [`TspKind::Cvrp`]): the truck capacity in Capacitated
/// Vehicle Routing Problem (CVRP).
/// - ```EDGE_WEIGHT_TYPE``` (required): indicates how the edge weights (or distances) are calculated.
/// Represented by the enum [`WeightKind`].
/// - ```EDGE_WEIGHT_FORMAT``` (required if ```EDGE_WEIGHT_TYPE``` is [`WeightKind::Explicit`]):
/// specifies how the edge weights are given in the file. Represented by the enum [`WeightFormat`].
/// - ```EDGE_DATA_FORMAT``` (optional): specifies how the edges of a graph are given in the file,
/// if the graph is not complete. Represented by the enum [`EdgeFormat`].
/// - ```NODE_COORD_TYPE``` (required if ```EDGE_WEIGHT_TYPE``` is not [`WeightKind::Explicit`]):
/// specifies how the coordinate for each node is given in the file. Represented by the enum [`CoordKind`].
/// - ```DISPLAY_DATA_TYPE``` (optional): spcifies how the coordinate for each node for display
/// purpose is given in the file. Represented by the enum [`DisplayKind`].
///
/// The *data part* has the following entries:
/// - ```NODE_COORD_SECTION```: 
/// # Example
/// 
/// The following example shows how to parse a TSP data from string with [`TspBuilder::parse_str`]:
///
/// ```
/// let s = "
/// NAME: test
/// TYPE: TSP
/// COMMENT: Test
/// DIMENSION: 3
/// EDGE_WEIGHT_TYPE: GEO
/// DISPLAY_DATA_TYPE: COORD_DISPLAY
/// NODE_COORD_SECTION
/// 1 38.24 20.42
/// 2 39.57 26.15
/// 3 40.56 25.32
/// EOF
/// ";
/// let result = TspBuilder::parse_str(s);
/// assert!(result.is_ok());
/// let _ = result.unwrap();
/// ```
///
/// We can also parse a file by calling the function [`TspBuilder::parse_path`]:
///
/// ```
/// let path = Path::new("./test.tsp");
/// let result = TspBuilder::parse_path(path);
/// assert!(result.is_ok());
/// ```
#[derive(Debug)]
pub struct Tsp {
    /// Name of the dataset. Maps to the entry ```NAME``` in a TSP file.
    name: String,
    /// Type specifier of the datset. Maps to the entry ```TYPE``` in a TSP file.
    kind: TspKind,
    /// Additional comments.
    comment: Option<String>,
    /// The dimension of a dataset.
    dim: usize,
    /// Maps to the entry ```EDGE_WEIGHT_TYPE``` in a TSP file.
    weight_kind: WeightKind,
    /// Maps to the entry ```EDGE_WEIGHT_FORMAT``` in a TSP file.
    weight_format: WeightFormat,
    /// Maps to the entry ```EDGE_DATA_FORMAT``` in a TSP file.
    edge_format: EdgeFormat,
    /// Maps to the entry ```NODE_COORD_TYPE``` in a TSP file.
    coord_kind: CoordKind,
    /// Maps to the entry ```DISPLAY_DATA_TYPE``` in a TSP file.
    disp_kind: DisplayKind,

    node_coords: Option<Vec<Point>>,

    edge_weights: Option<Vec<Vec<f64>>>,

    disp_coords: Option<Vec<Point>>,

    fixed_edges: Option<Vec<(usize, usize)>>,
}

impl Tsp {
    /// Returns the name of the dataset.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Returns the problem variant of the dataset.
    pub fn kind(&self) -> TspKind {
        self.kind
    }

    /// Returns comments of the dataset.
    pub fn comment(&self) -> Option<&String> {
        self.comment.as_ref()
    }

    /// Returns the dimension of the dataset.
    pub fn dim(&self) -> usize {
        self.dim
    }

    /// Return the enum indicating how the weights calculated or given.
    pub fn weight_kind(&self) -> WeightKind {
        self.weight_kind
    }

    /// Returns the enum indicating how the weights are stored in a file.
    pub fn weight_format(&self) -> WeightFormat {
        self.weight_format
    }

    /// Returns the enum indicating how node coordinates are stored in a file.
    pub fn coord_kind(&self) -> CoordKind {
        self.coord_kind
    }

    /// Returns the enum indicating how node coordinates for display purpose are stored in a file.
    pub fn disp_kind(&self) -> DisplayKind {
        self.disp_kind
    }


}

/// Responsible for constructing an instance of [`Tsp`].
#[derive(Debug, Default)]
pub struct TspBuilder {
    // Spec
    name: Option<String>,
    kind: Option<TspKind>,
    comment: Option<String>,
    dim: Option<usize>,
    weight_kind: Option<WeightKind>,
    weight_format: Option<WeightFormat>,
    edge_format: Option<EdgeFormat>,
    coord_kind: Option<CoordKind>,
    disp_kind: Option<DisplayKind>,
    // Data
    coords: Option<Vec<Point>>,
    edge_weights: Option<Vec<Vec<f64>>>,
    disp_coords: Option<Vec<Point>>,
    fixed_edges: Option<Vec<(usize, usize)>>,
}

impl TspBuilder {
    pub fn new() -> Self {
        TspBuilder {
            ..Default::default()
        }
    }

    /// Parses an input string.
    ///
    /// If all entries in the input string are valid, a [`Tsp`] object will be returned. Otherwise,
    /// an error [`ParseTspError`] is returned, containing hints why the parsing fails.
    // Should be in TryFrom once issue 50133 is fixed.
    // See: https://github.com/rust-lang/rust/issues/50133.
    pub fn parse_str<S>(s: S) -> Result<Tsp, ParseTspError>
    where
        S: AsRef<str>,
    {
        let mut itr = s.as_ref().lines().map(|l| l);
        Self::parse_it(&mut itr)
    }

    /// Parses the content of a file given from a path.
    ///
    /// If all entries in the input file are valid, a [`Tsp`] object will be returned. Otherwise,
    /// an error [`ParseTspError`] is returned, containing hints why the parsing fails.
    // Should be in TryFrom once issue 50133 is fixed.
    // See: https://github.com/rust-lang/rust/issues/50133.
    pub fn parse_path<P>(path: P) -> Result<Tsp, ParseTspError>
    where
        P: AsRef<Path>,
    {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut lines_it = reader.lines().map(|l| l.unwrap());
        Self::parse_it(&mut lines_it)
    }

    fn parse_it<I>(itr: &mut I) -> Result<Tsp, ParseTspError>
    where
        I: Iterator,
        <I as Iterator>::Item: AsRef<str>,
    {
        let splitter = |s: &str| {
            let val = s.split(":").collect::<Vec<&str>>();
            String::from(val[1].trim())
        };

        let mut builder = TspBuilder::new();

        while let Some(line) = itr.next() {
            let line = line.as_ref().trim();
            if line.len() == 0 {
                continue;
            }
            if line.starts_with("EOF") {
                break;
            }

            if line.starts_with(K_NAME) {
                builder.name = Some(splitter(&line));
            } else if line.starts_with(K_TYPE) {
                builder.kind = Some(TspKind::from(splitter(&line).as_str()));
            } else if line.starts_with("COMMENT") {
                // TODO: multiple-line comments?
                builder.comment = Some(splitter(&line));
            } else if line.starts_with(K_DIM) {
                builder.dim = Some(splitter(&line).parse::<usize>().unwrap());
            } else if line.starts_with("CAPACITY") {
                todo!()
            } else if line.starts_with(K_WEIGHT_TYPE) {
                let kind = WeightKind::from(splitter(&line).as_str());
                builder.weight_kind = Some(kind.clone());
                builder.coord_kind = Some(CoordKind::from(kind));
            } else if line.starts_with(K_WEIGHT_FORMAT) {
                builder.weight_format = Some(WeightFormat::from(splitter(&line).as_str()));
            } else if line.starts_with("EDGE_DATA_FORMAT") {
                builder.edge_format = Some(EdgeFormat::from(splitter(&line).as_str()));
            } else if line.starts_with("NODE_COORD_TYPE") {
                builder.coord_kind = Some(CoordKind::from(splitter(&line).as_str()));
            } else if line.starts_with("DISPLAY_DATA_TYPE") {
                builder.disp_kind = Some(DisplayKind::from(splitter(&line).as_str()));
            } else if line.starts_with(K_NODE_COORD_SEC) {
                builder.parse_node_coord_section(itr)?;
            } else if line.starts_with("DEPOT_SECTION") {
                todo!()
            } else if line.starts_with("DEMAND_SECTION") {
                todo!()
            } else if line.starts_with("EDGE_DATA_SECTION") {
                todo!()
            } else if line.starts_with("FIXED_EDGES_SECTION") {
                builder.parse_fixed_edge_section(itr)?;
            } else if line.starts_with("DISPLAY_DATA_SECTION") {
                builder.parse_display_data_section(itr)?;
            } else if line.starts_with("TOUR_SECTION") {
                todo!()
            } else if line.starts_with(K_EDGE_WEIGHT_SEC) {
                builder.parse_edge_weight_section(itr)?;
            } else {
                return Err(ParseTspError::InvalidEntry(String::from(line)));
            }
        }

        builder.build()
    }

    /// Parse the block `NODE_COORD_SECTION`.
    fn parse_node_coord_section<I>(&mut self, lines_it: &mut I) -> Result<(), ParseTspError>
    where
        I: Iterator,
        <I as Iterator>::Item: AsRef<str>,
    {
        self.validate_spec()?;

        let mut func: Box<dyn FnMut(&Vec<&str>, &mut Vec<Point>)> = match &self.coord_kind.unwrap()
        {
            CoordKind::Coo2d => {
                let f = |v: &Vec<&str>, r: &mut Vec<Point>| {
                    let p = Point::from((
                        v[0].parse::<usize>().unwrap(),
                        v[1].parse::<f64>().unwrap(),
                        v[2].parse::<f64>().unwrap(),
                    ));

                    r.push(p);
                };
                Box::new(f)
            }
            CoordKind::Coo3d => {
                let f = |v: &Vec<&str>, r: &mut Vec<Point>| {
                    let p = Point::from((
                        v[0].parse::<usize>().unwrap(),
                        v[1].parse::<f64>().unwrap(),
                        v[2].parse::<f64>().unwrap(),
                        v[3].parse::<f64>().unwrap(),
                    ));

                    r.push(p);
                };
                Box::new(f)
            }
            CoordKind::NoCoo | CoordKind::Undefined => {
                unimplemented!()
            }
        };

        let mut count = 0;
        let dim = self.dim.unwrap();
        let mut dta = Vec::with_capacity(dim);

        while count < dim {
            // TODO: replace unwrap()
            let line = lines_it.next().unwrap();
            func(
                &line
                    .as_ref()
                    .trim()
                    .split_whitespace()
                    .collect::<Vec<&str>>(),
                &mut dta,
            );
            count += 1;
        }

        self.coords = Some(dta);

        Ok(())
    }

    fn parse_fixed_edge_section<I>(&mut self, lines_it: &mut I) -> Result<(), ParseTspError>
    where
        I: Iterator,
        <I as Iterator>::Item: AsRef<str>,
    {
        let mut dta = Vec::new();

        loop {
            let line = lines_it.next().unwrap();
            if line.as_ref().trim().starts_with("-1") {
                break;
            }

            let mut it = line.as_ref().trim().split_whitespace();
            if let (Some(f), Some(l)) = (it.next(), it.next()) {
                dta.push((f.parse::<usize>().unwrap(), l.parse::<usize>().unwrap()));
            }
        }

        self.fixed_edges = Some(dta);

        Ok(())
    }

    fn parse_edge_weight_section<I>(&mut self, lines_it: &mut I) -> Result<(), ParseTspError>
    where
        I: Iterator,
        <I as Iterator>::Item: AsRef<str>,
    {
        self.validate_spec()?;
        let dim = self.dim.unwrap();

        // TODO: check memory consumption for large files.
        let (len_vec, cnt, mut it): (usize, usize, Box<dyn Iterator<Item = usize>>) =
            match self.weight_format.unwrap() {
                WeightFormat::Function => (0, 0, Box::new(std::iter::empty::<usize>())),
                WeightFormat::FullMatrix => {
                    (dim, dim * dim, Box::new(std::iter::repeat(dim).take(dim)))
                }
                WeightFormat::UpperRow => (dim - 1, dim * (dim - 1) / 2, Box::new((1..dim).rev())),
                WeightFormat::LowerRow => (dim - 1, dim * (dim - 1) / 2, Box::new(1..dim)),
                WeightFormat::UpperRowDiag => (dim, dim * (dim + 1) / 2, Box::new((1..=dim).rev())),
                WeightFormat::LowerRowDiag => (dim, dim * (dim + 1) / 2, Box::new(1..=dim)),
                WeightFormat::UpperCol => todo!(),
                WeightFormat::LowerCol => todo!(),
                WeightFormat::UpperColDiag => todo!(),
                WeightFormat::LowerColDiag => todo!(),
                WeightFormat::Undefined => (0, 0, Box::new(std::iter::empty::<usize>())),
            };

        let mut dta = Vec::with_capacity(len_vec);
        let mut v = Vec::with_capacity(cnt);

        while v.len() < cnt {
            let line = lines_it.next().unwrap();
            let mut tmp: Vec<f64> = line
                .as_ref()
                .trim()
                .split_whitespace()
                .map(|s| s.parse::<f64>().unwrap())
                .collect();

            v.append(&mut tmp);
        }

        while let Some(len_row) = it.next() {
            dta.push(v.drain(0..len_row).collect());
        }

        self.edge_weights = Some(dta);

        Ok(())
    }

    fn parse_display_data_section<I>(&mut self, lines_it: &mut I) -> Result<(), ParseTspError>
    where
        I: Iterator,
        <I as Iterator>::Item: AsRef<str>,
    {
        self.validate_spec()?;
        let dim = self.dim.unwrap();
        let mut dta = Vec::with_capacity(dim);

        let mut count = 0;
        while count < dim {
            let line = lines_it.next().unwrap();
            let v = line
                .as_ref()
                .trim()
                .split_whitespace()
                .collect::<Vec<&str>>();
            dta.push(Point::from((
                v[0].parse::<usize>().unwrap(),
                v[1].parse::<f64>().unwrap(),
                v[2].parse::<f64>().unwrap(),
            )));

            count += 1;
        }

        self.disp_coords = Some(dta);

        Ok(())
    }

    fn validate_spec(&self) -> Result<(), ParseTspError> {
        if self.name.is_none() {
            return Err(ParseTspError::MissingEntry(String::from(K_NAME)));
        }

        if self.kind.is_none() {
            return Err(ParseTspError::MissingEntry(String::from(K_TYPE)));
        }

        if self.dim.is_none() {
            return Err(ParseTspError::MissingEntry(String::from(K_DIM)));
        }

        if self.weight_kind.is_none() {
            return Err(ParseTspError::MissingEntry(String::from(K_WEIGHT_TYPE)));
        }

        Ok(())
    }

    fn validate_data(&self) -> Result<(), ParseTspError> {
        if self.weight_kind.is_some() {
            match self.weight_kind.unwrap() {
                WeightKind::Explicit => {
                    if self.edge_weights.is_none() {
                        return Err(ParseTspError::MissingEntry(String::from(K_EDGE_WEIGHT_SEC)));
                    }
                }
                _ => {
                    if self.coords.is_none() {
                        return Err(ParseTspError::MissingEntry(String::from(K_NODE_COORD_SEC)));
                    }
                }
            }
        }

        Ok(())
    }

    /// Validates the inputs and constructs a [`Tsp`] object if the validation is successful.
    /// Otherwise, returns an error [`ParseTspError`].
    pub fn build(self) -> Result<Tsp, ParseTspError> {
        self.validate_spec()?;
        self.validate_data()?;

        Ok(Tsp {
            name: self.name.unwrap(),
            kind: self.kind.unwrap(),
            comment: self.comment,
            dim: self.dim.unwrap(),
            weight_kind: self.weight_kind.unwrap(),
            weight_format: self.weight_format.unwrap_or(WeightFormat::Undefined),
            edge_format: self.edge_format.unwrap_or(EdgeFormat::Undefined),
            coord_kind: self.coord_kind.unwrap_or(CoordKind::Undefined),
            disp_kind: self.disp_kind.unwrap_or(DisplayKind::Undefined),
            node_coords: self.coords,
            edge_weights: self.edge_weights,
            disp_coords: self.disp_coords,
            fixed_edges: self.fixed_edges,
        })
    }
}

#[derive(Debug)]
enum Point {
    TwoD(usize, f64, f64),
    ThreeD(usize, f64, f64, f64),
}

impl From<(usize, f64, f64)> for Point {
    fn from(data: (usize, f64, f64)) -> Self {
        Self::TwoD(data.0, data.1, data.2)
    }
}

impl From<(usize, f64, f64, f64)> for Point {
    fn from(data: (usize, f64, f64, f64)) -> Self {
        Self::ThreeD(data.0, data.1, data.2, data.3)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum TspKind {
    /// Represents a symmetric travelling salesman problem.
    Tsp,
    /// Represents an asymmetric travelling salesman problem.
    Atsp,
    /// Represents a sequential ordering problem.
    Sop,
    /// Represents a Hamiltonian cycle problem.
    Hcp,
    /// Represents a capacitated vehicle routing problem.
    Cvrp,
    /// A collection of tours.
    Tour,
    Undefined,
}

impl_disp_enum!(TspKind);

impl From<&str> for TspKind {
    fn from(s: &str) -> Self {
        match s {
            "TSP" => Self::Tsp,
            "ATSP" => Self::Atsp,
            "SOP" => Self::Sop,
            "HCP" => Self::Hcp,
            "CVRP" => Self::Cvrp,
            "TOUR" => Self::Tour,
            _ => Self::Undefined,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum WeightKind {
    /// Weights are explicitly given in the data file.
    Explicit,
    /// Weights are measured through the Euclidean norm in 2D.
    Euc2d,
    /// Weights are measured through the Euclidean norm in 3D.
    Euc3d,
    /// Weights are measured through the maximum norm in 2D.
    Max2d,
    /// Weights are measured through the maximum norm in 3D.
    Max3d,
    /// Weights are measured through the Manhattan norm in 2D.
    Man2d,
    /// Weights are measured through the Manhattan norm in 3D.
    Man3d,
    /// Weights are measured through the Euclidean norm in 3D and then rounded up.
    Ceil2d,
    /// Weights are measured through the geographical distance function.
    Geo,
    /// Special distance function for problems ```att48``` and ```att532```.
    Att,
    /// Weights are measure through the special function (version 1) for crystallography problems.
    Xray1,
    /// Weights are measure through the special function (version 2) for crystallography problems.
    Xray2,
    /// The distance function is defined outside the scope of the data file.
    Custom,
    Undefined,
}

impl_disp_enum!(WeightKind);

impl From<&str> for WeightKind {
    fn from(s: &str) -> Self {
        match s {
            "EXPLICIT" => Self::Explicit,
            "EUC_2D" => Self::Euc2d,
            "EUC_3D" => Self::Euc3d,
            "MAX_2D" => Self::Max2d,
            "MAX_3D" => Self::Max3d,
            "MAN_2D" => Self::Man2d,
            "MAN_3D" => Self::Man3d,
            "CEIL_2D" => Self::Ceil2d,
            "GEO" => Self::Geo,
            "ATT" => Self::Att,
            "XRAY1" => Self::Xray1,
            "XRAY2" => Self::Xray2,
            "SPECIAL" => Self::Custom,
            _ => Self::Undefined,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum WeightFormat {
    /// Weights are calculated by the function stated in [WeightKind].
    Function,
    /// Weights are given in a full matrix.
    FullMatrix,
    /// Weights are given in an upper triangular matrix, row-wise without diagonal entries.
    UpperRow,
    /// Weights are given in a lower triangular matrix, row-wise without diagonal entries.
    LowerRow,
    /// Weights are given in an upper triangular matrix, row-wise with diagonal entries.
    UpperRowDiag,
    /// Weights are given in a lower triangular matrix, row-wise with diagonal entries.
    LowerRowDiag,
    /// Weights are given in an upper triangular matrix, col-wise without diagonal entries.
    UpperCol,
    /// Weights are given in an lower triangular matrix, col-wise without diagonal entries.
    LowerCol,
    /// Weights are given in an upper triangular matrix, col-wise with diagonal entries.
    UpperColDiag,
    /// Weights are given in a lower triangular matrix, col-wise with diagonal entries.
    LowerColDiag,
    Undefined,
}

impl From<&str> for WeightFormat {
    fn from(s: &str) -> Self {
        match s {
            "FUNCTION" => Self::Function,
            "FULL_MATRIX" => Self::FullMatrix,
            "UPPER_ROW" => Self::UpperRow,
            "LOWER_ROW" => Self::LowerRow,
            "UPPER_DIAG_ROW" => Self::UpperRowDiag,
            "LOWER_DIAG_ROW" => Self::LowerRowDiag,
            "UPPER_COL" => Self::UpperCol,
            "LOWER_COL" => Self::LowerCol,
            "UPPER_DIAG_COL" => Self::UpperColDiag,
            "LOWER_DIAG_COL" => Self::LowerColDiag,
            _ => Self::Undefined,
        }
    }
}

impl_disp_enum!(WeightFormat);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum EdgeFormat {
    EdgeList,
    AdjList,
    Undefined,
}

impl From<&str> for EdgeFormat {
    fn from(s: &str) -> Self {
        match s {
            "EDGE_LIST" => Self::EdgeList,
            "ADJ_LIST" => Self::AdjList,
            _ => Self::Undefined,
        }
    }
}

impl_disp_enum!(EdgeFormat);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum CoordKind {
    Coo2d,
    Coo3d,
    NoCoo,
    Undefined,
}

impl From<&str> for CoordKind {
    fn from(s: &str) -> Self {
        match s {
            "TWOD_COORDS" => Self::Coo2d,
            "THREED_COORDS" => Self::Coo3d,
            "NO_COORDS" => Self::NoCoo,
            _ => Self::Undefined,
        }
    }
}

impl From<WeightKind> for CoordKind {
    fn from(kind: WeightKind) -> Self {
        match kind {
            WeightKind::Euc2d
            | WeightKind::Max2d
            | WeightKind::Man2d
            | WeightKind::Ceil2d
            | WeightKind::Geo => Self::Coo2d,
            WeightKind::Euc3d | WeightKind::Max3d | WeightKind::Man3d => Self::Coo3d,
            _ => Self::Undefined,
        }
    }
}

impl_disp_enum!(CoordKind);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum DisplayKind {
    DispCoo,
    Disp2d,
    NoDisp,
    Undefined,
}

impl From<&str> for DisplayKind {
    fn from(s: &str) -> Self {
        match s {
            "COORD_DISPLAY" => Self::DispCoo,
            "TWOD_DISPLAY" => Self::Disp2d,
            "NO_DISPLAY" => Self::NoDisp,
            _ => Self::Undefined,
        }
    }
}

impl_disp_enum!(DisplayKind);