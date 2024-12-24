use super::{IterHalfTriangle3, IterNode3, IterTetrahedron3, Simplicial3};
use crate::graph_structure::simplicial3::structure::iter_halftriangle_3::XOR_TRIANGLE_SUBINDICES;

#[derive(Copy, Clone)]
/// Halfedge iterator
pub struct IterHalfEdge3<'a> {
    simplicial: &'a Simplicial3,
    ind_first: usize,
    xor0: usize,
    xor1: usize,
    xor2: usize,
}

impl<'a> IterHalfEdge3<'a> {
    /// Creates a new halfedge iterator from the given manifold simplicial and index.
    pub(super) fn new(
        simplicial: &'a Simplicial3,
        ind_face: usize,
        xor0: usize,
        xor1: usize,
        xor2: usize,
    ) -> IterHalfEdge3<'a> {
        IterHalfEdge3 {
            simplicial,
            ind_first: ind_face ^ xor0,
            xor0,
            xor1,
            xor2,
        }
    }

    pub(super) fn halfedges_starting_from_node(
        simplicial: &'a Simplicial3,
        ind_node: usize,
    ) -> [IterHalfEdge3<'a>; 3] {
        [
            IterHalfEdge3 {
                simplicial,
                ind_first: ind_node,
                xor0: XOR_TRIANGLE_SUBINDICES[0],
                xor1: XOR_TRIANGLE_SUBINDICES[1],
                xor2: XOR_TRIANGLE_SUBINDICES[2],
            },
            IterHalfEdge3 {
                simplicial,
                ind_first: ind_node,
                xor0: XOR_TRIANGLE_SUBINDICES[1],
                xor1: XOR_TRIANGLE_SUBINDICES[2],
                xor2: XOR_TRIANGLE_SUBINDICES[0],
            },
            IterHalfEdge3 {
                simplicial,
                ind_first: ind_node,
                xor0: XOR_TRIANGLE_SUBINDICES[2],
                xor1: XOR_TRIANGLE_SUBINDICES[0],
                xor2: XOR_TRIANGLE_SUBINDICES[1],
            },
        ]
    }

    pub(super) fn subindex(&self) -> usize {
        self.xor0 ^ 3
    }

    /// Gets node values
    pub fn node_values(&self) -> [usize; 2] {
        [
            self.simplicial.node_value(self.ind_first),
            self.simplicial.node_value(self.ind_first ^ self.xor2),
        ]
    }

    /// Gets first node
    pub fn first_node(&self) -> IterNode3<'a> {
        IterNode3::new(self.simplicial, self.ind_first)
    }

    /// Gets last node
    pub fn last_node(&self) -> IterNode3<'a> {
        IterNode3::new(self.simplicial, self.ind_first ^ self.xor2)
    }

    /// Gets next halfedge
    pub fn next(&self) -> IterHalfEdge3<'a> {
        IterHalfEdge3 {
            simplicial: self.simplicial,
            ind_first: self.ind_first ^ self.xor2,
            xor0: self.xor1,
            xor1: self.xor2,
            xor2: self.xor0,
        }
    }

    /// Gets prev halfedge
    pub fn prev(&self) -> IterHalfEdge3<'a> {
        IterHalfEdge3 {
            simplicial: self.simplicial,
            ind_first: self.ind_first ^ self.xor1,
            xor0: self.xor2,
            xor1: self.xor0,
            xor2: self.xor1,
        }
    }

    /// Gets neighor halfedge (on same tetrahedron)
    pub fn neighbor(&self) -> IterHalfEdge3<'a> {
        IterHalfEdge3 {
            simplicial: self.simplicial,
            ind_first: self.ind_first ^ self.xor2,
            xor0: self.xor0,
            xor1: self.xor1,
            xor2: self.xor2,
        }
    }

    /// Gets opposite halfedge, on opposite halftriangle
    pub fn opposite(&self) -> IterHalfEdge3<'a> {
        let ind_halftriangle = self.ind_first ^ self.xor0;
        self.simplicial
            .get_opposite_halfedge(ind_halftriangle, self.xor0, self.xor1, self.xor2)
    }

    /// Gets halftriangle
    pub fn halftriangle(&self) -> IterHalfTriangle3<'a> {
        IterHalfTriangle3::new(&self.simplicial, self.ind_first ^ self.xor0)
    }

    /// Gets tetrahedron iterator
    pub fn tetrahedron(&self) -> IterTetrahedron3<'a> {
        IterTetrahedron3::new(self.simplicial, self.ind_first >> 2)
    }

    /// Print halfedge values
    pub fn print(&self) -> () {
        let [n0, n1] = self.node_values();
        print!("[{} {}]", n0, n1)
    }
}
