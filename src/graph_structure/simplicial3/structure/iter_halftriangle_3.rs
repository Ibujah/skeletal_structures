use super::{IterHalfEdge3, IterNode3, Simplicial3};

/// For each triangle index within tetrahedron,
/// associate list of vertices within tetrahedron
pub const XOR_TRIANGLE_SUBINDICES: [usize; 3] = [3, 2, 1];
/// 0: [3, 2, 1]
/// 1: [2, 3, 0]
/// 2: [1, 0, 3]
/// 3: [0, 1, 2]

#[derive(Copy, Clone)]
/// Half triangle iterator
pub struct IterHalfTriangle3<'a> {
    simplicial: &'a Simplicial3,
    ind_halftriangle: usize,
}

impl<'a> IterHalfTriangle3<'a> {
    /// Creates a new vertex iterator from the given manifold triangular simplicial and index.
    pub(super) fn new(
        simplicial: &'a Simplicial3,
        ind_halftriangle: usize,
    ) -> IterHalfTriangle3<'a> {
        IterHalfTriangle3 {
            simplicial,
            ind_halftriangle,
        }
    }

    /// Gets node indices
    pub fn node_indices(&self) -> [usize; 3] {
        [
            self.ind_halftriangle ^ XOR_TRIANGLE_SUBINDICES[0],
            self.ind_halftriangle ^ XOR_TRIANGLE_SUBINDICES[1],
            self.ind_halftriangle ^ XOR_TRIANGLE_SUBINDICES[2],
        ]
    }

    /// Gets node iterators
    pub fn nodes(&self) -> [IterNode3<'a>; 3] {
        let ind_nods = self.node_indices();
        [
            IterNode3::new(self.simplicial, ind_nods[0]),
            IterNode3::new(self.simplicial, ind_nods[1]),
            IterNode3::new(self.simplicial, ind_nods[2]),
        ]
    }

    /// Gets list of halfedges starting at this vertex
    pub fn halfedges(&self) -> [IterHalfEdge3<'a>; 3] {
        [
            IterHalfEdge3::new(
                self.simplicial,
                self.ind_halftriangle,
                XOR_TRIANGLE_SUBINDICES[0],
                XOR_TRIANGLE_SUBINDICES[1],
            ),
            IterHalfEdge3::new(
                self.simplicial,
                self.ind_halftriangle,
                XOR_TRIANGLE_SUBINDICES[1],
                XOR_TRIANGLE_SUBINDICES[2],
            ),
            IterHalfEdge3::new(
                self.simplicial,
                self.ind_halftriangle,
                XOR_TRIANGLE_SUBINDICES[2],
                XOR_TRIANGLE_SUBINDICES[0],
            ),
        ]
    }
}
