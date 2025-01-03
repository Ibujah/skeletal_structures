/// Optimised 2D simplicial structure (no geometry)
mod simplicial_2;
pub use simplicial_2::Simplicial2;

/// Node iterator on 2D simplicial
mod iter_node_2;
pub use iter_node_2::IterNode2;

/// Halfedge iterator on 2D simplicial
mod iter_halfedge_2;
pub use iter_halfedge_2::IterHalfEdge2;

/// Triangle iterator on 2D simplicial
mod iter_triangle_2;
pub use iter_triangle_2::IterTriangle2;

/// Iterator on diagram dual of 2D simplicial
mod iter_diagram_cell_2;
pub use iter_diagram_cell_2::IterDiagCell2;

/// Halfedge iterator on diagram dual of 2D simplicial
mod iter_diagram_halfedge_2;
pub use iter_diagram_halfedge_2::IterDiagHalfEdge2;

/// Node iterator on diagram dual of 2D simplicial
mod iter_diagram_node_2;
pub use iter_diagram_node_2::IterDiagNode2;
