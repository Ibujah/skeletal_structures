use anyhow::Result;

use crate::graph_structure::simplicial3::structure::simplicial_3::ShiftType;

use super::{IterHalfTriangle3, IterTetrahedron3, Simplicial3};

/// Bowyer Watson algorithm to insert a node in a simplicial3 structure
pub struct BowyerWatsonInserter<'a> {
    simplicial: &'a mut Simplicial3,

    // structures to speed up tetrahedra insertion with Bowyer Watson algorithm
    should_rem_tet: Vec<bool>,
    should_keep_tet: Vec<bool>,
    tet_to_rem: Vec<usize>,
    tet_to_keep: Vec<usize>,
    tet_to_check: Vec<usize>,
}

impl<'a> BowyerWatsonInserter<'a> {
    /// Starts BW insertion, setting a first tetrahedron to remove
    pub fn new(simplicial: &mut Simplicial3, ind_first_tetra: usize) -> BowyerWatsonInserter {
        let nb_tetrahedra = simplicial.get_nb_tetrahedra();
        let mut bw_inserter = BowyerWatsonInserter {
            simplicial,
            should_rem_tet: vec![false; nb_tetrahedra],
            should_keep_tet: vec![false; nb_tetrahedra],
            tet_to_rem: Vec::new(),
            tet_to_keep: Vec::new(),
            tet_to_check: Vec::new(),
        };

        bw_inserter.bw_rem_tetra(ind_first_tetra);

        bw_inserter
    }

    /// Gets next tetrahedron to check
    /// returns set of 4 nodes
    pub fn bw_tetra_to_check(&mut self) -> Option<[usize; 4]> {
        loop {
            if let Some(ind_tetra) = self.tet_to_check.pop() {
                if self.should_rem_tet[ind_tetra] == false
                    && self.should_keep_tet[ind_tetra] == false
                {
                    let ind_first = ind_tetra << 2;
                    return Some([
                        self.simplicial.tet_nodes[ind_first],
                        self.simplicial.tet_nodes[ind_first + 1],
                        self.simplicial.tet_nodes[ind_first + 2],
                        self.simplicial.tet_nodes[ind_first + 3],
                    ]);
                }
            } else {
                break;
            }
        }
        None
    }

    /// Sets tetrahedron to remove
    pub fn bw_rem_tetra(&mut self, ind_tetra: usize) -> () {
        // get all triangles indices
        let tri0 = ind_tetra << 2;
        let tri1 = tri0 + 1;
        let tri2 = tri0 + 2;
        let tri3 = tri0 + 3;

        // get all opposite triangles indices
        let opp_tri0 = self.simplicial.get_halftriangle_opposite(tri0);
        let opp_tri1 = self.simplicial.get_halftriangle_opposite(tri1);
        let opp_tri2 = self.simplicial.get_halftriangle_opposite(tri2);
        let opp_tri3 = self.simplicial.get_halftriangle_opposite(tri3);

        // get all opposite tetrahedra indices
        let opp_tet0 = opp_tri0 >> 2;
        let opp_tet1 = opp_tri1 >> 2;
        let opp_tet2 = opp_tri2 >> 2;
        let opp_tet3 = opp_tri3 >> 2;

        // set opposite tetrahedra to check
        self.tet_to_check.push(opp_tet0);
        self.tet_to_check.push(opp_tet1);
        self.tet_to_check.push(opp_tet2);
        self.tet_to_check.push(opp_tet3);

        // check ind_tetra and and it to the remove list
        self.should_rem_tet[ind_tetra] = true;
        self.tet_to_rem.push(ind_tetra);
    }

    /// Sets tetrahedron to keep
    pub fn bw_keep_tetra(&mut self, ind_tetra: usize) -> () {
        self.should_keep_tet[ind_tetra] = true;
        self.tet_to_keep.push(ind_tetra);
    }

    fn find_first_boundary_triangle(&self) -> Result<usize> {
        if let Some(&ind_tetra_keep) = self.tet_to_keep.last() {
            let tetra = IterTetrahedron3::new(self.simplicial, ind_tetra_keep);
            let tris = tetra.halftriangles();
            if self.should_rem_tet[tris[0].opposite().tetrahedron().ind()] {
                Ok(tris[0].ind())
            } else if self.should_rem_tet[tris[1].opposite().tetrahedron().ind()] {
                Ok(tris[1].ind())
            } else if self.should_rem_tet[tris[2].opposite().tetrahedron().ind()] {
                Ok(tris[2].ind())
            } else if self.should_rem_tet[tris[3].opposite().tetrahedron().ind()] {
                Ok(tris[3].ind())
            } else {
                Err(anyhow::Error::msg("Isolated kept tetrahedron"))
            }
        } else {
            Err(anyhow::Error::msg("No kept tetrahedron"))
        }
    }

