use anyhow::Result;
use std::cmp::max;

use super::IterHalfEdge2;
use super::IterNode2;
use super::IterTriangle2;

/// 2D Simplicial structure
/// Always valid (all methods have to let simplicial with no holes, no triple edges)
#[derive(Clone)]
pub struct Simplicial2 {
    // i   : he1 \
    // i+1 : he2  | -> triangle
    // i+2 : he3 /
    // such that he2 = next(he1)
    // such that he3 = next(he2)
    // such that he1 = next(he3)
    // only first node is stored (last node is the next one, taking account of the %3)
    pub(super) halfedge_first_node: Vec<usize>,
    pub(super) halfedge_opposite: Vec<usize>,

    // optional attribute, containing indices of halfedges starting at given node
    pub(super) node_halfedges: Option<Vec<Vec<usize>>>,

    pub(super) nb_triangles: usize,
}

impl Simplicial2 {
    /// Simplicial structure initialisation
    pub fn new(register_node_halfedges: bool) -> Simplicial2 {
        let node_halfedges = if register_node_halfedges {
            Some(Vec::new())
        } else {
            None
        };
        Simplicial2 {
            halfedge_first_node: Vec::new(),
            halfedge_opposite: Vec::new(),
            node_halfedges,
            nb_triangles: 0,
        }
    }

    /////////////////////////////
    /// Private build methods ///
    /////////////////////////////

    pub(super) fn add_empty_triangle(&mut self) -> usize {
        self.halfedge_first_node
            .resize(self.halfedge_first_node.len() + 3, 0);
        self.halfedge_opposite
            .resize(self.halfedge_opposite.len() + 3, 0);

        self.nb_triangles = self.nb_triangles + 1;

        self.nb_triangles - 1
    }

    pub(super) fn set_triangle(
        &mut self,
        ind_tri: usize,
        nod1: usize,
        nod2: usize,
        nod3: usize,
    ) -> [usize; 3] {
        let ind_first = ind_tri * 3;
        self.halfedge_first_node[ind_first] = nod1;
        self.halfedge_first_node[ind_first + 1] = nod2;
        self.halfedge_first_node[ind_first + 2] = nod3;

        if let Some(vec) = self.node_halfedges.as_mut() {
            let max_nod = max(max(nod1, nod2), nod3);
            if vec.len() <= max_nod {
                vec.resize(max_nod + 1, Vec::new());
            }
            vec[nod1].push(ind_first);
            vec[nod2].push(ind_first + 1);
            vec[nod3].push(ind_first + 2);
        }

        [ind_first, ind_first + 1, ind_first + 2]
    }

    pub(super) fn unset_triangle(&mut self, ind_tri: usize) -> usize {
        if let Some(vec) = self.node_halfedges.as_mut() {
            let ind_first = ind_tri * 3;
            let nod1 = self.halfedge_first_node[ind_first];
            let nod2 = self.halfedge_first_node[ind_first + 1];
            let nod3 = self.halfedge_first_node[ind_first + 2];

            vec[nod1].retain(|&ind_he| ind_he != ind_first);
            vec[nod2].retain(|&ind_he| ind_he != (ind_first + 1));
            vec[nod3].retain(|&ind_he| ind_he != (ind_first + 2));
        }
        ind_tri
    }

    pub(super) fn oppose_halfedges(&mut self, he0: usize, he1: usize) {
        self.halfedge_opposite[he0] = he1;
        self.halfedge_opposite[he1] = he0;
    }

    ////////////////////////////
    /// Private find methods ///
    ////////////////////////////

    /// Checks if a node is in the simplicial
    fn find_node_index(&self, node0: usize) -> Option<usize> {
        if let Some(vec) = &self.node_halfedges {
            if node0 >= vec.len() || vec[node0].is_empty() {
                None
            } else {
                Some(vec[node0][0])
            }
        } else {
            for ind_halfedge in 0..self.halfedge_first_node.len() - 1 {
                if self.halfedge_first_node[ind_halfedge] == node0 {
                    return Some(ind_halfedge);
                }
            }
            None
        }
    }

    ///////////////////////////
    /// Public find methods ///
    ///////////////////////////

    /// Checks if an edge is in the simplicial
    pub fn find_halfedge_index(&self, node0: usize, node1: usize) -> Option<usize> {
        if let Some(ind_node) = self.find_node_index(node0) {
            for &ind_he in self.node_halfedge_indices(ind_node).iter() {
                if self.halfedge_last_node_value(ind_he) == node1 {
                    return Some(ind_he);
                }
            }
        }
        None
    }

    /// Checks if a triangle is in the simplicial
    pub fn find_triangle_index(&self, node0: usize, node1: usize, node2: usize) -> Option<usize> {
        if let Some(ind_he) = self.find_halfedge_index(node0, node1) {
            let ind_he_next = self.halfedge_next_index(ind_he);
            if self.halfedge_last_node_value(ind_he_next) == node2 {
                return Some(self.halfedge_triangle_index(ind_he));
            }
            let ind_he_opp_next = self.halfedge_next_index(self.halfedge_opposite_index(ind_he));
            if self.halfedge_last_node_value(ind_he_opp_next) == node2 {
                return Some(self.halfedge_triangle_index(ind_he_opp_next));
            }
        }

        None
    }

    ///////////////////////////////
    /// Public browsing methods ///
    ///////////////////////////////

