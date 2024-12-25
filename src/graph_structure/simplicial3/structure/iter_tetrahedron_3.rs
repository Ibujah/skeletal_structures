use super::{IterHalfTriangle3, IterNode3, Simplicial3};

#[derive(Copy, Clone)]
/// Triangle iterator
pub struct IterTetrahedron3<'a> {
    simplicial: &'a Simplicial3,
    ind_tetrahedron: usize,
}

impl<'a> IterTetrahedron3<'a> {
    /// Creates a new tetrahedron iterator from the given manifold triangular simplicial and index.
    pub(super) fn new(simplicial: &'a Simplicial3, ind_tetrahedron: usize) -> IterTetrahedron3<'a> {
        IterTetrahedron3 {
            simplicial,
            ind_tetrahedron,
        }
    }

    /// Gets index
    pub fn ind(&self) -> usize {
        self.ind_tetrahedron
    }

    /// Gets list of halftriangles starting surrounding this tetrahedron
    pub fn halftriangles(&self) -> [IterHalfTriangle3<'a>; 4] {
        let ind_first = self.ind_tetrahedron << 2;
        [
            IterHalfTriangle3::new(self.simplicial, ind_first + 0),
            IterHalfTriangle3::new(self.simplicial, ind_first + 1),
            IterHalfTriangle3::new(self.simplicial, ind_first + 2),
            IterHalfTriangle3::new(self.simplicial, ind_first + 3),
        ]
    }

    /// Gets node iterators
    pub fn nodes(&self) -> [IterNode3<'a>; 4] {
        let ind_first = self.ind_tetrahedron << 2;
        [
            IterNode3::new(self.simplicial, ind_first + 0),
            IterNode3::new(self.simplicial, ind_first + 1),
            IterNode3::new(self.simplicial, ind_first + 2),
            IterNode3::new(self.simplicial, ind_first + 3),
        ]
    }

    /// Gets node values
    pub fn node_values(&self) -> [usize; 4] {
        let ind_first = self.ind_tetrahedron << 2;
        [
            self.simplicial.tet_nodes[ind_first + 0],
            self.simplicial.tet_nodes[ind_first + 1],
            self.simplicial.tet_nodes[ind_first + 2],
            self.simplicial.tet_nodes[ind_first + 3],
        ]
    }

    /// Converts tetrahedron to string
    pub fn to_string(&self) -> String {
        let [n0, n1, n2, n3] = self.node_values();
        format!("[{} {} {} {}]", n0, n1, n2, n3)
    }
}
