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

/// Checks validity of simplicial graph
pub fn simplicial3_is_valid(simplicial: &Simplicial3) -> Result<bool> {
    let mut valid = true;

    for he in simplicial.get_all_halfedges().iter() {
        valid = valid && halfedge3_is_valid(&he);
    }

    Ok(valid)
}
