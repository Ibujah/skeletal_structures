use anyhow::Result;
use nalgebra::Vector3;
use ply_rs::ply::Property;

use super::IterHalfEdge;
use super::IterVertex;
use super::ManifoldTriangularMesh3D;

use crate::graph_structure::simplicial2::IterTriangle2;

#[derive(Copy, Clone)]
/// Triangle iterator
pub struct IterFace<'a> {
    mesh: &'a ManifoldTriangularMesh3D,
    iter_triangle: IterTriangle2<'a>,
}

impl<'a> IterFace<'a> {
    /// Creates a new triangle iterator from the given manifold triangular mesh and index.
    pub(super) fn new(
        mesh: &'a ManifoldTriangularMesh3D,
        iter_triangle: IterTriangle2<'a>,
    ) -> IterFace<'a> {
        IterFace {
            mesh,
            iter_triangle,
        }
    }

    /// Surrounding halfedges (array of halfedge iterators)
    pub fn halfedges(&self) -> [IterHalfEdge<'a>; 3] {
        let [he0, he1, he2] = self.iter_triangle.halfedges();
        [
            IterHalfEdge::new(self.mesh, he0),
            IterHalfEdge::new(self.mesh, he1),
            IterHalfEdge::new(self.mesh, he2),
        ]
    }

    /// usizes(array of nodes)
    pub fn vertex_indices(&self) -> [usize; 3] {
        self.iter_triangle.node_values()
    }

    /// array of vertices
    pub fn vertices(&self) -> [IterVertex; 3] {
        let [no0, no1, no2] = self.iter_triangle.nodes();
        [
            IterVertex::new(self.mesh, no0),
            IterVertex::new(self.mesh, no1),
            IterVertex::new(self.mesh, no2),
        ]
    }

    /// Normal
    pub fn compute_normal(&self) -> Vector3<f64> {
        let [v0, v1, v2] = self.vertices();
        let vert0 = v0.vertex();
        let vert1 = v1.vertex();
        let vert2 = v2.vertex();

        let vec0 = vert1 - vert0;
        let vec1 = vert2 - vert1;

        vec0.cross(&vec1).normalize()
    }

    /// Get given vertex property
    pub fn get_property_value(&self, property_name: String) -> Result<Property> {
        self.mesh
            .get_face_properties()
            .get_property_value(self.iter_triangle.index(), property_name)
    }
}
