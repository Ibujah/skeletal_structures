use anyhow::Result;
use nalgebra::base::*;
use ply_rs::ply::{Property, PropertyType, ScalarType};

use super::super::super::property_set::PropertySet;

#[derive(Clone)]
/// Mesh3D is a struct that represents a mesh in three-dimensional space.
pub struct Mesh3D {
    /// A vector of vertices, where each vertex is a `Vertex`,
    /// which is a point with x, y, z coordinates.
    pub(super) vertices: Vec<Vector3<f64>>,

    /// Set of faces (3 or more vertices)
    pub(super) faces: Vec<Vec<usize>>,

    /// Set of vertices properties
    pub(super) vertex_properties: PropertySet,

    /// Set of faces properties
    pub(super) face_properties: PropertySet,
}

impl Mesh3D {
    /// Mesh constructor
    pub fn new(vertices: Vec<Vector3<f64>>, faces: Vec<Vec<usize>>) -> Mesh3D {
        let mut vertex_properties = PropertySet::new("vertex", vertices.len());
        let mut face_properties = PropertySet::new("face", faces.len());

        vertex_properties.add_property(
            "x".to_string(),
            PropertyType::Scalar(ScalarType::Double),
            Property::Double(0.),
        );
        vertex_properties.add_property(
            "y".to_string(),
            PropertyType::Scalar(ScalarType::Double),
            Property::Double(0.),
        );
        vertex_properties.add_property(
            "z".to_string(),
            PropertyType::Scalar(ScalarType::Double),
            Property::Double(0.),
        );
        for ind_vertex in 0..vertices.len() {
            vertex_properties.push_element();
            vertex_properties
                .set_property_value(
                    ind_vertex,
                    "x".to_string(),
                    Property::Double(vertices[ind_vertex][0]),
                )
                .unwrap();
            vertex_properties
                .set_property_value(
                    ind_vertex,
                    "y".to_string(),
                    Property::Double(vertices[ind_vertex][1]),
                )
                .unwrap();
            vertex_properties
                .set_property_value(
                    ind_vertex,
                    "z".to_string(),
                    Property::Double(vertices[ind_vertex][2]),
                )
                .unwrap();
        }

        face_properties.add_property(
            "vertex_indices".to_string(),
            PropertyType::List(ScalarType::UInt, ScalarType::UInt),
            Property::ListUInt(Vec::new()),
        );
        for ind_face in 0..faces.len() {
            face_properties.push_element();
            face_properties
                .set_property_value(
                    ind_face,
                    "vertex_indices".to_string(),
                    Property::ListUInt(faces[ind_face].iter().map(|&i| i as u32).collect()),
                )
                .unwrap();
        }

        Mesh3D {
            vertices,
            faces,
            vertex_properties,
            face_properties,
        }
    }

    /// Gets number of vertices
    pub fn get_nb_vertices(&self) -> usize {
        self.vertices.len()
    }

    /// Gets vertex iterator
    pub fn get_vertex(&self, ind_vertex: usize) -> Result<Vector3<f64>> {
        if ind_vertex >= self.vertices.len() {
            return Err(anyhow::Error::msg("Vertex index out of bounds"));
        }
        Ok(self.vertices[ind_vertex])
    }

    /// Gets number of faces
    pub fn get_nb_faces(&self) -> usize {
        self.faces.len()
    }

    /// Gets all faces
    pub fn get_face(&self, ind_face: usize) -> Result<&Vec<usize>> {
        if ind_face >= self.faces.len() {
            return Err(anyhow::Error::msg("Face index out of bounds"));
        }
        Ok(&self.faces[ind_face])
    }

    /// Adds a vertex property with a default value
    pub fn add_vertex_property(
        &mut self,
        property_name: String,
        property_type: PropertyType,
        property_default: Property,
    ) -> () {
        self.vertex_properties
            .add_property(property_name, property_type, property_default);
    }

    /// Sets a vertex property
    pub fn set_vertex_property_value(
        &mut self,
        ind_vertex: usize,
        property_name: String,
        property_value: Property,
    ) -> Result<()> {
        if ind_vertex >= self.vertices.len() {
            return Err(anyhow::Error::msg(
                "set_vertex_property_value(): Index out of bounds",
            ));
        }
        self.vertex_properties
            .set_property_value(ind_vertex, property_name, property_value)
    }

    /// Get vertex properties
    pub fn get_vertex_properties(&self) -> &PropertySet {
        &self.vertex_properties
    }

    /// Get face properties
    pub fn get_face_properties(&self) -> &PropertySet {
        &self.face_properties
    }

    /// Adds a face property with a default value
    pub fn add_face_property(
        &mut self,
        property_name: String,
        property_type: PropertyType,
        property_default: Property,
    ) -> () {
        self.face_properties
            .add_property(property_name, property_type, property_default);
    }

    /// Sets a face property
    pub fn set_face_property_value(
        &mut self,
        ind_face: usize,
        property_name: String,
        property_value: Property,
    ) -> Result<()> {
        if ind_face >= self.faces.len() {
            return Err(anyhow::Error::msg(
                "set_face_property_value(): Index out of bounds",
            ));
        }
        self.face_properties
            .set_property_value(ind_face, property_name, property_value)
    }
}
