use anyhow::Result;
use nalgebra::base::*;
use ply_rs::ply::{Property, PropertyType, ScalarType};

use crate::mesh_structure::property_set::PropertySet;

#[derive(Clone)]
/// Mesh3D is a struct that represents a mesh in three-dimensional space.
pub struct Mesh3D {
    /// A vector of vertices, where each vertex is a 2D vector,
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
    /// Empty mesh constructor
    pub fn new() -> Mesh3D {
        let mut vertex_properties = PropertySet::new("vertex", 0);
        let mut face_properties = PropertySet::new("face", 0);

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

        face_properties.add_property(
            "vertex_indices".to_string(),
            PropertyType::List(ScalarType::Char, ScalarType::UInt),
            Property::ListUInt(Vec::new()),
        );
        Mesh3D {
            vertices: Vec::new(),
            faces: Vec::new(),
            vertex_properties,
            face_properties,
        }
    }

    /// Create mesh from vertices and faces
    pub fn create(vertices: Vec<Vector3<f64>>, faces: Vec<Vec<usize>>) -> Mesh3D {
        let mut vertex_properties = PropertySet::new("vertex", 0);
        let mut face_properties = PropertySet::new("face", 0);

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

    /// Inserts new vertex
    pub fn insert_vertex(&mut self, vertex: Vector3<f64>) -> Result<usize> {
        let id = self.vertices.len();
        self.vertices.push(vertex);
        self.vertex_properties.push_element();
        self.vertex_properties.set_property_value(
            id,
            "x".to_string(),
            Property::Double(vertex[0]),
        )?;
        self.vertex_properties.set_property_value(
            id,
            "y".to_string(),
            Property::Double(vertex[1]),
        )?;
        self.vertex_properties.set_property_value(
            id,
            "z".to_string(),
            Property::Double(vertex[2]),
        )?;
        Ok(id)
    }

    /// Inserts new face
    pub fn insert_face(&mut self, face: Vec<usize>) -> Result<usize> {
        let id = self.faces.len();
        self.face_properties.push_element();
        self.face_properties
            .set_property_value(
                id,
                "vertex_indices".to_string(),
                Property::ListInt(face.iter().map(|&i| i as i32).collect()),
            )
            .unwrap();
        self.faces.push(face);
        Ok(id)
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

    /// Adds vertex property of type f32
    pub fn add_vertex_property_f32(&mut self, property_name: String, default_value: f32) -> () {
        self.add_vertex_property(
            property_name,
            PropertyType::Scalar(ScalarType::Float),
            Property::Float(default_value),
        );
    }

    /// Adds vertex property of type f64
    pub fn add_vertex_property_f64(&mut self, property_name: String, default_value: f64) -> () {
        self.add_vertex_property(
            property_name,
            PropertyType::Scalar(ScalarType::Double),
            Property::Double(default_value),
        );
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

    /// Set vertex property of type f32
    pub fn set_vertex_property_f32(
        &mut self,
        ind_vertex: usize,
        property_name: String,
        property_value: f32,
    ) -> Result<()> {
        self.set_vertex_property_value(ind_vertex, property_name, Property::Float(property_value))
    }

    /// Set vertex property of type f64
    pub fn set_vertex_property_f64(
        &mut self,
        ind_vertex: usize,
        property_name: String,
        property_value: f64,
    ) -> Result<()> {
        self.set_vertex_property_value(ind_vertex, property_name, Property::Double(property_value))
    }

    /// Get vertex properties
    pub fn get_vertex_properties(&self) -> &PropertySet {
        &self.vertex_properties
    }

    /// Get vertex property value of type f32
    pub fn get_vertex_property_value_f32(
        &self,
        ind_vertex: usize,
        property_name: String,
    ) -> Result<f32> {
        let property = self
            .vertex_properties
            .get_property_value(ind_vertex, property_name)?;
        if let Property::Float(value) = property {
            Ok(value)
        } else {
            Err(anyhow::Error::msg("Property is not of type f32"))
        }
    }

    /// Get vertex property value of type f64
    pub fn get_vertex_property_value_f64(
        &self,
        ind_vertex: usize,
        property_name: String,
    ) -> Result<f64> {
        let property = self
            .vertex_properties
            .get_property_value(ind_vertex, property_name)?;
        if let Property::Double(value) = property {
            Ok(value)
        } else {
            Err(anyhow::Error::msg("Property is not of type f64"))
        }
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

    /// Adds face property of type Vec<u32>
    pub fn add_face_property_vec_u32(
        &mut self,
        property_name: String,
        default_value: Vec<u32>,
    ) -> () {
        self.add_face_property(
            property_name,
            PropertyType::List(ScalarType::UInt, ScalarType::UInt),
            Property::ListUInt(default_value),
        );
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

    /// Sets a face property of type Vec<u32>
    pub fn set_face_property_vec_u32(
        &mut self,
        ind_face: usize,
        property_name: String,
        value: Vec<u32>,
    ) -> Result<()> {
        self.set_face_property_value(ind_face, property_name, Property::ListUInt(value))
    }

    /// Get face properties
    pub fn get_face_properties(&self) -> &PropertySet {
        &self.face_properties
    }

    /// Get face property value of type Vec<u32>
    pub fn get_face_property_value_vec_u32(
        &self,
        ind_face: usize,
        property_name: String,
    ) -> Result<Vec<u32>> {
        let property = self
            .face_properties
            .get_property_value(ind_face, property_name)?;
        if let Property::ListUInt(value) = property {
            Ok(value)
        } else {
            Err(anyhow::Error::msg("Property is not of type Vec<u32>"))
        }
    }
}
