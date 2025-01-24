use super::IterDiagHalfEdge2;
use super::IterDiagNode2;
use super::IterNode2;
use super::Simplicial2;

#[derive(Copy, Clone)]
/// Dual cell iterator
pub struct IterDiagCell2<'a> {
    simplicial: &'a Simplicial2,
    ind_halfedge: usize,
}

impl<'a> IterDiagCell2<'a> {
    /// Creates a new cell iterator from the given manifold triangular simplicial and index.
    pub(super) fn new(simplicial: &'a Simplicial2, ind_halfedge: usize) -> IterDiagCell2<'a> {
        IterDiagCell2 {
            simplicial,
            ind_halfedge,
        }
    }

    /// Gets vertex index
    pub fn value(&self) -> usize {
        self.simplicial.halfedge_first_node_value(self.ind_halfedge)
    }

    /// Gets list of halfedges surrouding the cell
    pub fn halfedges(&self) -> Vec<IterDiagHalfEdge2<'a>> {
        self.dual()
            .halfedges()
            .iter()
            .map(|&he| he.dual())
            .collect()
    }

    /// Gets list of nodes surrouding the cell
    pub fn nodes(&self) -> Vec<IterDiagNode2<'a>> {
        self.dual()
            .halfedges()
            .iter()
            .map(|&he| he.dual().first_node())
            .collect()
    }

    /// Get dual node
    pub fn dual(&self) -> IterNode2<'a> {
        IterNode2::new(self.simplicial, self.ind_halfedge)
    }

    /// Cell to string
    pub fn to_string(&self) -> String {
        format!("Diagram Cell {}", self.value(),)
    }
}
