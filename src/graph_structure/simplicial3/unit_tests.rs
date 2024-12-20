#[cfg(test)]
mod simplicial3_test {
    use anyhow::Result;

    use crate::graph_structure::simplicial3::IterHalfTriangle3;

    use super::super::structure::Simplicial3;

    fn test_triangle(triabc: IterHalfTriangle3, a: usize, b: usize, c: usize) -> () {
        assert!(triabc.node_indices() == [a, b, c]);
        let [heab, hebc, heca] = triabc.halfedges();

        assert!(heab.node_indices() == [a, b]);
        assert!(hebc.node_indices() == [b, c]);
        assert!(heca.node_indices() == [c, a]);

        assert!(heab.next().node_indices() == [b, c]);
        assert!(hebc.next().node_indices() == [c, a]);
        assert!(heca.next().node_indices() == [a, b]);

        assert!(heab.next().next().next().node_indices() == [a, b]);
        assert!(hebc.next().next().next().node_indices() == [b, c]);
        assert!(heca.next().next().next().node_indices() == [c, a]);

        assert!(heab.prev().node_indices() == [c, a]);
        assert!(hebc.prev().node_indices() == [a, b]);
        assert!(heca.prev().node_indices() == [b, c]);

        assert!(heab.prev().prev().prev().node_indices() == [a, b]);
        assert!(hebc.prev().prev().prev().node_indices() == [b, c]);
        assert!(heca.prev().prev().prev().node_indices() == [c, a]);

        assert!(heab.neighbor().node_indices() == [b, a]);
        assert!(hebc.neighbor().node_indices() == [c, b]);
        assert!(heca.neighbor().node_indices() == [a, c]);

        assert!(heab.neighbor().neighbor().node_indices() == [a, b]);
        assert!(hebc.neighbor().neighbor().node_indices() == [b, c]);
        assert!(heca.neighbor().neighbor().node_indices() == [c, a]);
    }

    #[test]
    fn insert_test() -> Result<()> {
        let mut simpl = Simplicial3::new(false);

        let [ind_tetra0, _] = simpl.first_tetrahedron([0, 1, 2, 3])?;

        let tetra0 = simpl.get_tetrahedron_from_index(ind_tetra0)?;

        assert!(simpl.get_nb_tetrahedra() == 2);

        let [tri321, tri230, tri103, tri012] = tetra0.halftriangles();

        test_triangle(tri321, 3, 2, 1);
        test_triangle(tri230, 2, 3, 0);
        test_triangle(tri103, 1, 0, 3);
        test_triangle(tri012, 0, 1, 2);

        Ok(())
    }
}
