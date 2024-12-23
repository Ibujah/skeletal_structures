use anyhow::Result;

use std::cmp::max;

use super::IterHalfEdge3;
use super::IterHalfTriangle3;
use super::IterNode3;
use super::IterTetrahedron3;

#[derive(Copy, Clone)]
pub(super) enum ShiftType {
    ABC2ACB = 0,
    ABC2BAC = 1,
    ABC2CBA = 2,
    Unset = 3,
}

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
    pub(super) halftriangle_shift: Vec<ShiftType>,

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

    /////////////////////////////
    /// Private build methods ///
    /////////////////////////////

    pub(super) fn add_empty_tetrahedron(&mut self) -> usize {
        self.tet_nodes.resize(self.tet_nodes.len() + 4, 0);
        self.halftriangle_opposite
            .resize(self.halftriangle_opposite.len() + 4, 0);
        self.halftriangle_shift
            .resize(self.halftriangle_shift.len() + 4, ShiftType::Unset);

        self.nb_tetrahedra = self.nb_tetrahedra + 1;

        self.nb_tetrahedra - 1
    }

    pub(super) fn unset_tetrahedron(&mut self, ind_tet: usize) {
        let ind_first = ind_tet * 4;

        let nod0 = self.tet_nodes[ind_first];
        let nod1 = self.tet_nodes[ind_first + 1];
        let nod2 = self.tet_nodes[ind_first + 2];
        let nod3 = self.tet_nodes[ind_first + 3];

        self.tet_nodes[ind_first] = 0;
        self.tet_nodes[ind_first + 1] = 0;
        self.tet_nodes[ind_first + 2] = 0;
        self.tet_nodes[ind_first + 3] = 0;

        self.halftriangle_opposite[ind_first] = 0;
        self.halftriangle_opposite[ind_first + 1] = 0;
        self.halftriangle_opposite[ind_first + 2] = 0;
        self.halftriangle_opposite[ind_first + 3] = 0;

        self.halftriangle_shift[ind_first] = ShiftType::Unset;
        self.halftriangle_shift[ind_first + 1] = ShiftType::Unset;
        self.halftriangle_shift[ind_first + 2] = ShiftType::Unset;
        self.halftriangle_shift[ind_first + 3] = ShiftType::Unset;

        if let Some(vec) = self.node_positions.as_mut() {
            vec[nod0].retain(|&x| x >> 2 != ind_tet);
            vec[nod1].retain(|&x| x >> 2 != ind_tet);
            vec[nod2].retain(|&x| x >> 2 != ind_tet);
            vec[nod3].retain(|&x| x >> 2 != ind_tet);
        }
    }

    pub(super) fn set_tetrahedron(
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
        self.tet_nodes[ind_first + 3] = nod4;

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

    pub(super) fn oppose_halftriangles(
        &mut self,
        htri0: usize,
        htri1: usize,
        shift_tri0: ShiftType,
        shift_tri1: ShiftType,
    ) {
        self.halftriangle_opposite[htri0] = htri1;
        self.halftriangle_opposite[htri1] = htri0;
        self.halftriangle_shift[htri0] = shift_tri0;
        self.halftriangle_shift[htri1] = shift_tri1;
    }

    pub(super) fn oppose_halftriangles_auto(&mut self, htri0: usize, htri1: usize) -> Result<()> {
        let tri0 = IterHalfTriangle3::new(self, htri0);
        let tri1 = IterHalfTriangle3::new(self, htri1);

        let [hab, _, _] = tri0.halfedges();
        let [hde, hef, hfd] = tri1.halfedges();

        let [a, b] = hab.node_values();

        let (shift0, shift1) = if hde.node_values() == [b, a] {
            (ShiftType::ABC2BAC, ShiftType::ABC2BAC)
        } else if hef.node_values() == [b, a] {
            (ShiftType::ABC2CBA, ShiftType::ABC2CBA)
        } else if hfd.node_values() == [b, a] {
            (ShiftType::ABC2ACB, ShiftType::ABC2ACB)
        } else {
            return Err(anyhow::Error::msg("Faces are not opposite"));
        };

        self.oppose_halftriangles(htri0, htri1, shift0, shift1);

        Ok(())
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

    ////////////////////////////////
    /// Private browsing methods ///
    ////////////////////////////////

    /// Gets opposite halftriangle index
    pub(super) fn get_halftriangle_opposite(&self, ind_halftriangle: usize) -> usize {
        self.halftriangle_opposite[ind_halftriangle]
    }

    pub(super) fn get_opposite_halfedge(
        &self,
        ind_halftriangle: usize,
        xor0: usize,
        xor1: usize,
        xor2: usize,
    ) -> IterHalfEdge3 {
        let ind_halftriangle_opposite = self.get_halftriangle_opposite(ind_halftriangle);

        let (new_xor0, new_xor1, new_xor2) = match self.halftriangle_shift[ind_halftriangle] {
            ShiftType::ABC2ACB => (xor2, xor1, xor0),
            ShiftType::ABC2CBA => (xor1, xor0, xor2),
            ShiftType::ABC2BAC => (xor0, xor2, xor1),
            ShiftType::Unset => panic!(),
        };
        IterHalfEdge3::new(
            self,
            ind_halftriangle_opposite,
            4 - new_xor0,
            4 - new_xor1,
            4 - new_xor2,
        )
    }

    ///////////////////////////////
    /// Public browsing methods ///
    ///////////////////////////////

    /// Gets node value
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

        self.oppose_halftriangles(t321, t123, ShiftType::ABC2CBA, ShiftType::ABC2CBA);
        self.oppose_halftriangles(t230, t032, ShiftType::ABC2CBA, ShiftType::ABC2CBA);
        self.oppose_halftriangles(t103, t301, ShiftType::ABC2CBA, ShiftType::ABC2CBA);
        self.oppose_halftriangles(t012, t210, ShiftType::ABC2CBA, ShiftType::ABC2CBA);

        Ok([ind_tet0, ind_tet1])
    }
}