    fn external_neighbors(
        &self,
        boundary_tri: &mut Vec<usize>,
        boundary_nei_opt: &mut Vec<[Option<usize>; 3]>,
        ind_bnd_tri_1: usize,
    ) -> [usize; 3] {
        let cur_tri_bnd = IterHalfTriangle3::new(self.simplicial, boundary_tri[ind_bnd_tri_1]);
        let mut boundary_nei_1 = [0; 3];
        let he_1 = cur_tri_bnd.halfedges();
        // for each edge of the triangle, get neighbor triangle
        for subind_1 in 0..3 {
            // only if neighbor is not already set
            if boundary_nei_opt[ind_bnd_tri_1][subind_1].is_some() {
                continue;
            }
            // starting from opposite of the halfedge,
            // search from first triangle separating removed and non removed tetrahedra
            let mut he_cur = he_1[subind_1].opposite().neighbor().opposite();
            loop {
                // if opposite tetrahedron is to keep, break
                if !self.should_rem_tet[he_cur.halftriangle().tetrahedron().ind()] {
                    break;
                }
                he_cur = he_cur.neighbor().opposite();
            }

            // get triangle index (within simplicial)
            let ind_tri2 = he_cur.halftriangle().ind();
            // get halfedge index within triangle
            let subind_2 = he_cur.subindex();
            // if triangle is already in boundary, get its index, else add it
            let ind_bnd_tri_2 = if let Some((i2, _)) = boundary_tri
                .iter()
                .enumerate()
                .find(|(_, &ind)| ind == ind_tri2)
            {
                i2
            } else {
                boundary_tri.push(ind_tri2);
                boundary_nei_opt.push([None; 3]);
                boundary_tri.len() - 1
            };
            // add neighbor to both triangles
            boundary_nei_opt[ind_bnd_tri_1][subind_1] = Some(ind_bnd_tri_2);
            boundary_nei_opt[ind_bnd_tri_2][subind_2] = Some(ind_bnd_tri_1);
            boundary_nei_1[subind_1] = ind_bnd_tri_2;
        }
        boundary_nei_1
    }

    fn build_boundary_triangles_graph(
        &self,
        ind_tri_first: usize,
    ) -> (Vec<usize>, Vec<[usize; 3]>) {
        let mut boundary_tri = vec![ind_tri_first];
        let mut boundary_nei_opt: Vec<[Option<usize>; 3]> = vec![[None; 3]];
        let mut boundary_nei = Vec::new();
        let mut ind_bnd_tri_1 = 0;
        loop {
            let boundary_nei_1 =
                self.external_neighbors(&mut boundary_tri, &mut boundary_nei_opt, ind_bnd_tri_1);
            boundary_nei.push(boundary_nei_1);
            ind_bnd_tri_1 += 1;
            if ind_bnd_tri_1 >= boundary_tri.len() {
                break;
            }
        }

        (boundary_tri, boundary_nei)
    }

    /// BW insertion algorithm
    pub fn bw_insert_node(&mut self, node: usize) -> Result<Vec<usize>> {
        if self.tet_to_check.len() != 0 {
            return Err(anyhow::Error::msg(
                "Cannot insert node if all tetrahedra are not checked",
            ));
        }

        // 1 - find boundary triangle
        let ind_tri_first = self.find_first_boundary_triangle()?;

        // 2 - build boundary triangles graph
        let (vec_tri, vec_nei) = self.build_boundary_triangles_graph(ind_tri_first);

        // 3 - create each tetrahedra: triangle and added node
        let mut added_tets = Vec::new();
        for i in 0..vec_tri.len() {
            let cur_tri = IterHalfTriangle3::new(self.simplicial, vec_tri[i]);
            let [nod0, nod1, nod2] = cur_tri.node_values();

            let ind_tet = if let Some(ind_tet_replace) = self.tet_to_rem.pop() {
                self.simplicial.unset_tetrahedron(ind_tet_replace);
                ind_tet_replace
            } else {
                self.simplicial.add_empty_tetrahedron()
            };

            self.simplicial
                .set_tetrahedron(ind_tet, node, nod0, nod1, nod2);

            added_tets.push(ind_tet);
        }

        // 4 - create links
        for i in 0..vec_tri.len() {
            let (tri0, tri1, tri2, tri3) = (
                added_tets[i] * 4,
                added_tets[i] * 4 + 1,
                added_tets[i] * 4 + 2,
                added_tets[i] * 4 + 3,
            );

            let ind_tri_nei = vec_tri[i];

            self.simplicial.oppose_halftriangles(
                tri0,
                ind_tri_nei,
                ShiftType::ABC2CBA,
                ShiftType::ABC2CBA,
            );

            let ind_nei_0 = vec_nei[i][0];
            let ind_nei_1 = vec_nei[i][1];
            let ind_nei_2 = vec_nei[i][2];

            let ind_tet_nei_0 = added_tets[ind_nei_0];
            let ind_tet_nei_1 = added_tets[ind_nei_1];
            let ind_tet_nei_2 = added_tets[ind_nei_2];

            self.simplicial
                .oppose_halftriangles_auto(tri1, ind_tet_nei_0)?;
            self.simplicial
                .oppose_halftriangles_auto(tri2, ind_tet_nei_1)?;
            self.simplicial
                .oppose_halftriangles_auto(tri3, ind_tet_nei_2)?;
            todo!()
        }

        loop {
            if let Some(ind_tetra_keep) = self.tet_to_keep.pop() {
                self.should_keep_tet[ind_tetra_keep] = false;
            } else {
                break;
            }
        }

        Ok(added_tets)
    }
}