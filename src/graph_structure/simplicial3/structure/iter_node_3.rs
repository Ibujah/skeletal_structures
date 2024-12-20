use super::IterHalfEdge3;
use super::Simplicial3;

#[derive(Copy, Clone)]
/// Vertex iterator
pub struct IterNode3<'a> {
    simplicial: &'a Simplicial3,
    ind_node: usize,
}

impl<'a> IterNode3<'a> {
    /// Creates a new vertex iterator from the given manifold triangular simplicial and index.
    pub(super) fn new(simplicial: &'a Simplicial3, ind_node: usize) -> IterNode3<'a> {
        IterNode3 {
            simplicial,
            ind_node,
        }
    }

    /// Gets node
    pub fn value(&self) -> usize {
        self.simplicial.node_value(self.ind_node)
    }

    /// Gets list of halfedges starting at this vertex
    pub fn halfedges(&self) -> Vec<IterHalfEdge3<'a>> {
        todo!()
    }
}
