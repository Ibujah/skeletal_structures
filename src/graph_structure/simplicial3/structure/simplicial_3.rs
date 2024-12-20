use anyhow::Result;

use std::cmp::max;

use super::IterNode3;
use super::IterTetrahedron3;

/// 3D Simplicial structure
pub struct Simplicial3 {
    // i   : nod0 \
    // i+1 : nod1  | -> tetrahedron
    // i+2 : nod2  |
    // i+3 : nod3 /
    // such that tri0 = (i+3, i+2, i+1)
    // such that tri1 = (i+2, i+3, i+0)
    // such that tri2 = (i+1, i+0, i+3)
    // such that tri3 = (i+0, i+1, i+2)
    tet_nodes: Vec<usize>,
    halftriangle_opposite: Vec<usize>,

    nb_tetrahedra: usize,

    // optional attribute, containing  node positions within tet_nodes
    node_positions: Option<Vec<Vec<usize>>>,
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
            nb_tetrahedra: 0,
            node_positions,
        }
    }

    /////////////////////////////
    /// Private build methods ///
    /////////////////////////////

    fn add_empty_tetrahedron(&mut self) -> usize {
        self.tet_nodes.resize(self.tet_nodes.len() + 4, 0);
        self.halftriangle_opposite
            .resize(self.halftriangle_opposite.len() + 4, 0);

        self.nb_tetrahedra = self.nb_tetrahedra + 1;

        self.nb_tetrahedra - 1
    }

    fn set_tetrahedron(
        &mut self,
        ind_tet: usize,
        nod1: usize,
        nod2: usize,
        nod3: usize,
        nod4: usize,
    ) -> [usize; 4] {
        let ind_first = ind_tet * 4;
        self.tet_nodes[ind_first] = nod1;
        self.tet_nodes[ind_first + 1] = nod2;
        self.tet_nodes[ind_first + 2] = nod3;
        self.tet_nodes[ind_first + 3] = nod3;

        if let Some(vec) = self.node_positions.as_mut() {
            let max_nod = max(max(nod1, nod2), max(nod3, nod4));
            if vec.len() <= max_nod {
                vec.resize(max_nod + 1, Vec::new());
            }
            vec[nod1].push(ind_first);
            vec[nod2].push(ind_first + 1);
            vec[nod3].push(ind_first + 2);
            vec[nod4].push(ind_first + 3);
        }

        [ind_first, ind_first + 1, ind_first + 2, ind_first + 3]
    }

    fn oppose_halftriangles(&mut self, htri0: usize, htri1: usize) {
        self.halftriangle_opposite[htri0] = htri1;
        self.halftriangle_opposite[htri1] = htri0;
    }

    ////////////////////////////
    /// Public find methods ///
    ////////////////////////////

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

    ///////////////////////////////
    /// Public browsing methods ///
    ///////////////////////////////

    /// Gets halfedge first node value
    pub fn node_value(&self, ind_node: usize) -> usize {
        self.tet_nodes[ind_node]
    }

    /// Gets number of tetrahedra
    pub fn get_nb_tetrahedra(&self) -> usize {
        self.nb_tetrahedra
    }

    /// Gets tetrahedron iterator from index
    pub fn get_tetrahedron_from_index(&self, ind_tetra: usize) -> Result<IterTetrahedron3> {
        if ind_tetra > self.get_nb_tetrahedra() {
            return Err(anyhow::Error::msg("Tetrahedron index out of bounds"));
        }
        Ok(IterTetrahedron3::new(self, ind_tetra))
    }

    ////////////////////////////////
    /// Public modifying methods ///
    ////////////////////////////////

    /// Inserts a first tetrahedron in the structure
    pub fn first_tetrahedron(&mut self, nodes: [usize; 4]) -> Result<[usize; 2]> {
        if self.nb_tetrahedra != 0 {
            return Err(anyhow::Error::msg("Already tetrahedra in simplicial"));
        }
        let [n0, n1, n2, n3] = nodes;

        let ind_tet0 = self.add_empty_tetrahedron();
        let ind_tet1 = self.add_empty_tetrahedron();

        let [t321, t230, t103, t012] = self.set_tetrahedron(ind_tet0, n0, n1, n2, n3);
        let [t032, t301, t210, t123] = self.set_tetrahedron(ind_tet1, n1, n2, n3, n0);

        self.oppose_halftriangles(t321, t123);
        self.oppose_halftriangles(t230, t032);
        self.oppose_halftriangles(t103, t301);
        self.oppose_halftriangles(t012, t210);

        Ok([ind_tet0, ind_tet1])
    }
}
