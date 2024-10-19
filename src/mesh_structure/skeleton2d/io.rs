use anyhow::Result;

use ply_rs::ply::{Addable, DefaultElement, Encoding, Ply};
use ply_rs::writer::Writer;
use std::fs::File;

use super::skeleton2d::Skeleton2D;

/// Save a skeleton to a PLY file
pub fn save_skeleton2d(
    filename: &str,
    skeleton: &Skeleton2D,
    header: Option<String>,
) -> Result<()> {
    let mut ply = Ply::<DefaultElement>::new();
    ply.header.encoding = Encoding::Ascii;
    if let Some(h) = header {
        ply.header.comments.push(h);
    }

    let vertex_header_element = skeleton.get_vertex_properties().get_header_element();
    let edge_header_element = skeleton.get_edge_properties().get_header_element();

    let vertices_payload_element = skeleton.get_vertex_properties().get_payload_element();
    let edges_payload_element = skeleton.get_edge_properties().get_payload_element();

    ply.header.elements.add(vertex_header_element);
    ply.header.elements.add(edge_header_element);

    ply.payload
        .insert("vertex".to_string(), vertices_payload_element);
    ply.payload
        .insert("edge".to_string(), edges_payload_element);

    ply.make_consistent().unwrap();

    let mut file = File::create(filename)?;
    let w = Writer::new();
    w.write_ply(&mut file, &mut ply).unwrap();
    Ok(())
}