    /// Gets halfedge iterator from index
    pub fn get_halfedge_from_index(&self, ind_he: usize) -> Result<IterHalfEdge2> {
        if ind_he > self.halfedge_first_node.len() {
            return Err(anyhow::Error::msg("Halfedge index out of bounds"));
        }
        Ok(IterHalfEdge2::new(self, ind_he))
    }

    /// Gets triangle iterator from index
    pub fn get_triangle_from_index(&self, ind_tri: usize) -> Result<IterTriangle2> {
        if ind_tri > self.get_nb_triangles() {
            return Err(anyhow::Error::msg("Triangle index out of bounds"));
        }
        Ok(IterTriangle2::new(self, ind_tri))
    }

    /// Gets node halfedges indices
    pub fn node_halfedge_indices(&self, ind_node: usize) -> Vec<usize> {
        if let Some(vec) = &self.node_halfedges {
            let nod_val = self.halfedge_first_node[ind_node];
            vec[nod_val].clone()
        } else {
            let mut vec_he = Vec::new();
            let ind_he = ind_node;

            vec_he.push(ind_he);
            let mut ind_he_cur = self.halfedge_next_index(self.halfedge_opposite_index(ind_he));
            while ind_he_cur != ind_he {
                vec_he.push(ind_he_cur);
                ind_he_cur = self.halfedge_next_index(self.halfedge_opposite_index(ind_he_cur));
            }

            vec_he
        }
    }

    /// Gets halfedge first node value
    pub fn halfedge_first_node_value(&self, ind_he: usize) -> usize {
        self.halfedge_first_node[ind_he]
    }

    /// Gets halfedge last node value
    pub fn halfedge_last_node_value(&self, ind_he: usize) -> usize {
        let ind_he_next = self.halfedge_next_index(ind_he);
        self.halfedge_first_node_value(ind_he_next)
    }

    /// Gets halfedge next halfedge index
    pub fn halfedge_next_index(&self, ind_he: usize) -> usize {
        let on_fac = ind_he % 3;

        if on_fac == 2 {
            ind_he - 2
        } else {
            ind_he + 1
        }
    }

    /// Gets halfedge previous halfedge index
    pub fn halfedge_previous_index(&self, ind_he: usize) -> usize {
        let on_fac = ind_he % 3;

        if on_fac == 0 {
            ind_he + 2
        } else {
            ind_he - 1
        }
    }

    /// Gets halfedge opposite halfedge index
    pub fn halfedge_opposite_index(&self, ind_he: usize) -> usize {
        self.halfedge_opposite[ind_he]
    }

    /// Gets halfedge triangle index
    pub fn halfedge_triangle_index(&self, ind_he: usize) -> usize {
        ind_he / 3
    }

    /// Gets triangle nodes values
    pub fn triangle_node_values(&self, ind_tri: usize) -> [usize; 3] {
        let ind_first = ind_tri * 3;

        [
            self.halfedge_first_node[ind_first],
            self.halfedge_first_node[ind_first + 1],
            self.halfedge_first_node[ind_first + 2],
        ]
    }

    /// Gets triangle halfedges indices
    pub fn triangle_halfedge_indices(&self, ind_tri: usize) -> [usize; 3] {
        let ind_first = ind_tri * 3;

        [ind_first, ind_first + 1, ind_first + 2]
    }

    ///////////////////////////////
    /// Public browsing methods ///
    ///////////////////////////////

    /// Gets total number of halfedges
    pub fn get_nb_halfedges(&self) -> usize {
        self.halfedge_first_node.len()
    }

    /// Gets number of triangles
    pub fn get_nb_triangles(&self) -> usize {
        self.nb_triangles
    }

    /// Gets all halfedge iterators
    pub fn get_all_halfedges(&self) -> Vec<IterHalfEdge2> {
        (0..self.get_nb_halfedges())
            .into_iter()
            .map(|ind_he| IterHalfEdge2::new(self, ind_he))
            .collect()
    }

    /// Gets all triangle iterators
    pub fn get_all_triangles(&self) -> Vec<IterTriangle2> {
        (0..self.get_nb_triangles())
            .into_iter()
            .map(|ind_he| IterTriangle2::new(self, ind_he))
            .collect()
    }

    /// Checks if a node is in the simplicial
    ///
    /// Returns halfedge iterator if found
    pub fn find_node(&self, node0: usize) -> Option<IterNode2> {
        if let Some(ind_node) = self.find_node_index(node0) {
            Some(IterNode2::new(self, ind_node))
        } else {
            None
        }
    }
    /// Checks if an edge is in the simplicial
    ///
    /// Returns halfedge iterator if found
    pub fn find_halfedge(&self, node0: usize, node1: usize) -> Option<IterHalfEdge2> {
        if let Some(ind_he) = self.find_halfedge_index(node0, node1) {
            Some(IterHalfEdge2::new(self, ind_he))
        } else {
            None
        }
    }

    /// Checks if a triangle is in the simplicial
    ///
    /// Returns triangle iterator if found
    pub fn find_triangle(&self, node0: usize, node1: usize, node2: usize) -> Option<IterTriangle2> {
        if let Some(ind_tri) = self.find_triangle_index(node0, node1, node2) {
            Some(IterTriangle2::new(self, ind_tri))
        } else {
            None
        }
    }

    /// Println each triangle of the graph
    pub fn println(&self) -> () {
        for ind_tri in 0..self.nb_triangles {
            let tri = IterTriangle2::new(self, ind_tri);
            print!("  ");
            tri.println();
        }
    }
}
