use super::{IterDiagNode2, IterHalfEdge2, Simplicial2};

#[derive(Copy, Clone)]
/// Diagram halfedge iterator
pub struct IterDiagHalfEdge2<'a> {
    simplicial: &'a Simplicial2,
    ind_halfedge: usize,
}

impl<'a> IterDiagHalfEdge2<'a> {
    /// Creates a new dual halfedge iterator from the given manifold triangular mesh and index.
    pub(super) fn new(simplicial: &'a Simplicial2, ind_halfedge: usize) -> IterDiagHalfEdge2<'a> {
        IterDiagHalfEdge2 {
            simplicial,
            ind_halfedge,
        }
    }

    /// Get dual halfedge
    pub fn dual(&self) -> IterHalfEdge2<'a> {
        IterHalfEdge2::new(self.simplicial, self.ind_halfedge)
    }

    /// Get opposite diagram halfedge
    pub fn opposite(&self) -> IterDiagHalfEdge2<'a> {
        self.dual().opposite().dual()
    }

    /// Get first node of the diagram halfedge
    pub fn first_node(&self) -> IterDiagNode2<'a> {
        self.dual().triangle().dual()
    }

    /// Get last node of the diagram halfedge
    pub fn last_node(&self) -> IterDiagNode2<'a> {
        self.dual().opposite().triangle().dual()
    }

    /// Next diagram edge
    pub fn next(&self) -> IterDiagHalfEdge2<'a> {
        self.dual().opposite().next().dual()
    }

    /// Previous diagram edge
    pub fn previous(&self) -> IterDiagHalfEdge2<'a> {
        self.dual().previous().opposite().dual()
    }

    /// Halfedge to string
    pub fn to_string(&self) -> String {
        format!(
            "Diagram Edge {} -> {}",
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
