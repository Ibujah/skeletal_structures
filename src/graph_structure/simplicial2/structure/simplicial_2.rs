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
    halfedge_first_node: Vec<usize>,
    halfedge_opposite: Vec<usize>,

    // optional attribute, containing indices of halfedges starting at given node
    node_halfedges: Option<Vec<Vec<usize>>>,

    nb_triangles: usize,
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

    fn add_empty_triangle(&mut self) -> usize {
        self.halfedge_first_node
            .resize(self.halfedge_first_node.len() + 3, 0);
        self.halfedge_opposite
            .resize(self.halfedge_opposite.len() + 3, 0);

        self.nb_triangles = self.nb_triangles + 1;

        self.nb_triangles - 1
    }

    fn set_triangle(
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

    fn unset_triangle(&mut self, ind_tri: usize) -> usize {
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

    fn oppose_halfedges(&mut self, he0: usize, he1: usize) {
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
            vec[ind_node].clone()
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

    ////////////////////////////////
    /// Public modifying methods ///
    ////////////////////////////////

    /// Inserts first triangle, and its opposite, in the structure
    pub fn insert_first_triangle(&mut self, nodes: [usize; 3]) -> Result<[usize; 2]> {
        if self.nb_triangles != 0 {
            return Err(anyhow::Error::msg("Already triangles in simplicial"));
        }

        let [n0, n1, n2] = nodes;

        let ind_tri0 = self.add_empty_triangle();
        let ind_tri1 = self.add_empty_triangle();

        let [h01, h12, h20] = self.set_triangle(ind_tri0, n0, n1, n2);
        let [h02, h21, h10] = self.set_triangle(ind_tri1, n0, n2, n1);

        self.oppose_halfedges(h01, h10);
        self.oppose_halfedges(h12, h21);
        self.oppose_halfedges(h20, h02);

        Ok([ind_tri0, ind_tri1])
    }

    /// Inserts a new node in a triangle
    pub fn insert_node_within_triangle(
        &mut self,
        node: usize,
        ind_tri: usize,
    ) -> Result<[usize; 3]> {
        let [n0, n1, n2] = self.triangle_node_values(ind_tri);
        let [h01, h12, h20] = self.triangle_halfedge_indices(ind_tri);
        let h10 = self.halfedge_opposite_index(h01);
        let h21 = self.halfedge_opposite_index(h12);
        let h02 = self.halfedge_opposite_index(h20);

        let ind_tri0 = self.unset_triangle(ind_tri);
        let ind_tri1 = self.add_empty_triangle();
        let ind_tri2 = self.add_empty_triangle();

        let [h01, h1n, hn0] = self.set_triangle(ind_tri0, n0, n1, node);
        let [h12, h2n, hn1] = self.set_triangle(ind_tri1, n1, n2, node);
        let [h20, h0n, hn2] = self.set_triangle(ind_tri2, n2, n0, node);

        self.oppose_halfedges(h10, h01);
        self.oppose_halfedges(h12, h21);
        self.oppose_halfedges(h20, h02);

        self.oppose_halfedges(h0n, hn0);
        self.oppose_halfedges(h1n, hn1);
        self.oppose_halfedges(h2n, hn2);

        Ok([ind_tri, self.nb_triangles - 2, self.nb_triangles - 1])
    }

    /// Flips halfedge
    pub fn flip_halfedge(&mut self, ind_he: usize) -> Result<[usize; 2]> {
        let ind_he_opp = self.halfedge_opposite_index(ind_he);

        // ind_he is ca
        let hab = self.halfedge_next_index(ind_he);
        let hbc = self.halfedge_previous_index(ind_he);

        // ind_he_opp is ac
        let hcd = self.halfedge_next_index(ind_he_opp);
        let hda = self.halfedge_previous_index(ind_he_opp);

        let na = self.halfedge_first_node_value(hab);
        let nb = self.halfedge_first_node_value(hbc);
        let nc = self.halfedge_first_node_value(hcd);
        let nd = self.halfedge_first_node_value(hda);

        let hba = self.halfedge_opposite_index(hab);
        let hcb = self.halfedge_opposite_index(hbc);
        let hdc = self.halfedge_opposite_index(hcd);
        let had = self.halfedge_opposite_index(hda);

        let ind_tri1 = self.halfedge_triangle_index(ind_he);
        let ind_tri2 = self.halfedge_triangle_index(ind_he_opp);

        let [hbc, hcd, hdb] = self.set_triangle(ind_tri1, nb, nc, nd);
        let [hda, hab, hbd] = self.set_triangle(ind_tri2, nd, na, nb);

        self.oppose_halfedges(hab, hba);
        self.oppose_halfedges(hbc, hcb);
        self.oppose_halfedges(hcd, hdc);
        self.oppose_halfedges(hda, had);

        self.oppose_halfedges(hbd, hdb);

        Ok([hbd, hdb])
    }

    /// Builds full simplicial from set of triangles
    pub fn insert_triangle_list(&mut self, triangles: Vec<[usize; 3]>) -> Result<()> {
        if self.get_nb_triangles() != 0 {
            return Err(anyhow::Error::msg("Simplicial should be empty"));
        }
        for &[nod0, nod1, nod2] in triangles.iter() {
            let ind_tri = self.add_empty_triangle();
            self.set_triangle(ind_tri, nod0, nod1, nod2);
        }

        let mut to_attribute: Vec<usize> = (0..self.get_nb_halfedges()).collect();
        while let Some(ind_he) = to_attribute.pop() {
            let n0 = self.halfedge_first_node_value(ind_he);
            let n1 = self.halfedge_last_node_value(ind_he);
            let mut found = false;
            for i in 0..to_attribute.len() {
                let ind_he_opp = to_attribute[i];
                let n0o = self.halfedge_first_node_value(ind_he_opp);
                let n1o = self.halfedge_last_node_value(ind_he_opp);
                if n0 == n1o && n1 == n0o {
                    self.oppose_halfedges(ind_he, ind_he_opp);
                    to_attribute.remove(i);
                    found = true;
                    break;
                }
            }
            if !found {
                return Err(anyhow::Error::msg(
                    "Given faces do not form a manifold self",
                ));
            }
        }

        Ok(())
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
