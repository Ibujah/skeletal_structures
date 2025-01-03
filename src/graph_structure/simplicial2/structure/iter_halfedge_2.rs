use super::IterDiagHalfEdge2;
use super::IterNode2;
use super::IterTriangle2;
use super::Simplicial2;

#[derive(Copy, Clone)]
/// Halfedge iterator
pub struct IterHalfEdge2<'a> {
    simplicial: &'a Simplicial2,
    ind_halfedge: usize,
}

impl<'a> IterHalfEdge2<'a> {
    /// Creates a new halfedge iterator from the given manifold triangular mesh and index.
    pub(super) fn new(simplicial: &'a Simplicial2, ind_halfedge: usize) -> IterHalfEdge2<'a> {
        IterHalfEdge2 {
            simplicial,
            ind_halfedge,
        }
    }

    /// Gets halfedge index
    /// /!\ Can be modified if simplicial is modified
    pub fn index(&self) -> usize {
        self.ind_halfedge
    }

    /// First node
    pub fn first_node(&self) -> IterNode2<'a> {
        IterNode2::new(self.simplicial, self.ind_halfedge)
    }

    /// Last node
    pub fn last_node(&self) -> IterNode2<'a> {
        self.next().first_node()
    }

    /// Next halfedge on same triangle
    pub fn next(&self) -> IterHalfEdge2<'a> {
        let ind_next = self.simplicial.halfedge_next_index(self.ind_halfedge);
        IterHalfEdge2::new(self.simplicial, ind_next)
    }

    /// Previous halfedge on same triangle
    pub fn previous(&self) -> IterHalfEdge2<'a> {
        let ind_prev = self.simplicial.halfedge_previous_index(self.ind_halfedge);
        IterHalfEdge2::new(self.simplicial, ind_prev)
    }

    /// Opposite halfedge: Same vertices in opposite order (on neighbor triangle)
    pub fn opposite(&self) -> IterHalfEdge2<'a> {
        let ind_opp = self.simplicial.halfedge_opposite_index(self.ind_halfedge);
        IterHalfEdge2::new(self.simplicial, ind_opp)
    }

    /// Triangle containing halfedge
    pub fn triangle(&self) -> IterTriangle2<'a> {
        let ind_triangle = self.simplicial.halfedge_triangle_index(self.ind_halfedge);
        IterTriangle2::new(self.simplicial, ind_triangle)
    }

    /// Get dual cell halfedge
    pub fn dual(&self) -> IterDiagHalfEdge2<'a> {
        IterDiagHalfEdge2::new(self.simplicial, self.ind_halfedge)
    }

    /// Halfedge to string
    pub fn to_string(&self) -> String {
        format!(
            "Edge {} -> {}",
            self.first_node().value(),
            self.last_node().value()
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
