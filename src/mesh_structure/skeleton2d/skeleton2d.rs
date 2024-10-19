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

    /// Edges number
    pub(super) nb_edges: usize,

    /// List of oriented edges starting from each vertex
    pub(super) edges: Vec<Vec<usize>>,

    /// Set of edges properties
    pub(super) edge_properties: PropertySet,
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
        let mut edge_properties = PropertySet::new("edge", 0);
        edge_properties.add_property(
            "v1".to_string(),
            PropertyType::Scalar(ScalarType::Int),
            Property::Int(0),
        );
        edge_properties.add_property(
            "v2".to_string(),
            PropertyType::Scalar(ScalarType::Int),
            Property::Int(0),
        );
        Skeleton2D {
            vertices: vec![],
            radii: vec![],
            vertex_properties,
            nb_edges: 0,
            edges: vec![],
            edge_properties,
        }
    }

    /// Inserts new vertex into the skeleton
    pub fn insert_vertex(&mut self, coords: Vector2<f64>, radius: f64) -> Result<usize> {
        let id = self.vertices.len();
        self.vertices.push(coords);
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

    /// Adds edge linking two vertices in skeleton
    pub fn insert_edge(&mut self, v1: usize, v2: usize) -> Result<()> {
        if v1 >= self.edges.len() || v2 >= self.edges.len() {
            return Err(anyhow::Error::msg("Vertex indices out of bounds"));
        }
        if v1 == v2 {
            return Err(anyhow::Error::msg("Cannot have edge between same vertex"));
        }
        // insert if edge does not already exists
        if !self.edges[v1].contains(&v2) {
            self.edges[v1].push(v2);
            self.edges[v2].push(v1);
            let id = self.nb_edges;
            self.edge_properties.push_element();
            let (minv, maxv) = if v1 < v2 { (v1, v2) } else { (v2, v1) };
            self.edge_properties.set_property_value(
                id,
                "v1".to_string(),
                Property::Int(minv as i32),
            )?;
            self.edge_properties.set_property_value(
                id,
                "v2".to_string(),
                Property::Int(maxv as i32),
            )?;
            self.nb_edges = self.nb_edges + 1;
        }
        Ok(())
    }

    /// Get number of vertices
    pub fn get_nb_vertex(&self) -> usize {
        self.vertices.len()
    }

    /// Get vertex coordinates
    pub fn get_vertex_coords(&self, ind_vertex: usize) -> Result<Vector2<f64>> {
        if ind_vertex >= self.vertices.len() {
            return Err(anyhow::Error::msg("Vertex index out of bounds"));
        }
        Ok(self.vertices[ind_vertex])
    }

    /// Get vertex radius
    pub fn get_vertex_radius(&self, ind_vertex: usize) -> Result<f64> {
        if ind_vertex >= self.vertices.len() {
            return Err(anyhow::Error::msg("Vertex index out of bounds"));
        }
        Ok(self.radii[ind_vertex])
    }

    /// Get vertex neighbors
    pub fn get_vertex_neighbors(&self, ind_vertex: usize) -> Result<Vec<usize>> {
        if ind_vertex >= self.edges.len() {
            return Err(anyhow::Error::msg("Vertex index out of bounds"));
        }
        Ok(self.edges[ind_vertex].clone())
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

    /// Get edge properties
    pub fn get_edge_properties(&self) -> &PropertySet {
        &self.edge_properties
    }
}
