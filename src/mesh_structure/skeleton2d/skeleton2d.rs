use nalgebra::base::*;

#[derive(Clone)]
/// Skeleton 2D is a struct that represents a 2D skeleton
pub struct Skeleton2D {
    /// A vector of vertices, where each vertex is a 2D vector,
    /// which is a point with x, y coordinates.
    pub(super) vertices: Vec<Vector2<f64>>,

    /// List of neighbors of each vertex
    pub(super) neighbors: Vec<Vec<usize>>,
}
