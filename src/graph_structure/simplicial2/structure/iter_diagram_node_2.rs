use super::IterDiagHalfEdge2;
use super::IterTriangle2;
use super::Simplicial2;

#[derive(Copy, Clone)]
/// Dual node iterator
pub struct IterDiagNode2<'a> {
    simplicial: &'a Simplicial2,
    ind_triangle: usize,
}

impl<'a> IterDiagNode2<'a> {
    /// Creates a new diagram node iterator from the given manifold triangular mesh and index.
    pub(super) fn new(simplicial: &'a Simplicial2, ind_triangle: usize) -> IterDiagNode2<'a> {
        IterDiagNode2 {
            simplicial,
            ind_triangle,
        }
    }

    /// Gets diagram node index
    /// /!\ Can be modified if simplicial is modified
    pub fn index(&self) -> usize {
        self.ind_triangle
    }

    /// Surrounding diagarm halfedges strating from cell node
    pub fn halfedges(&self) -> [IterDiagHalfEdge2<'a>; 3] {
        let [he0, he1, he2] = self.dual().halfedges();
        [he0.dual(), he1.dual(), he2.dual()]
    }

    /// Get dual triangle
    pub fn dual(&self) -> IterTriangle2<'a> {
        IterTriangle2::new(self.simplicial, self.ind_triangle)
    }

    /// Cell Node to string
    pub fn to_string(&self) -> String {
        format!("Diagram Node {}", self.index())
    }
}
