use anyhow::Result;

use super::Simplicial2;
use std::cmp::max;

/////////////////////////////
/// Private build methods ///
/////////////////////////////

fn add_empty_triangle(simpl: &mut Simplicial2) -> usize {
    simpl
        .halfedge_first_node
        .resize(simpl.halfedge_first_node.len() + 3, 0);
    simpl
        .halfedge_opposite
        .resize(simpl.halfedge_opposite.len() + 3, 0);

    simpl.nb_triangles = simpl.nb_triangles + 1;

    simpl.nb_triangles - 1
}

fn set_triangle(
    simpl: &mut Simplicial2,
    ind_tri: usize,
    nod1: usize,
    nod2: usize,
    nod3: usize,
) -> [usize; 3] {
    let ind_first = ind_tri * 3;
    simpl.halfedge_first_node[ind_first] = nod1;
    simpl.halfedge_first_node[ind_first + 1] = nod2;
    simpl.halfedge_first_node[ind_first + 2] = nod3;

    if let Some(vec) = simpl.node_halfedges.as_mut() {
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

fn unset_triangle(simpl: &mut Simplicial2, ind_tri: usize) -> usize {
    if let Some(vec) = simpl.node_halfedges.as_mut() {
        let ind_first = ind_tri * 3;
        let nod1 = simpl.halfedge_first_node[ind_first];
        let nod2 = simpl.halfedge_first_node[ind_first + 1];
        let nod3 = simpl.halfedge_first_node[ind_first + 2];

        vec[nod1].retain(|&ind_he| ind_he != ind_first);
        vec[nod2].retain(|&ind_he| ind_he != (ind_first + 1));
        vec[nod3].retain(|&ind_he| ind_he != (ind_first + 2));
    }
    ind_tri
}

fn oppose_halfedges(simpl: &mut Simplicial2, he0: usize, he1: usize) {
    simpl.halfedge_opposite[he0] = he1;
    simpl.halfedge_opposite[he1] = he0;
}

////////////////////////////////
/// Public modifying methods ///
////////////////////////////////

/// Replace node value by new value
pub fn replace_node_value(
    simpl: &mut Simplicial2,
    old_node_value: usize,
    new_node_value: usize,
) -> Result<()> {
    // check if new value is not already in the simplicial, and store old_value indices
    let mut old_val_ind = Vec::new();
    for i in 0..simpl.halfedge_first_node.len() {
        if simpl.halfedge_first_node[i] == new_node_value {
            return Err(anyhow::Error::msg("New node value already in simplicial"));
        }
        if simpl.halfedge_first_node[i] == old_node_value {
            old_val_ind.push(i);
        }
    }
    // if old value is not in the simplicial, return
    if old_val_ind.is_empty() {
        return Ok(());
    }

    for ind in old_val_ind {
        simpl.halfedge_first_node[ind] = new_node_value;
    }

    if let Some(vec) = &mut simpl.node_halfedges {
        let nod_hedg = vec[old_node_value].clone();

        vec[old_node_value].clear();

        if vec.len() <= new_node_value {
            vec.resize(new_node_value + 1, Vec::new());
        }
        vec[new_node_value] = nod_hedg;
    }

    Ok(())
}

/// Inserts first triangle, and its opposite, in the structure
pub fn insert_first_triangle(simpl: &mut Simplicial2, nodes: [usize; 3]) -> Result<[usize; 2]> {
    if simpl.nb_triangles != 0 {
        return Err(anyhow::Error::msg("Already triangles in simplicial"));
    }

    let [n0, n1, n2] = nodes;

    let ind_tri0 = add_empty_triangle(simpl);
    let ind_tri1 = add_empty_triangle(simpl);

    let [h01, h12, h20] = set_triangle(simpl, ind_tri0, n0, n1, n2);
    let [h02, h21, h10] = set_triangle(simpl, ind_tri1, n0, n2, n1);

    oppose_halfedges(simpl, h01, h10);
    oppose_halfedges(simpl, h12, h21);
    oppose_halfedges(simpl, h20, h02);

    Ok([ind_tri0, ind_tri1])
}

/// Inserts a new node in a triangle
pub fn insert_node_within_triangle(
    simpl: &mut Simplicial2,
    node: usize,
    ind_tri: usize,
) -> Result<[usize; 3]> {
    let [n0, n1, n2] = simpl.triangle_node_values(ind_tri);
    let [h01, h12, h20] = simpl.triangle_halfedge_indices(ind_tri);
    let h10 = simpl.halfedge_opposite_index(h01);
    let h21 = simpl.halfedge_opposite_index(h12);
    let h02 = simpl.halfedge_opposite_index(h20);

    let ind_tri0 = unset_triangle(simpl, ind_tri);
    let ind_tri1 = add_empty_triangle(simpl);
    let ind_tri2 = add_empty_triangle(simpl);

    let [h01, h1n, hn0] = set_triangle(simpl, ind_tri0, n0, n1, node);
    let [h12, h2n, hn1] = set_triangle(simpl, ind_tri1, n1, n2, node);
    let [h20, h0n, hn2] = set_triangle(simpl, ind_tri2, n2, n0, node);

    oppose_halfedges(simpl, h10, h01);
    oppose_halfedges(simpl, h12, h21);
    oppose_halfedges(simpl, h20, h02);

    oppose_halfedges(simpl, h0n, hn0);
    oppose_halfedges(simpl, h1n, hn1);
    oppose_halfedges(simpl, h2n, hn2);

    Ok([ind_tri, simpl.nb_triangles - 2, simpl.nb_triangles - 1])
}

/// Flips halfedge
pub fn flip_halfedge(simpl: &mut Simplicial2, ind_he: usize) -> Result<[usize; 2]> {
    let ind_he_opp = simpl.halfedge_opposite_index(ind_he);

    // ind_he is ca
    let hab = simpl.halfedge_next_index(ind_he);
    let hbc = simpl.halfedge_previous_index(ind_he);

    // ind_he_opp is ac
    let hcd = simpl.halfedge_next_index(ind_he_opp);
    let hda = simpl.halfedge_previous_index(ind_he_opp);

    let na = simpl.halfedge_first_node_value(hab);
    let nb = simpl.halfedge_first_node_value(hbc);
    let nc = simpl.halfedge_first_node_value(hcd);
    let nd = simpl.halfedge_first_node_value(hda);

    let hba = simpl.halfedge_opposite_index(hab);
    let hcb = simpl.halfedge_opposite_index(hbc);
    let hdc = simpl.halfedge_opposite_index(hcd);
    let had = simpl.halfedge_opposite_index(hda);

    let ind_tri1 = simpl.halfedge_triangle_index(ind_he);
    let ind_tri2 = simpl.halfedge_triangle_index(ind_he_opp);

    let [hbc, hcd, hdb] = set_triangle(simpl, ind_tri1, nb, nc, nd);
    let [hda, hab, hbd] = set_triangle(simpl, ind_tri2, nd, na, nb);

    oppose_halfedges(simpl, hab, hba);
    oppose_halfedges(simpl, hbc, hcb);
    oppose_halfedges(simpl, hcd, hdc);
    oppose_halfedges(simpl, hda, had);

    oppose_halfedges(simpl, hbd, hdb);

    Ok([hbd, hdb])
}

/// Builds full simplicial from set of triangles
pub fn build_from_triangle_list(
    triangles: Vec<[usize; 3]>,
    register_node_halfedges: bool,
) -> Result<Simplicial2> {
    let mut simpl = Simplicial2::new(register_node_halfedges);

    for &[nod0, nod1, nod2] in triangles.iter() {
        let ind_tri = add_empty_triangle(&mut simpl);
        set_triangle(&mut simpl, ind_tri, nod0, nod1, nod2);
    }

    let mut to_attribute: Vec<usize> = (0..simpl.get_nb_halfedges()).collect();
    while let Some(ind_he) = to_attribute.pop() {
        let n0 = simpl.halfedge_first_node_value(ind_he);
        let n1 = simpl.halfedge_last_node_value(ind_he);
        let mut found = false;
        for i in 0..to_attribute.len() {
            let ind_he_opp = to_attribute[i];
            let n0o = simpl.halfedge_first_node_value(ind_he_opp);
            let n1o = simpl.halfedge_last_node_value(ind_he_opp);
            if n0 == n1o && n1 == n0o {
                oppose_halfedges(&mut simpl, ind_he, ind_he_opp);
                to_attribute.remove(i);
                found = true;
                break;
            }
        }
        if !found {
            return Err(anyhow::Error::msg(
                "Given faces do not form a manifold simpl",
            ));
        }
    }

    Ok(simpl)
}
