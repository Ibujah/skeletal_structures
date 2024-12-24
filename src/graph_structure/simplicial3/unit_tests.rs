#[cfg(test)]
mod simplicial3_test {
    use anyhow::Result;
    use rstest::rstest;

    use crate::graph_structure::simplicial3::{
        BowyerWatsonInserter, IterHalfTriangle3, Simplicial3,
    };

    fn test_triangle(triabc: IterHalfTriangle3, a: usize, b: usize, c: usize) -> () {
        assert!(triabc.node_values() == [a, b, c]);
        let [heab, hebc, heca] = triabc.halfedges();

        assert!(heab.node_values() == [a, b]);
        assert!(hebc.node_values() == [b, c]);
        assert!(heca.node_values() == [c, a]);

        assert!(heab.next().node_values() == [b, c]);
        assert!(hebc.next().node_values() == [c, a]);
        assert!(heca.next().node_values() == [a, b]);

        assert!(heab.next().next().next().node_values() == [a, b]);
        assert!(hebc.next().next().next().node_values() == [b, c]);
        assert!(heca.next().next().next().node_values() == [c, a]);

        assert!(heab.prev().node_values() == [c, a]);
        assert!(hebc.prev().node_values() == [a, b]);
        assert!(heca.prev().node_values() == [b, c]);

        assert!(heab.prev().prev().prev().node_values() == [a, b]);
        assert!(hebc.prev().prev().prev().node_values() == [b, c]);
        assert!(heca.prev().prev().prev().node_values() == [c, a]);

        assert!(heab.neighbor().node_values() == [b, a]);
        assert!(hebc.neighbor().node_values() == [c, b]);
        assert!(heca.neighbor().node_values() == [a, c]);

        assert!(heab.neighbor().neighbor().node_values() == [a, b]);
        assert!(hebc.neighbor().neighbor().node_values() == [b, c]);
        assert!(heca.neighbor().neighbor().node_values() == [c, a]);

        assert!(heab.opposite().node_values() == [b, a]);
        assert!(hebc.opposite().node_values() == [c, b]);
        assert!(heca.opposite().node_values() == [a, c]);
    }

    #[rstest]
    #[case(0, 1, 2, 3)]
    #[case(10, 11, 12, 13)]
    fn neighbor_test(
        #[case] n0: usize,
        #[case] n1: usize,
        #[case] n2: usize,
        #[case] n3: usize,
    ) -> Result<()> {
        let mut simpl = Simplicial3::new(false);

        let [ind_tetra0, _] = simpl.first_tetrahedron([n0, n1, n2, n3])?;

        let tetra0 = simpl.get_tetrahedron_from_index(ind_tetra0)?;

        assert!(simpl.get_nb_tetrahedra() == 2);

        let [tri321, tri230, tri103, tri012] = tetra0.halftriangles();

        test_triangle(tri321, n3, n2, n1);
        test_triangle(tri230, n2, n3, n0);
        test_triangle(tri103, n1, n0, n3);
        test_triangle(tri012, n0, n1, n2);

        Ok(())
    }

    #[rstest]
    #[case(true, 0, 1, 2, 3)]
    #[case(true, 10, 11, 12, 13)]
    #[case(false, 0, 1, 2, 3)]
    #[case(false, 10, 11, 12, 13)]
    fn getter_test(
        #[case] register_node_halfedges: bool,
        #[case] n0: usize,
        #[case] n1: usize,
        #[case] n2: usize,
        #[case] n3: usize,
    ) -> Result<()> {
        let mut simpl = Simplicial3::new(register_node_halfedges);

        simpl.first_tetrahedron([n0, n1, n2, n3])?;

        let nod1 = if let Some(nod) = simpl.find_node(n1) {
            nod
        } else {
            return Err(anyhow::anyhow!("Node not found"));
        };
        assert!(nod1.value() == n1);

        let he_from_1 = nod1.halfedges();
        let he_from_1_values = he_from_1
            .iter()
            .map(|he| he.node_values())
            .collect::<Vec<_>>();
        let mut tet_from_1_values = he_from_1
            .iter()
            .map(|he| he.tetrahedron().node_values())
            .collect::<Vec<_>>();
        tet_from_1_values.sort();
        tet_from_1_values.dedup();
        assert!(he_from_1_values.len() == 6);
        assert!(he_from_1_values.contains(&[n1, n0]));
        assert!(he_from_1_values.contains(&[n1, n2]));
        assert!(he_from_1_values.contains(&[n1, n3]));
        assert!(tet_from_1_values.len() == 2);

        let he20 = if let Some(he) = simpl.find_halfedge(n2, n0) {
            he
        } else {
            return Err(anyhow::anyhow!("Halfedge not found"));
        };
        assert!(he20.node_values() == [n2, n0]);

        let tri310 = if let Some(tri) = simpl.find_halftriangle(n3, n1, n0) {
            tri
        } else {
            return Err(anyhow::anyhow!("Halftriangle not found"));
        };
        assert!(
            tri310.node_values() == [n3, n1, n0]
                || tri310.node_values() == [n1, n0, n3]
                || tri310.node_values() == [n0, n3, n1]
        );

        let tet0231 = if let Some(tet) = simpl.find_tetrahedron(n0, n2, n3, n1) {
            tet
        } else {
            return Err(anyhow::anyhow!("Tetrahedron not found"));
        };
        assert!(tet0231.node_values().contains(&n0));
        assert!(tet0231.node_values().contains(&n2));
        assert!(tet0231.node_values().contains(&n3));
        assert!(tet0231.node_values().contains(&n1));
        Ok(())
    }

    #[test]
    fn insert_bw_test() -> Result<()> {
        let mut simpl = Simplicial3::new(false);

        simpl.first_tetrahedron([0, 1, 2, 3])?;

        let mut bw_inserter = BowyerWatsonInserter::new(&mut simpl, 1);

        while let Some(_) = bw_inserter.bw_tetra_to_check() {
            bw_inserter.bw_keep_tetra()?;
        }

        bw_inserter.bw_insert_node(4)?;

        Ok(())
    }
}
