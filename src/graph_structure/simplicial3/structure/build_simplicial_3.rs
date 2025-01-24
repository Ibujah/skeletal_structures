use anyhow::Result;
use std::cmp::max;

use super::{
    simplicial_3::ABC2ACB, simplicial_3::ABC2BAC, simplicial_3::ABC2CBA, IterHalfTriangle3,
    Simplicial3,
};

/////////////////////////////
/// Private build methods ///
/////////////////////////////

pub(super) fn add_empty_tetrahedron(simpl3: &mut Simplicial3) -> usize {
    simpl3.tet_nodes.resize(simpl3.tet_nodes.len() + 4, 0);
    simpl3
        .halftriangle_opposite
        .resize(simpl3.halftriangle_opposite.len() + 4, 0);
    simpl3
        .halftriangle_shift
        .resize(simpl3.halftriangle_shift.len() + 4, 3);

    simpl3.nb_tetrahedra = simpl3.nb_tetrahedra + 1;

    simpl3.nb_tetrahedra - 1
}

pub(super) fn unset_tetrahedron(simpl3: &mut Simplicial3, ind_tet: usize) {
    let ind_first = ind_tet << 2;

    let nod0 = simpl3.tet_nodes[ind_first];
    let nod1 = simpl3.tet_nodes[ind_first + 1];
    let nod2 = simpl3.tet_nodes[ind_first + 2];
    let nod3 = simpl3.tet_nodes[ind_first + 3];

    simpl3.tet_nodes[ind_first] = 0;
    simpl3.tet_nodes[ind_first + 1] = 0;
    simpl3.tet_nodes[ind_first + 2] = 0;
    simpl3.tet_nodes[ind_first + 3] = 0;

    simpl3.halftriangle_opposite[ind_first] = 0;
    simpl3.halftriangle_opposite[ind_first + 1] = 0;
    simpl3.halftriangle_opposite[ind_first + 2] = 0;
    simpl3.halftriangle_opposite[ind_first + 3] = 0;

    simpl3.halftriangle_shift[ind_first] = 3;
    simpl3.halftriangle_shift[ind_first + 1] = 3;
    simpl3.halftriangle_shift[ind_first + 2] = 3;
    simpl3.halftriangle_shift[ind_first + 3] = 3;

    if let Some(vec) = simpl3.node_positions.as_mut() {
        vec[nod0].retain(|&x| x >> 2 != ind_tet);
        vec[nod1].retain(|&x| x >> 2 != ind_tet);
        vec[nod2].retain(|&x| x >> 2 != ind_tet);
        vec[nod3].retain(|&x| x >> 2 != ind_tet);
    }
}

