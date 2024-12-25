use anyhow::Result;

pub use super::super::IterHalfEdge2;
pub use super::super::Simplicial2;

/// Checks halfedge validity
pub fn halfedge2_is_valid(halfedge: &IterHalfEdge2) -> bool {
    let first_node = halfedge.first_node();
    let last_node = halfedge.last_node();

    let he_next = halfedge.next();
    let he_prev = halfedge.previous();
    let he_opp = halfedge.opposite();

    let mut valid = true;

    if he_next.first_node().value() != last_node.value() {
        log::error!("{}: Wrong next halfedge", halfedge.to_string());
        valid = false;
    }
    if he_prev.last_node().value() != first_node.value() {
        log::error!("{}: Wrong previous halfedge", halfedge.to_string());
        valid = false;
    }

    if he_opp.first_node().value() != last_node.value()
        || he_opp.last_node().value() != first_node.value()
    {
        log::error!("{}: Wrong opposite halfedge", halfedge.to_string());
        valid = false;
    }

    valid
}

/// Checks validity of simplicial graph
pub fn simplicial2_is_valid(simplicial: &Simplicial2) -> Result<bool> {
    let mut valid = true;

    for he in simplicial.get_all_halfedges().iter() {
        valid = valid && halfedge2_is_valid(&he);
    }

    Ok(valid)
}
