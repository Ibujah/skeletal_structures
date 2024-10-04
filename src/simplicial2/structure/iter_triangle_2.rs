use super::IterCellNode2;
use super::IterHalfEdge2;
use super::IterNode2;
use super::Simplicial2;

#[derive(Copy, Clone)]
/// Triangle iterator
pub struct IterTriangle2<'a> {
    simplicial: &'a Simplicial2,
    ind_triangle: usize,
}

impl<'a> IterTriangle2<'a> {
    /// Creates a new triangle iterator from the given manifold triangular mesh and index.
    pub(super) fn new(simplicial: &'a Simplicial2, ind_triangle: usize) -> IterTriangle2<'a> {
        IterTriangle2 {
            simplicial,
            ind_triangle,
        }
    }

    /// Gets triangle index
    /// /!\ Can be modified if simplicial is modified
    pub fn index(&self) -> usize {
        self.ind_triangle
    }

    /// Surrounding halfedges (array of halfedge iterators)
    pub fn halfedges(&self) -> [IterHalfEdge2<'a>; 3] {
        let [ind_he0, ind_he1, ind_he2] =
            self.simplicial.triangle_halfedge_indices(self.ind_triangle);
        [
            IterHalfEdge2::new(self.simplicial, ind_he0),
            IterHalfEdge2::new(self.simplicial, ind_he1),
            IterHalfEdge2::new(self.simplicial, ind_he2),
        ]
    }

    /// usizes(array of nodes)
    pub fn node_values(&self) -> [usize; 3] {
        self.simplicial.triangle_node_values(self.ind_triangle)
    }

    /// array of nodes
    pub fn nodes(&self) -> [IterNode2<'a>; 3] {
        let [he0, he1, he2] = self.halfedges();
        [he0.first_node(), he1.first_node(), he2.first_node()]
    }

    /// Get dual cell node
    pub fn dual(&self) -> IterCellNode2<'a> {
        IterCellNode2::new(self.simplicial, self.ind_triangle)
    }

    /// Triangle to string
    pub fn to_string(&self) -> String {
        let [nod1, nod2, nod3] = self.nodes();
        format!(
            "Face {} -> {} -> {}",
            nod1.value(),
            nod2.value(),
            nod3.value()
        )
    }

    /// Print triangle string
    pub fn print(&self) -> () {
        print!("{}", self.to_string());
    }

    /// Println triangle string
    pub fn println(&self) -> () {
        println!("{}", self.to_string());
    }
}
