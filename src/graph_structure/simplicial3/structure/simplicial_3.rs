use anyhow::Result;

use super::IterHalfEdge3;
use super::IterHalfTriangle3;
use super::IterNode3;
use super::IterTetrahedron3;

pub(super) const ABC2BAC: usize = 2;
pub(super) const ABC2CBA: usize = 1;
pub(super) const ABC2ACB: usize = 0;

/// 3D Simplicial structure
pub struct Simplicial3 {
    // i   : nod0 \
    // i+1 : nod1  | -> tetrahedron
    // i+2 : nod2  |
    // i+3 : nod3 /
    // such that tri0 = (i+3, i+2, i+1) = ((i+0)^3, (i+0)^2, (i+0)^1)
    // such that tri1 = (i+2, i+3, i+0) = ((i+1)^3, (i+1)^2, (i+1)^1)
    // such that tri2 = (i+1, i+0, i+3) = ((i+2)^3, (i+2)^2, (i+2)^1)
    // such that tri3 = (i+0, i+1, i+2) = ((i+3)^3, (i+3)^2, (i+3)^1)
    pub(super) tet_nodes: Vec<usize>,
    /// For each halftriangle, index of opposite halftriangle
    pub(super) halftriangle_opposite: Vec<usize>,
    /// Three kind if shifts, to associate halfedges between each halftriangle
    pub(super) halftriangle_shift: Vec<usize>,

    /// Number of tetrahedra
    pub(super) nb_tetrahedra: usize,

    // optional attribute, containing  node positions within tet_nodes
    pub(super) node_positions: Option<Vec<Vec<usize>>>,
}

impl Simplicial3 {
    /// Simplicial structure initialisation
    pub fn new(register_node_halfedges: bool) -> Simplicial3 {
        let node_positions = if register_node_halfedges {
            Some(Vec::new())
        } else {
            None
        };
        Simplicial3 {
            tet_nodes: Vec::new(),
            halftriangle_opposite: Vec::new(),
            halftriangle_shift: Vec::new(),
            nb_tetrahedra: 0,
            node_positions,
        }
    }

    pub(super) fn get_opposite_xor(
        &self,
        shift: usize,
        xor0: usize,
        xor1: usize,
        xor2: usize,
    ) -> (usize, usize, usize) {
        (
            ((xor0 + xor0 + shift) % 3) + 1,
            ((xor1 + xor0 + shift) % 3) + 1,
            ((xor2 + xor0 + shift) % 3) + 1,
        )
    }
    ///////////////////////////
    /// Public find methods ///
    ///////////////////////////

    /// Checks if a node is in the simplicial
    pub fn find_node(&self, node: usize) -> Option<IterNode3> {
        if let Some(vec) = &self.node_positions {
            if node >= vec.len() || vec[node].is_empty() {
                None
            } else {
                let ind_node = vec[node][0];
                Some(IterNode3::new(self, ind_node))
            }
        } else {
            for ind_node in 0..self.tet_nodes.len() - 1 {
                if self.tet_nodes[ind_node] == node {
                    return Some(IterNode3::new(self, ind_node));
                }
            }
            None
        }
    }

    /// Checks if a halfedge is in the simplicial
    pub fn find_halfedge(&self, node0: usize, node1: usize) -> Option<IterHalfEdge3> {
        if let Some(node) = self.find_node(node0) {
            let vec_he = node.halfedges();
            for he in vec_he {
                if he.node_values()[1] == node1 {
                    return Some(he);
                }
            }
        }
        None
    }

    /// Checks if a halftriangle is in the simplicial
    pub fn find_halftriangle(
        &self,
        node0: usize,
        node1: usize,
        node2: usize,
    ) -> Option<IterHalfTriangle3> {
        if let Some(node) = self.find_node(node0) {
            let vec_he = node.halfedges();
            for he in vec_he {
                if he.node_values()[1] == node1 && he.next().node_values()[1] == node2 {
                    return Some(he.halftriangle());
                }
            }
        }
        None
    }

    /// Checks if a tetrahedron is in the simplicial
    pub fn find_tetrahedron(
        &self,
        node0: usize,
        node1: usize,
        node2: usize,
        node3: usize,
    ) -> Option<IterTetrahedron3> {
        if let Some(htri_test0) = self.find_halftriangle(node0, node1, node2) {
            let he_test0 = htri_test0.halfedges()[0];
            let nod_test0 = he_test0.neighbor().next().node_values()[1];

            if nod_test0 == node3 {
                return Some(htri_test0.tetrahedron());
            }

            let htri_test1 = htri_test0.opposite();
            let he_test1 = htri_test1.halfedges()[0];
            let nod_test1 = he_test1.neighbor().next().node_values()[1];

            if nod_test1 == node3 {
                return Some(htri_test1.tetrahedron());
            }
        }
        None
    }

    ///////////////////////////////
    /// Public browsing methods ///
    ///////////////////////////////

    /// Gets opposite halftriangle index
    pub fn get_halftriangle_opposite(&self, ind_halftriangle: usize) -> usize {
        self.halftriangle_opposite[ind_halftriangle]
    }

    /// Gets node value
    pub fn node_value(&self, ind_node: usize) -> usize {
        self.tet_nodes[ind_node]
    }

    /// Gets list of node indices for given node
    pub fn node_indices(&self, node: usize) -> Vec<usize> {
        if let Some(position) = &self.node_positions {
            position[node].clone()
        } else {
            self.tet_nodes
                .iter()
                .enumerate()
                .filter(|(_, &val)| val == node)
                .map(|(ind_nod, _)| ind_nod)
                .collect()
        }
    }

    /// Gets number of tetrahedra
    pub fn get_nb_tetrahedra(&self) -> usize {
        self.nb_tetrahedra
    }

    /// Gets triangle iterator from index
    pub fn get_halftriangle_from_index(&self, ind_htri: usize) -> Result<IterHalfTriangle3> {
        if ind_htri > self.get_nb_tetrahedra() << 2 {
            return Err(anyhow::Error::msg("Halftriangle index out of bounds"));
        }
        Ok(IterHalfTriangle3::new(self, ind_htri))
    }

    /// Gets tetrahedron iterator from index
    pub fn get_tetrahedron_from_index(&self, ind_tetra: usize) -> Result<IterTetrahedron3> {
        if ind_tetra > self.get_nb_tetrahedra() {
            return Err(anyhow::Error::msg("Tetrahedron index out of bounds"));
        }
        Ok(IterTetrahedron3::new(self, ind_tetra))
    }

    /// Gets all halftriangles iterators
    pub fn get_all_halftriangles(&self) -> Vec<IterHalfTriangle3> {
        (0..self.get_nb_tetrahedra() << 2)
            .into_iter()
            .map(|ind_htri| IterHalfTriangle3::new(self, ind_htri))
            .collect()
    }

    /// Gets all halfedge iterators
    pub fn get_all_halfedges(&self) -> Vec<IterHalfEdge3> {
        self.get_all_halftriangles()
            .into_iter()
            .flat_map(|htri| htri.halfedges())
            .collect()
    }

    /// Gets all tetrahedron iterators
    pub fn get_all_tetrahedra(&self) -> Vec<IterTetrahedron3> {
        (0..self.get_nb_tetrahedra())
            .into_iter()
            .map(|ind_tetra| IterTetrahedron3::new(self, ind_tetra))
            .collect()
    }
}
