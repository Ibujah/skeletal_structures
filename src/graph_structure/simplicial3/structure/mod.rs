/// Optimised 3D simplicial structure (no geometry)
mod simplicial_3;
pub use simplicial_3::Simplicial3;

/// Node iterator on 3D simplicial
mod iter_node_3;
pub use iter_node_3::IterNode3;

/// Halfedge iterator on 3D simplicial
mod iter_halfedge_3;
pub use iter_halfedge_3::IterHalfEdge3;

/// Halftriangle iterator on 3D simplicial
mod iter_halftriangle_3;
pub use iter_halftriangle_3::IterHalfTriangle3;

/// Tetrahedron iterator on 3D simplicial
mod iter_tetrahedron_3;
pub use iter_tetrahedron_3::IterTetrahedron3;

/// Bowyer-Watson algorithm for 3D simplicial
mod bowyer_watson_inserter;
pub use bowyer_watson_inserter::BowyerWatsonInserter;

/// Build function for 3D simplicial
mod build_simplicial_3;
pub use build_simplicial_3::*;
