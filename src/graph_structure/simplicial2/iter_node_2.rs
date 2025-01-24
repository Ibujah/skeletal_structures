use super::Simplicial2;

use super::IterDiagCell2;
use super::IterHalfEdge2;

#[derive(Copy, Clone)]
/// Vertex iterator
pub struct IterNode2<'a> {
    simplicial: &'a Simplicial2,
    ind_halfedge: usize,
}

impl<'a> IterNode2<'a> {
    /// Creates a new vertex iterator from the given manifold triangular simplicial and index.
    pub(super) fn new(simplicial: &'a Simplicial2, ind_halfedge: usize) -> IterNode2<'a> {
        IterNode2 {
            simplicial,
            ind_halfedge,
        }
    }

    /// Gets vertex index
    pub fn value(&self) -> usize {
        self.simplicial.halfedge_first_node_value(self.ind_halfedge)
    }

    /// Gets list of halfedges starting at this vertex
    pub fn halfedges(&self) -> Vec<IterHalfEdge2<'a>> {
        self.simplicial
            .node_halfedge_indices(self.ind_halfedge)
            .iter()
            .map(|&ind_he| IterHalfEdge2::new(&self.simplicial, ind_he))
            .collect()
    }

    /// Get dual cell
    pub fn dual(&self) -> IterDiagCell2<'a> {
        IterDiagCell2::new(self.simplicial, self.ind_halfedge)
    }

    /// Node to string
    pub fn to_string(&self) -> String {
        format!("Node {}", self.value(),)
    }
}
