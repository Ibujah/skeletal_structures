#[cfg(test)]
mod mesh_test {
    use anyhow::Result;
    use nalgebra::base::*;

    use super::super::mesh3d::Mesh3D;

    use ply_rs::ply::{Property, PropertyType, ScalarType};

    fn build_cube() -> Result<Mesh3D> {
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
            vec![4, 2, 0],
            vec![2, 7, 3],
            vec![6, 5, 7],
            vec![1, 7, 5],
            vec![0, 3, 1],
            vec![4, 1, 5],
            vec![4, 6, 2],
            vec![2, 6, 7],
            vec![6, 4, 5],
            vec![1, 3, 7],
            vec![0, 2, 3],
            vec![4, 0, 1],
        ];
        Ok(Mesh3D::create(vertex_coordinates, triangular_faces_indices))
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
