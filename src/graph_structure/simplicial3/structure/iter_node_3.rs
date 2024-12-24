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

    /// Gets node value
    pub fn value(&self) -> usize {
        self.simplicial.node_value(self.ind_node)
    }

    /// Gets list of halfedges starting at this vertex
    pub fn halfedges(&self) -> Vec<IterHalfEdge3<'a>> {
        if let Some(position) = &self.simplicial.node_positions {
            let val = self.value();
            position[val]
                .iter()
                .flat_map(|&ind_nod| {
                    IterHalfEdge3::halfedges_starting_from_node(self.simplicial, ind_nod)
                        .into_iter()
                })
                .collect()
        } else {
            let val = self.value();
            let mut vec_he = Vec::new();
            for ind_nod in 0..self.simplicial.tet_nodes.len() {
                if self.simplicial.tet_nodes[ind_nod] == val {
                    let [he0, he1, he2] =
                        IterHalfEdge3::halfedges_starting_from_node(self.simplicial, ind_nod);
                    vec_he.push(he0);
                    vec_he.push(he1);
                    vec_he.push(he2);
                }
            }
            vec_he
        }
    }

    /// Print node value
    pub fn print(&self) -> () {
        let n0 = self.value();
        print!("[{}]", n0)
    }
}
