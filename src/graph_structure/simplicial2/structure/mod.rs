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

/// Cell iterator on dual of 2d simplicial
mod iter_cell_2;
pub use iter_cell_2::IterCell2;

/// Cell halfedge iterator on dual of 2d simplicial
mod iter_cell_halfedge_2;
pub use iter_cell_halfedge_2::IterCellHalfEdge2;

/// Cell node iterator on dual of 2d simplicial
mod iter_cell_node_2;
pub use iter_cell_node_2::IterCellNode2;
