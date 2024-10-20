use anyhow::Result;

use nalgebra::base::*;
use ply_rs::parser::Parser;
use ply_rs::ply::{Addable, DefaultElement, Encoding, Ply, Property};
use ply_rs::writer::Writer;
use std::fs::File;

use super::skeleton2d::Skeleton2D;

/// Save a skeleton to a PLY file
pub fn save_skeleton2d_ply(
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

/// Load a skeleton from a PLY file
pub fn load_skeleton2d_ply(file_path: &str) -> Result<Skeleton2D> {
    let mut f = std::fs::File::open(file_path)?;

    let ply = Parser::<DefaultElement>::new().read_ply(&mut f)?;

    let mut skeleton = Skeleton2D::new();

    // Load vertices
    if !ply.payload.contains_key("vertex") {
        return Err(anyhow::Error::msg("No vertex element in file"));
    }
    for v in ply.payload["vertex"].iter() {
        let x = if let Some(x_prop) = v.get("x") {
            if let &Property::Float(x) = x_prop {
                x
            } else {
                return Err(anyhow::Error::msg("No x property in vertex"));
            }
        } else {
            return Err(anyhow::Error::msg("No x property in vertex"));
        };
        let y = if let Some(y_prop) = v.get("y") {
            if let &Property::Float(y) = y_prop {
                y
            } else {
                return Err(anyhow::Error::msg("No y property in vertex"));
            }
        } else {
            return Err(anyhow::Error::msg("No y property in vertex"));
        };
        let radius = if let Some(radius_prop) = v.get("radius") {
            if let &Property::Float(radius) = radius_prop {
                radius
            } else {
                return Err(anyhow::Error::msg("No radius propertradius in vertex"));
            }
        } else {
            return Err(anyhow::Error::msg("No radius propertradius in vertex"));
        };
        let ind_vertex = skeleton.insert_vertex(Vector2::new(x as f64, y as f64), radius as f64)?;

        for (key, prop) in v.into_iter() {
            match (key.as_ref(), prop) {
                ("x", _) => (),
                ("y", _) => (),
                ("radius", _) => (),
                (k, p) => {
                    skeleton.set_vertex_property_value(ind_vertex, k.to_string(), p.clone())?;
                    ()
                }
            }
        }
    }

    // Load edges
    if !ply.payload.contains_key("edge") {
        return Err(anyhow::Error::msg("No edge element in file"));
    }
    for e in ply.payload["edge"].iter() {
        let vertex1 = if let Some(vertex1_prop) = e.get("vertex1") {
            if let &Property::Int(vertex1) = vertex1_prop {
                vertex1
            } else {
                return Err(anyhow::Error::msg("No vertex1 property in edge"));
            }
        } else {
            return Err(anyhow::Error::msg("No vertex1 property in edge"));
        };
        let vertex2 = if let Some(vertex2_prop) = e.get("vertex2") {
            if let &Property::Int(vertex2) = vertex2_prop {
                vertex2
            } else {
                return Err(anyhow::Error::msg("No vertex2 property in edge"));
            }
        } else {
            return Err(anyhow::Error::msg("No vertex2 property in edge"));
        };
        skeleton.insert_edge(vertex1 as usize, vertex2 as usize)?;
    }

    Ok(skeleton)
}
