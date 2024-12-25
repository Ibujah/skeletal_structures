use super::{IterHalfEdge3, IterNode3, IterTetrahedron3, Simplicial3};

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

    /// Gets index
    pub fn ind(&self) -> usize {
        self.ind_halftriangle
    }

    /// Gets node values
    pub fn node_values(&self) -> [usize; 3] {
        [
            self.simplicial
                .node_value(self.ind_halftriangle ^ XOR_TRIANGLE_SUBINDICES[0]),
            self.simplicial
                .node_value(self.ind_halftriangle ^ XOR_TRIANGLE_SUBINDICES[1]),
            self.simplicial
                .node_value(self.ind_halftriangle ^ XOR_TRIANGLE_SUBINDICES[2]),
        ]
    }

    /// Gets node iterators
    pub fn nodes(&self) -> [IterNode3<'a>; 3] {
        let ind_nods = self.node_values();
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
                XOR_TRIANGLE_SUBINDICES[2],
            ),
            IterHalfEdge3::new(
                self.simplicial,
                self.ind_halftriangle,
                XOR_TRIANGLE_SUBINDICES[1],
                XOR_TRIANGLE_SUBINDICES[2],
                XOR_TRIANGLE_SUBINDICES[0],
            ),
            IterHalfEdge3::new(
                self.simplicial,
                self.ind_halftriangle,
                XOR_TRIANGLE_SUBINDICES[2],
                XOR_TRIANGLE_SUBINDICES[0],
                XOR_TRIANGLE_SUBINDICES[1],
            ),
        ]
    }

    /// Gets opposite halftriangle
    pub fn opposite(&self) -> IterHalfTriangle3<'a> {
        let ind_halftriangle_opposite = self
            .simplicial
            .get_halftriangle_opposite(self.ind_halftriangle);
        IterHalfTriangle3 {
            simplicial: self.simplicial,
            ind_halftriangle: ind_halftriangle_opposite,
        }
    }

    /// Gets tetrahedron iterator
    pub fn tetrahedron(&self) -> IterTetrahedron3<'a> {
        IterTetrahedron3::new(self.simplicial, self.ind_halftriangle >> 2)
    }

    /// Converts halftriangle to string
    pub fn to_string(&self) -> String {
        let [n0, n1, n2] = self.node_values();
        format!("[{} {} {}]", n0, n1, n2)
    }
}
