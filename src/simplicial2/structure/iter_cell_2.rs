use super::IterCellHalfEdge2;
use super::IterCellNode2;
use super::IterNode2;
use super::Simplicial2;

#[derive(Copy, Clone)]
/// Vertex iterator
pub struct IterCell2<'a> {
    simplicial: &'a Simplicial2,
    ind_halfedge: usize,
}

impl<'a> IterCell2<'a> {
    /// Creates a new cell iterator from the given manifold triangular simplicial and index.
    pub(super) fn new(simplicial: &'a Simplicial2, ind_halfedge: usize) -> IterCell2<'a> {
        IterCell2 {
            simplicial,
            ind_halfedge,
        }
    }

    /// Gets vertex index
    pub fn value(&self) -> usize {
        self.simplicial.halfedge_first_node_value(self.ind_halfedge)
    }

    /// Gets list of cell halfedges surrouding the cell
    pub fn cell_halfedges(&self) -> Vec<IterCellHalfEdge2<'a>> {
        self.dual()
            .halfedges()
            .iter()
            .map(|&he| he.dual())
            .collect()
    }

    /// Gets list of cell nodes surrouding the cell
    pub fn cell_nodes(&self) -> Vec<IterCellNode2<'a>> {
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
        format!("Cell {}", self.value(),)
    }
}
