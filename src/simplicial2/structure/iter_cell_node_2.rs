use super::IterCellHalfEdge2;
use super::IterTriangle2;
use super::Simplicial2;

#[derive(Copy, Clone)]
/// Cell node iterator
pub struct IterCellNode2<'a> {
    simplicial: &'a Simplicial2,
    ind_triangle: usize,
}

impl<'a> IterCellNode2<'a> {
    /// Creates a new cell node iterator from the given manifold triangular mesh and index.
    pub(super) fn new(simplicial: &'a Simplicial2, ind_triangle: usize) -> IterCellNode2<'a> {
        IterCellNode2 {
            simplicial,
            ind_triangle,
        }
    }

    /// Gets cell node index
    /// /!\ Can be modified if simplicial is modified
    pub fn index(&self) -> usize {
        self.ind_triangle
    }

    /// Surrounding cell halfedges strating from cell node
    pub fn cell_halfedges(&self) -> [IterCellHalfEdge2<'a>; 3] {
        let [he0, he1, he2] = self.dual().halfedges();
        [he0.dual(), he1.dual(), he2.dual()]
    }

    /// Get dual triangle
    pub fn dual(&self) -> IterTriangle2<'a> {
        IterTriangle2::new(self.simplicial, self.ind_triangle)
    }

    /// Cell Node to string
    pub fn to_string(&self) -> String {
        format!("Cell Node {}", self.index())
    }
}
