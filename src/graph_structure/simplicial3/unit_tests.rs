#[cfg(test)]
mod simplicial3_test {
    use anyhow::Result;
    use rstest::rstest;

    use crate::graph_structure::simplicial3::IterHalfTriangle3;

    use super::super::structure::Simplicial3;

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

    #[test]
    fn neighbor_test() -> Result<()> {
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

    #[rstest]
    #[case(true)]
    #[case(false)]
    fn getter_test(#[case] register_node_halfedges: bool) -> Result<()> {
        let mut simpl = Simplicial3::new(register_node_halfedges);

        simpl.first_tetrahedron([0, 1, 2, 3])?;

        let nod1 = if let Some(nod) = simpl.find_node(1) {
            nod
        } else {
            return Err(anyhow::anyhow!("Node not found"));
        };
        assert!(nod1.value() == 1);

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
        assert!(he_from_1_values.contains(&[1, 0]));
        assert!(he_from_1_values.contains(&[1, 2]));
        assert!(he_from_1_values.contains(&[1, 3]));
        assert!(tet_from_1_values.len() == 2);

        let he20 = if let Some(he) = simpl.find_halfedge(2, 0) {
            he
        } else {
            return Err(anyhow::anyhow!("Halfedge not found"));
        };
        assert!(he20.node_values() == [2, 0]);

        let tri310 = if let Some(tri) = simpl.find_halftriangle(3, 1, 0) {
            tri
        } else {
            return Err(anyhow::anyhow!("Halftriangle not found"));
        };
        assert!(
            tri310.node_values() == [3, 1, 0]
                || tri310.node_values() == [1, 0, 3]
                || tri310.node_values() == [0, 3, 1]
        );

        let tet0231 = if let Some(tet) = simpl.find_tetrahedron(0, 2, 3, 1) {
            tet
        } else {
            return Err(anyhow::anyhow!("Tetrahedron not found"));
        };
        assert!(tet0231.node_values().contains(&0));
        assert!(tet0231.node_values().contains(&2));
        assert!(tet0231.node_values().contains(&3));
        assert!(tet0231.node_values().contains(&1));
        Ok(())
    }
}