pub(super) fn set_tetrahedron(
    simpl3: &mut Simplicial3,
    ind_tet: usize,
    nod1: usize,
    nod2: usize,
    nod3: usize,
    nod4: usize,
) -> [usize; 4] {
    let ind_first = ind_tet * 4;
    simpl3.tet_nodes[ind_first] = nod1;
    simpl3.tet_nodes[ind_first + 1] = nod2;
    simpl3.tet_nodes[ind_first + 2] = nod3;
    simpl3.tet_nodes[ind_first + 3] = nod4;

    if let Some(vec) = simpl3.node_positions.as_mut() {
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

pub(super) fn remove_tetrahedron(simpl3: &mut Simplicial3, ind_tetra: usize) -> Result<()> {
    unset_tetrahedron(simpl3, ind_tetra);

    if ind_tetra != simpl3.nb_tetrahedra - 1 {
        let ind_tri_opp1 = simpl3.halftriangle_opposite[simpl3.halftriangle_opposite.len() - 4];
        let ind_tri_opp2 = simpl3.halftriangle_opposite[simpl3.halftriangle_opposite.len() - 3];
        let ind_tri_opp3 = simpl3.halftriangle_opposite[simpl3.halftriangle_opposite.len() - 2];
        let ind_tri_opp4 = simpl3.halftriangle_opposite[simpl3.halftriangle_opposite.len() - 1];

        let shift_tri_opp1 = simpl3.halftriangle_shift[simpl3.halftriangle_shift.len() - 4];
        let shift_tri_opp2 = simpl3.halftriangle_shift[simpl3.halftriangle_shift.len() - 3];
        let shift_tri_opp3 = simpl3.halftriangle_shift[simpl3.halftriangle_shift.len() - 2];
        let shift_tri_opp4 = simpl3.halftriangle_shift[simpl3.halftriangle_shift.len() - 1];

        let [nod1, nod2, nod3, nod4] = simpl3
            .get_tetrahedron_from_index(simpl3.nb_tetrahedra - 1)?
            .node_values();

        let [ind_tri1, ind_tri2, ind_tri3, ind_tri4] =
            set_tetrahedron(simpl3, ind_tetra, nod1, nod2, nod3, nod4);

        simpl3.halftriangle_opposite[ind_tri1] = ind_tri_opp1;
        simpl3.halftriangle_opposite[ind_tri2] = ind_tri_opp2;
        simpl3.halftriangle_opposite[ind_tri3] = ind_tri_opp3;
        simpl3.halftriangle_opposite[ind_tri4] = ind_tri_opp4;

        simpl3.halftriangle_shift[ind_tri1] = shift_tri_opp1;
        simpl3.halftriangle_shift[ind_tri2] = shift_tri_opp2;
        simpl3.halftriangle_shift[ind_tri3] = shift_tri_opp3;
        simpl3.halftriangle_shift[ind_tri4] = shift_tri_opp4;

        simpl3.halftriangle_opposite[ind_tri_opp1] = ind_tri1;
        simpl3.halftriangle_opposite[ind_tri_opp2] = ind_tri2;
        simpl3.halftriangle_opposite[ind_tri_opp3] = ind_tri3;
        simpl3.halftriangle_opposite[ind_tri_opp4] = ind_tri4;
    }

    simpl3.tet_nodes.pop();
    simpl3.tet_nodes.pop();
    simpl3.tet_nodes.pop();
    simpl3.tet_nodes.pop();

    simpl3.halftriangle_opposite.pop();
    simpl3.halftriangle_opposite.pop();
    simpl3.halftriangle_opposite.pop();
    simpl3.halftriangle_opposite.pop();

    simpl3.halftriangle_shift.pop();
    simpl3.halftriangle_shift.pop();
    simpl3.halftriangle_shift.pop();
    simpl3.halftriangle_shift.pop();

    simpl3.nb_tetrahedra = simpl3.nb_tetrahedra - 1;

    Ok(())
}

pub(super) fn oppose_halftriangles(
    simpl3: &mut Simplicial3,
    htri0: usize,
    htri1: usize,
    shift_tri: usize,
) {
    simpl3.halftriangle_opposite[htri0] = htri1;
    simpl3.halftriangle_opposite[htri1] = htri0;
    simpl3.halftriangle_shift[htri0] = shift_tri;
    simpl3.halftriangle_shift[htri1] = shift_tri;
}

pub(super) fn oppose_halftriangles_auto(
    simpl3: &mut Simplicial3,
    htri0: usize,
    htri1: usize,
) -> Result<()> {
    let tri0 = IterHalfTriangle3::new(simpl3, htri0);
    let tri1 = IterHalfTriangle3::new(simpl3, htri1);

    let [a, _, _] = tri0.node_values();
    let [d, e, f] = tri1.node_values();

    let shift = if a == d {
        ABC2ACB
    } else if a == e {
        ABC2BAC
    } else if a == f {
        ABC2CBA
    } else {
        return Err(anyhow::Error::msg("Faces are not opposite"));
    };

    oppose_halftriangles(simpl3, htri0, htri1, shift);

    Ok(())
}

////////////////////////////////
/// Public modifying methods ///
////////////////////////////////

/// Inserts a first tetrahedron in the structure
pub fn first_tetrahedron(simpl3: &mut Simplicial3, nodes: [usize; 4]) -> Result<[usize; 2]> {
    if simpl3.nb_tetrahedra != 0 {
        return Err(anyhow::Error::msg("Already tetrahedra in simplicial"));
    }
    let [n0, n1, n2, n3] = nodes;

    let ind_tet0 = add_empty_tetrahedron(simpl3);
    let ind_tet1 = add_empty_tetrahedron(simpl3);

    let [t321, t230, t103, t012] = set_tetrahedron(simpl3, ind_tet0, n0, n1, n2, n3);
    let [t032, t301, t210, t123] = set_tetrahedron(simpl3, ind_tet1, n1, n2, n3, n0);

    oppose_halftriangles(simpl3, t321, t123, ABC2CBA);
    oppose_halftriangles(simpl3, t230, t032, ABC2CBA);
    oppose_halftriangles(simpl3, t103, t301, ABC2CBA);
    oppose_halftriangles(simpl3, t012, t210, ABC2CBA);

    Ok([ind_tet0, ind_tet1])
}
