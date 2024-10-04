use super::IterFace;
use super::IterVertex;
use super::ManifoldTriangularMesh3D;

use crate::graph_structure::simplicial2::IterHalfEdge2;

#[derive(Copy, Clone)]
/// Halfedge iterator
pub struct IterHalfEdge<'a> {
    mesh: &'a ManifoldTriangularMesh3D,
    iter_halfedge: IterHalfEdge2<'a>,
}

impl<'a> IterHalfEdge<'a> {
    /// Creates a new halfedge iterator from the given manifold triangular mesh and index.
    pub(super) fn new(
        mesh: &'a ManifoldTriangularMesh3D,
        iter_halfedge: IterHalfEdge2<'a>,
    ) -> IterHalfEdge<'a> {
        IterHalfEdge {
            mesh,
            iter_halfedge,
        }
    }

    /// First node
    pub fn first_vertex(&self) -> IterVertex<'a> {
        IterVertex::new(self.mesh, self.iter_halfedge.first_node())
    }

    /// Last node
    pub fn last_vertex(&self) -> IterVertex<'a> {
        IterVertex::new(self.mesh, self.iter_halfedge.last_node())
    }

    /// Next halfedge on same triangle
    pub fn next(&self) -> IterHalfEdge<'a> {
        IterHalfEdge::new(self.mesh, self.iter_halfedge.next())
    }

    /// Previous halfedge on same triangle
    pub fn previous(&self) -> IterHalfEdge<'a> {
        IterHalfEdge::new(self.mesh, self.iter_halfedge.previous())
    }

    /// Opposite halfedge: Same vertices in opposite order (on neighbor triangle)
    pub fn opposite(&self) -> IterHalfEdge<'a> {
        IterHalfEdge::new(self.mesh, self.iter_halfedge.opposite())
    }

    /// Triangle containing halfedge
    pub fn face(&self) -> IterFace<'a> {
        IterFace::new(self.mesh, self.iter_halfedge.triangle())
    }
}
