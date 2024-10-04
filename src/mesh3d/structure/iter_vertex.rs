use anyhow::Result;
use nalgebra::Vector3;
use ply_rs::ply::Property;

use super::ManifoldTriangularMesh3D;

use super::IterHalfEdge;

use crate::graph_structure::simplicial2::IterNode2;

#[derive(Copy, Clone)]
/// Vertex iterator
pub struct IterVertex<'a> {
    mesh: &'a ManifoldTriangularMesh3D,
    iter_node: IterNode2<'a>,
}

impl<'a> IterVertex<'a> {
    /// Creates a new vertex iterator from the given manifold triangular simplicial and index.
    pub(super) fn new(
        mesh: &'a ManifoldTriangularMesh3D,
        iter_node: IterNode2<'a>,
    ) -> IterVertex<'a> {
        IterVertex { mesh, iter_node }
    }

    /// Gets vertex index
    pub fn index(&self) -> usize {
        self.iter_node.value()
    }

    /// Get vertex coordinates
    pub fn vertex(&self) -> Vector3<f64> {
        self.mesh.vertices[self.index()]
    }

    /// Get given vertex property
    pub fn get_property_value(&self, property_name: String) -> Result<Property> {
        self.mesh
            .get_vertex_properties()
            .get_property_value(self.index(), property_name)
    }

    /// Gets list of halfedges starting at this vertex
    pub fn halfedges(&self) -> Vec<IterHalfEdge<'a>> {
        self.iter_node
            .halfedges()
            .iter()
            .map(|&it_he| IterHalfEdge::new(self.mesh, it_he))
            .collect()
    }
}
