#[cfg(test)]
mod mesh_test {
    use anyhow::Result;
    use nalgebra::base::*;

    use crate::mesh3d::mesh_quality_check::*;
    use crate::mesh3d::ManifoldTriangularMesh3D;

    use ply_rs::ply::{Property, PropertyType, ScalarType};

    fn build_cube() -> Result<ManifoldTriangularMesh3D> {
        let vertex_coordinates = vec![
            Vector3::new(1.0, 1.0, -1.0),
            Vector3::new(1.0, -1.0, -1.0),
            Vector3::new(1.0, 1.0, 1.0),
            Vector3::new(1.0, -1.0, 1.0),
            Vector3::new(-1.0, 1.0, -1.0),
            Vector3::new(-1.0, -1.0, -1.0),
            Vector3::new(-1.0, 1.0, 1.0),
            Vector3::new(-1.0, -1.0, 1.0),
        ];
        let triangular_faces_indices = vec![
            [4, 2, 0],
            [2, 7, 3],
            [6, 5, 7],
            [1, 7, 5],
            [0, 3, 1],
            [4, 1, 5],
            [4, 6, 2],
            [2, 6, 7],
            [6, 4, 5],
            [1, 3, 7],
            [0, 2, 3],
            [4, 0, 1],
        ];
        ManifoldTriangularMesh3D::new(vertex_coordinates, triangular_faces_indices)
    }

    #[test]
    fn check_no_sharp_edges() -> Result<()> {
        let mesh = build_cube()?;

        assert!(!has_sharp_edges(&mesh)?);
        Ok(())
    }

    #[test]
    fn check_no_self_intersection() -> Result<()> {
        let mesh = build_cube()?;

        assert!(!has_self_intersection(&mesh)?);
        Ok(())
    }

    #[test]
    fn add_vertex_property() -> Result<()> {
        let mut mesh = build_cube()?;

        mesh.add_vertex_property(
            "some_double".to_string(),
            PropertyType::Scalar(ScalarType::Double),
            Property::Double(0.1),
        );

        mesh.set_vertex_property_value(5, "some_double".to_string(), Property::Double(0.2))?;

        for i in 0..mesh.get_nb_vertices() {
            let prop = mesh
                .get_vertex_properties()
                .get_property_value(i, "some_double".to_string())?;

            if i != 5 {
                assert!(prop == Property::Double(0.1))
            } else {
                assert!(prop == Property::Double(0.2))
            }
        }
        Ok(())
    }

    #[test]
    fn add_triangle_property() -> Result<()> {
        let mut mesh = build_cube()?;

        mesh.add_face_property(
            "some_double".to_string(),
            PropertyType::Scalar(ScalarType::Double),
            Property::Double(0.1),
        );

        mesh.set_face_property_value(5, "some_double".to_string(), Property::Double(0.2))?;

        for i in 0..mesh.get_nb_vertices() {
            let prop = mesh
                .get_face_properties()
                .get_property_value(i, "some_double".to_string())?;

            if i != 5 {
                assert!(prop == Property::Double(0.1))
            } else {
                assert!(prop == Property::Double(0.2))
            }
        }
        Ok(())
    }
}
