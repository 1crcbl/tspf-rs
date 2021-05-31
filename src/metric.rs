use std::f64::consts::PI;

const EARTH_RADIUS: f64 = 6378.388;

/// An enum for distance functions defined in the ```TSPLIB``` format.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum MetricKind {
    /// Two-dimensional Euclidean distance.
    Euc2d,
    /// Three-dimensional Euclidean distance.
    Euc3d,
    /// Two-dimensional maximum distance.
    Max2d,
    /// Three-dimensional maximum distance.
    Max3d,
    /// Two-dimensional Manhattan distance.
    Man2d,
    /// Three-dimensional Manhattan distance.
    Man3d,
    /// Geographical distance.
    Geo,
    /// Special distance function for problems ```att48``` and ```att532```.
    Att,
    /// Special distance function for crystallography problems of version 1.
    Xray1,
    /// Special distance function for crystallography problems of version 2.
    Xray2,
    /// Distance function defined by users.
    Custom,
    /// No distance function is given.
    Undefined,
}

impl MetricKind {
    /// Calculates and returns the cost (or distance) between two points.
    ///
    /// For [`MetricKind::Custom`] and [`MetricKind::Undefined`], the function will always return ```0.```.
    pub fn cost<T>(&self, a: &T, b: &T) -> f64
    where
        T: MetricPoint,
    {
        match self {
            MetricKind::Euc2d => euc_2d(a.x(), a.y(), b.x(), b.y()),
            MetricKind::Euc3d => euc_3d(a.x(), a.y(), a.z(), b.x(), b.y(), b.z()),
            MetricKind::Geo => geo(a.x(), a.y(), b.x(), b.y()),
            MetricKind::Max2d => max_2d(a.x(), a.y(), b.x(), b.y()),
            MetricKind::Max3d => max_3d(a.x(), a.y(), a.z(), b.x(), b.y(), b.z()),
            MetricKind::Man2d => man_2d(a.x(), a.y(), b.x(), b.y()),
            MetricKind::Man3d => man_3d(a.x(), a.y(), a.z(), b.x(), b.y(), b.z()),
            MetricKind::Att => att(a.x(), a.y(), b.x(), b.y()),
            MetricKind::Xray1 => xray1(a.x(), a.y(), a.z(), b.x(), b.y(), b.z()),
            MetricKind::Xray2 => xray2(a.x(), a.y(), a.z(), b.x(), b.y(), b.z()),
            _ => 0.,
        }
    }
}

pub trait MetricPoint {
    /// Returns the ```x```-coordinate of a point.
    fn x(&self) -> f64;

    /// Returns the ```y```-coordinate of a point.
    fn y(&self) -> f64;

    /// Returns the ```z```-coordinate of a point.
    fn z(&self) -> f64;
}

/// Calculates the 2D-Euclidean distance between two points.
#[inline]
pub fn euc_2d(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
}

/// Calculates the 3D-Euclidean distance between two points.
#[inline]
pub fn euc_3d(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> f64 {
    ((x1 - x2).powi(2) + (y1 - y2).powi(2) + (z1 - z2).powi(2)).sqrt()
}

/// Calculates the 2D-Manhattan distance between two points.
#[inline]
pub fn man_2d(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

/// Calculates the 3D-Manhattan distance between two points.
#[inline]
pub fn man_3d(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> f64 {
    (x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs()
}

/// Calculates the 2D maximum distance between two points.
#[inline]
pub fn max_2d(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let (dx, dy) = ((x1 - x2).abs(), (y1 - y2).abs());
    dx.max(dy)
}

/// Calculates the 3D maximum distance between two points.
#[inline]
pub fn max_3d(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> f64 {
    let (dx, dy, dz) = ((x1 - x2).abs(), (y1 - y2).abs(), (z1 - z2).abs());
    dx.max(dy).max(dz)
}

/// Calculates the geographical between two points.
#[inline]
pub fn geo(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let (lat_a, lon_a) = (to_geo_coord(x1), to_geo_coord(y1));
    let (lat_b, lon_b) = (to_geo_coord(x2), to_geo_coord(y2));

    let q1 = (lon_a - lon_b).cos();
    let q2 = (lat_a - lat_b).cos();
    let q3 = (lat_a + lat_b).cos();
    let q4 = (0.5 * ((1. + q1) * q2 - (1. - q1) * q3)).acos();
    EARTH_RADIUS * q4 + 1.
}

#[inline]
fn to_geo_coord(x: f64) -> f64 {
    let deg = x.trunc();
    let min = x - deg;
    PI * (deg + 5. * min / 3.) / 180.
}

/// Calculates the distance between two points for dataset from AT&T Bell laboratory, published by Padberg and Rinaldi in 1987.
#[inline]
pub fn att(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    (((x1 - x2).powi(2) + (y1 - y2).powi(2)) / 10.).sqrt()
}

/// Calculates the distance between two points for crystallography problems (version 1).
#[inline]
pub fn xray1(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> f64 {
    let dx = (x1 - x2).abs();
    let p = dx.min((dx - 360.).abs());
    let c = (y1 - y2).abs();
    let t = (z1 - z2).abs();
    100. * p.max(c.max(t))
}

/// Calculates the distance between two points for crystallography problems (version 2).
#[inline]
pub fn xray2(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> f64 {
    let dx = (x1 - x2).abs();
    let p = dx.min((dx - 360.).abs());
    let c = (y1 - y2).abs();
    let t = (z1 - z2).abs();
    100. * (p / 1.25).max((c / 1.5).max(t / 1.15))
}
