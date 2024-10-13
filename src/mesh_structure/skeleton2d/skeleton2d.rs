use anyhow::Result;
use nalgebra::base::*;

use crate::mesh_structure::property_set::PropertySet;
use ply_rs::ply::{Property, PropertyType, ScalarType};

#[derive(Clone)]
/// Skeleton 2D is a struct that represents a 2D skeleton
pub struct Skeleton2D {
    /// A vector of vertices, where each vertex is a 2D vector,
    /// which is a point with x, y coordinates.
    pub(super) vertices: Vec<Vector2<f64>>,

    pub(super) radii: Vec<f64>,

    /// Set of vertices properties
    pub(super) vertex_properties: PropertySet,

    /// List of oriented edges starting from each vertex
    pub(super) edges: Vec<Vec<usize>>,
}

impl Skeleton2D {
    /// Creates a new empty skeleton
    pub fn new() -> Self {
        let mut vertex_properties = PropertySet::new("vertex", 0);
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
            "radius".to_string(),
            PropertyType::Scalar(ScalarType::Double),
            Property::Double(0.),
        );
        Skeleton2D {
            vertices: vec![],
            radii: vec![],
            vertex_properties,
            edges: vec![],
        }
    }

    /// Insert new vertex into the skeleton
    pub fn insert_vertex(&mut self, coords: Vec<f64>, radius: f64) -> Result<usize> {
        let id = self.vertices.len();
        self.vertices.push(Vector2::new(coords[0], coords[1]));
        self.radii.push(radius);
        self.vertex_properties.push_element();
        self.vertex_properties.set_property_value(
            id,
            "x".to_string(),
            Property::Double(coords[0]),
        )?;
        self.vertex_properties.set_property_value(
            id,
            "y".to_string(),
            Property::Double(coords[1]),
        )?;
        self.vertex_properties.set_property_value(
            id,
            "radius".to_string(),
            Property::Double(radius),
        )?;
        self.edges.push(Vec::new());
        Ok(id)
    }

    /// Adds edge in skeleton
    pub fn add_edge(&mut self, v1: usize, v2: usize) -> Result<()> {
        if v1 >= self.edges.len() || v2 >= self.edges.len() {
            return Err(anyhow::Error::msg("Vertex indices out of bounds"));
        }
        // insert if edge does not already exists
        if !self.edges[v1].contains(&v2) {
            self.edges[v1].push(v2);
            self.edges[v2].push(v1);
        }
        Ok(())
    }
}
