use super::{IterNode3, Simplicial3};

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
    ) -> IterHalfEdge3<'a> {
        IterHalfEdge3 {
            simplicial,
            ind_first: ind_face ^ xor0,
            xor0,
            xor1,
            xor2: xor0 ^ xor1,
        }
    }

    /// Gets node indices
    pub fn node_indices(&self) -> [usize; 2] {
        [self.ind_first, self.ind_first ^ self.xor2]
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
}
