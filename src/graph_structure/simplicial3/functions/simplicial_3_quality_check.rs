use anyhow::Result;

pub use super::super::IterHalfEdge3;
pub use super::super::Simplicial3;

/// Checks halfedge validity
pub fn halfedge3_is_valid(halfedge: &IterHalfEdge3) -> bool {
    let first_node = halfedge.first_node();
    let last_node = halfedge.last_node();

    let he_next = halfedge.next();
    let he_prev = halfedge.prev();
    let he_opp = halfedge.opposite();
    let he_nei = halfedge.neighbor();

    let mut valid = true;

    if he_next.first_node().value() != last_node.value() {
        log::error!("{}: Wrong next halfedge", halfedge.to_string());
        valid = false;
    }
    if he_next.halftriangle().node_values() != halfedge.halftriangle().node_values() {
        log::error!(
            "{}: Wrong halftriangle for next hafledge",
            halfedge.to_string()
        );
        valid = false;
    }
    if he_next.tetrahedron().node_values() != halfedge.tetrahedron().node_values() {
        log::error!(
            "{}: Wrong tetrahedron for next hafledge",
            halfedge.to_string()
        );
        valid = false;
    }

    if he_prev.last_node().value() != first_node.value() {
        log::error!("{}: Wrong previous halfedge", halfedge.to_string());
        valid = false;
    }
    if he_prev.halftriangle().node_values() != halfedge.halftriangle().node_values() {
        log::error!(
            "{}: Wrong halftriangle for prev hafledge",
            halfedge.to_string()
        );
        valid = false;
    }
    if he_prev.tetrahedron().node_values() != halfedge.tetrahedron().node_values() {
        log::error!(
            "{}: Wrong tetrahedron for prev hafledge",
            halfedge.to_string()
        );
        valid = false;
    }

    if he_opp.first_node().value() != last_node.value()
        || he_opp.last_node().value() != first_node.value()
    {
        log::error!("{}: Wrong opposite halfedge", halfedge.to_string());
        valid = false;
    }
    if he_opp.tetrahedron().node_values() == halfedge.tetrahedron().node_values() {
        log::error!(
            "{}: Wrong tetrahedron for oppoite hafledge",
            halfedge.to_string()
        );
        valid = false;
    }

    if he_nei.first_node().value() != last_node.value()
        || he_nei.last_node().value() != first_node.value()
    {
        log::error!("{}: Wrong neighbor halfedge", halfedge.to_string());
        valid = false;
    }
    if he_nei.halftriangle().node_values() == halfedge.halftriangle().node_values() {
        log::error!(
            "{}: Wrong halftriangle for neighbor hafledge",
            halfedge.to_string()
        );
        valid = false;
    }
    if he_nei.tetrahedron().node_values() != halfedge.tetrahedron().node_values() {
        log::error!(
            "{}: Wrong tetrahedron for neighbor hafledge",
            halfedge.to_string()
        );
        valid = false;
    }

    valid
}

/// Checks node indices within simplicial
pub fn check_node_indices(simplicial: &Simplicial3) -> bool {
    let mut valid = true;
    let mut nod_indices = Vec::new();

    for tetra in simplicial.get_all_tetrahedra().iter() {
        let ind_tet = tetra.ind();
        let [n0, n1, n2, n3] = tetra.node_values();
        let nod_max = std::cmp::max(n0, std::cmp::max(n1, std::cmp::max(n2, n3)));
        if nod_indices.len() <= nod_max {
            nod_indices.resize(nod_max + 1, Vec::new());
        }

        nod_indices[n0].push((ind_tet << 2) + 0);
        nod_indices[n1].push((ind_tet << 2) + 1);
        nod_indices[n2].push((ind_tet << 2) + 2);
        nod_indices[n3].push((ind_tet << 2) + 3);
    }

    for (node_value, vec) in nod_indices.iter().enumerate() {
        let nod_opt = simplicial.find_node(node_value);
        if vec.len() == 0 {
            if nod_opt.is_some() {
                valid = false;
                break;
            } else {
                continue;
            }
        }

        if let Some(nod) = nod_opt {
            let mut ind_he = nod
                .halfedges()
                .iter()
                .map(|he| he.ind_first())
                .collect::<Vec<_>>();
            ind_he.sort();
            ind_he.dedup();
            let mut vec_clone = vec.clone();
            vec_clone.sort();
            vec_clone.dedup();
            if ind_he != vec_clone {
                valid = false;
                break;
            }
        } else {
            valid = false;
            break;
        }
    }

    valid
}

/// Checks validity of simplicial graph
pub fn simplicial3_is_valid(simplicial: &Simplicial3) -> Result<bool> {
    let mut valid = true;

    for he in simplicial.get_all_halfedges().iter() {
        valid = valid && halfedge3_is_valid(&he);
    }

    valid = valid && check_node_indices(simplicial);

    Ok(valid)
}
