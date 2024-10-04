#[cfg(test)]
mod simplicial2_test {
    use anyhow::Result;

    use crate::simplicial2::simplicial_is_valid;
    use crate::simplicial2::Simplicial2;

    #[test]
    fn insert_test() -> Result<()> {
        let mut simpl = Simplicial2::new(false);

        let [_, ind_tri1] = simpl.insert_first_triangle([0, 1, 2])?;

        assert!(simplicial_is_valid(&simpl)?);
        assert!(simpl.get_nb_triangles() == 2);
        assert!(simpl.find_triangle(0, 1, 2).is_some());

        let he01 = simpl
            .find_halfedge(0, 1)
            .ok_or(anyhow::Error::msg("could not find edge [0; 1]"))?;

        let he12 = he01.next();
        let he20 = he12.next();

        let he10 = he01.opposite();
        let he21 = he12.opposite();
        let he02 = he20.opposite();

        assert!(he01.triangle().index() == he12.triangle().index());
        assert!(he12.triangle().index() == he20.triangle().index());

        assert!(he01.first_node().value() == 0);
        assert!(he01.last_node().value() == 1);

        assert!(he12.first_node().value() == 1);
        assert!(he12.last_node().value() == 2);

        assert!(he20.first_node().value() == 2);
        assert!(he20.last_node().value() == 0);

        assert!(he10.triangle().index() == he21.triangle().index());
        assert!(he21.triangle().index() == he02.triangle().index());

        assert!(he10.first_node().value() == 1);
        assert!(he10.last_node().value() == 0);

        assert!(he21.first_node().value() == 2);
        assert!(he21.last_node().value() == 1);

        assert!(he02.first_node().value() == 0);
        assert!(he02.last_node().value() == 2);

        simpl.insert_node_within_triangle(3, ind_tri1)?;

        assert!(simpl.get_nb_triangles() == 4);
        assert!(simpl.find_triangle(0, 1, 2).is_some());
        assert!(simpl.find_triangle(0, 1, 3).is_some());
        assert!(simpl.find_triangle(0, 2, 3).is_some());
        assert!(simpl.find_triangle(1, 2, 3).is_some());

        assert!(simplicial_is_valid(&simpl)?);
        Ok(())
    }

    #[test]
    fn node_register_test() -> Result<()> {
        let mut simpl = Simplicial2::new(true);
        let [_, ind_tri1] = simpl.insert_first_triangle([0, 1, 2])?;

        assert!(simplicial_is_valid(&simpl)?);
        assert!(simpl.get_nb_triangles() == 2);
        assert!(simpl.find_triangle(0, 1, 2).is_some());

        let he01 = simpl
            .find_halfedge(0, 1)
            .ok_or(anyhow::Error::msg("could not find edge [0; 1]"))?;

        let he12 = he01.next();
        let he20 = he12.next();

        let he10 = he01.opposite();
        let he21 = he12.opposite();
        let he02 = he20.opposite();

        assert!(he01.triangle().index() == he12.triangle().index());
        assert!(he12.triangle().index() == he20.triangle().index());

        assert!(he01.first_node().value() == 0);
        assert!(he01.last_node().value() == 1);

        assert!(he12.first_node().value() == 1);
        assert!(he12.last_node().value() == 2);

        assert!(he20.first_node().value() == 2);
        assert!(he20.last_node().value() == 0);

        assert!(he10.triangle().index() == he21.triangle().index());
        assert!(he21.triangle().index() == he02.triangle().index());

        assert!(he10.first_node().value() == 1);
        assert!(he10.last_node().value() == 0);

        assert!(he21.first_node().value() == 2);
        assert!(he21.last_node().value() == 1);

        assert!(he02.first_node().value() == 0);
        assert!(he02.last_node().value() == 2);

        simpl.insert_node_within_triangle(3, ind_tri1)?;

        assert!(simpl.get_nb_triangles() == 4);
        assert!(simpl.find_triangle(0, 1, 2).is_some());
        assert!(simpl.find_triangle(0, 1, 3).is_some());
        assert!(simpl.find_triangle(0, 2, 3).is_some());
        assert!(simpl.find_triangle(1, 2, 3).is_some());

        assert!(simplicial_is_valid(&simpl)?);

        Ok(())
    }
}
