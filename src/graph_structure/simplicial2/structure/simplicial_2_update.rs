use anyhow::Result;

use super::Simplicial2;

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

    let ind_tri0 = simpl.add_empty_triangle();
    let ind_tri1 = simpl.add_empty_triangle();

    let [h01, h12, h20] = simpl.set_triangle(ind_tri0, n0, n1, n2);
    let [h02, h21, h10] = simpl.set_triangle(ind_tri1, n0, n2, n1);

    simpl.oppose_halfedges(h01, h10);
    simpl.oppose_halfedges(h12, h21);
    simpl.oppose_halfedges(h20, h02);

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

    let ind_tri0 = simpl.unset_triangle(ind_tri);
    let ind_tri1 = simpl.add_empty_triangle();
    let ind_tri2 = simpl.add_empty_triangle();

    let [h01, h1n, hn0] = simpl.set_triangle(ind_tri0, n0, n1, node);
    let [h12, h2n, hn1] = simpl.set_triangle(ind_tri1, n1, n2, node);
    let [h20, h0n, hn2] = simpl.set_triangle(ind_tri2, n2, n0, node);

    simpl.oppose_halfedges(h10, h01);
    simpl.oppose_halfedges(h12, h21);
    simpl.oppose_halfedges(h20, h02);

    simpl.oppose_halfedges(h0n, hn0);
    simpl.oppose_halfedges(h1n, hn1);
    simpl.oppose_halfedges(h2n, hn2);

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

    let [hbc, hcd, hdb] = simpl.set_triangle(ind_tri1, nb, nc, nd);
    let [hda, hab, hbd] = simpl.set_triangle(ind_tri2, nd, na, nb);

    simpl.oppose_halfedges(hab, hba);
    simpl.oppose_halfedges(hbc, hcb);
    simpl.oppose_halfedges(hcd, hdc);
    simpl.oppose_halfedges(hda, had);

    simpl.oppose_halfedges(hbd, hdb);

    Ok([hbd, hdb])
}

/// Builds full simplicial from set of triangles
pub fn insert_triangle_list(simpl: &mut Simplicial2, triangles: Vec<[usize; 3]>) -> Result<()> {
    if simpl.get_nb_triangles() != 0 {
        return Err(anyhow::Error::msg("Simplicial should be empty"));
    }
    for &[nod0, nod1, nod2] in triangles.iter() {
        let ind_tri = simpl.add_empty_triangle();
        simpl.set_triangle(ind_tri, nod0, nod1, nod2);
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
                simpl.oppose_halfedges(ind_he, ind_he_opp);
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

    Ok(())
}
