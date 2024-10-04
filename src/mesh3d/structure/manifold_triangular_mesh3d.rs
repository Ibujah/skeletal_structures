use anyhow::Result;
use nalgebra::base::*;

use crate::simplicial2::Simplicial2;

use super::property_set::PropertySet;

use super::{IterFace, IterHalfEdge, IterVertex};

use ply_rs::ply::{Property, PropertyType, ScalarType};

#[derive(Clone)]
/// ManifoldTriangularMesh3D is a struct that represents a manifold mesh in three-dimensional space.
pub struct ManifoldTriangularMesh3D {
    /// A vector of vertices, where each vertex is a `Vertex`,
    /// which is a point with x, y, z coordinates.
    pub(super) vertices: Vec<Vector3<f64>>,

    pub(super) simplicial: Simplicial2,

    /// Set of vertices properties
    pub(super) vertex_properties: PropertySet,

    /// Set of faces properties
    pub(super) face_properties: PropertySet,
}

impl ManifoldTriangularMesh3D {
    /// Manifold mesh constructor
    pub fn new(
        vertices: Vec<Vector3<f64>>,
        faces: Vec<[usize; 3]>,
    ) -> Result<ManifoldTriangularMesh3D> {
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

        face_properties.add_property(
            "vertex_indices".to_string(),
            PropertyType::List(ScalarType::UInt, ScalarType::UInt),
            Property::ListUInt(Vec::new()),
        );

        let mut simplicial = Simplicial2::new(true);
        simplicial.insert_triangle_list(faces)?;

        Ok(ManifoldTriangularMesh3D {
            vertices,

            simplicial,

            vertex_properties,
            face_properties,
        })
    }

    /// Gets number of vertices
    pub fn get_nb_vertices(&self) -> usize {
        self.vertices.len()
    }

    /// Gets vertex iterator
    pub fn get_vertex(&self, ind_vertex: usize) -> Result<IterVertex> {
        let iter_node = self
            .simplicial
            .find_node(ind_vertex)
            .ok_or(anyhow::Error::msg("Vertex does not exist"))?;
        Ok(IterVertex::new(self, iter_node))
    }

    /// Gets number of halfedges
    pub fn get_nb_halfedges(&self) -> usize {
        self.simplicial.get_nb_halfedges()
    }

    /// Gets all halfedges
    pub fn get_all_halfedges(&self) -> Vec<IterHalfEdge> {
        self.simplicial
            .get_all_halfedges()
            .into_iter()
            .map(|he| IterHalfEdge::new(self, he))
            .collect()
    }

    /// Gets number of faces
    pub fn get_nb_faces(&self) -> usize {
        self.simplicial.get_nb_triangles()
    }

    /// Gets all faces
    pub fn get_all_faces(&self) -> Vec<IterFace> {
        self.simplicial
            .get_all_triangles()
            .into_iter()
            .map(|tri| IterFace::new(self, tri))
            .collect()
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
        ind_triangle: usize,
        property_name: String,
        property_value: Property,
    ) -> Result<()> {
        if ind_triangle >= self.simplicial.get_nb_triangles() {
            return Err(anyhow::Error::msg(
                "set_face_property_value(): Index out of bounds",
            ));
        }
        self.face_properties
            .set_property_value(ind_triangle, property_name, property_value)
    }
}
