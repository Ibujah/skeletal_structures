use super::{IterCellNode2, IterHalfEdge2, Simplicial2};

#[derive(Copy, Clone)]
/// Halfedge iterator
pub struct IterCellHalfEdge2<'a> {
    simplicial: &'a Simplicial2,
    ind_halfedge: usize,
}

impl<'a> IterCellHalfEdge2<'a> {
    /// Creates a new halfedge iterator from the given manifold triangular mesh and index.
    pub(super) fn new(simplicial: &'a Simplicial2, ind_halfedge: usize) -> IterCellHalfEdge2<'a> {
        IterCellHalfEdge2 {
            simplicial,
            ind_halfedge,
        }
    }

    /// Get dual halfedge
    pub fn dual(&self) -> IterHalfEdge2<'a> {
        IterHalfEdge2::new(self.simplicial, self.ind_halfedge)
    }

    /// Get opposite halfedge
    pub fn opposite(&self) -> IterCellHalfEdge2<'a> {
        self.dual().opposite().dual()
    }

    /// Get first node of the cell halfedge
    pub fn first_node(&self) -> IterCellNode2<'a> {
        self.dual().triangle().dual()
    }

    /// Get last node of the cell halfedge
    pub fn last_node(&self) -> IterCellNode2<'a> {
        self.dual().opposite().triangle().dual()
    }

    /// Next cell edge
    pub fn next(&self) -> IterCellHalfEdge2<'a> {
        self.dual().opposite().next().dual()
    }

    /// Previous cell edge
    pub fn previous(&self) -> IterCellHalfEdge2<'a> {
        self.dual().previous().opposite().dual()
    }

    /// Halfedge to string
    pub fn to_string(&self) -> String {
        format!(
            "CellEdge {} -> {}",
            self.first_node().index(),
            self.last_node().index()
        )
    }

    /// Print halfedge string
    pub fn print(&self) -> () {
        print!("{}", self.to_string());
    }

    /// Println halfedge string
    pub fn println(&self) -> () {
        println!("{}", self.to_string());
    }
}
