use anyhow::Result;
use nalgebra::base::*;
use std::fs::File;
use std::io::{self, BufRead};

use ply_rs::ply::{Addable, DefaultElement, Encoding, Ply};
use ply_rs::writer::Writer;

use super::mesh3d::Mesh3D;

/// Loads obj file as manifold mesh
pub fn load_obj_manifold(filename: &str) -> Result<Mesh3D> {
    let mut vertices = Vec::new();
    let mut faces = Vec::new();

    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();
    for line_ in lines {
        if let Ok(line) = line_ {
            if line.len() > 2 {
                if &line[..2] == "v " {
                    let mut line_split = line.split_whitespace();
                    let mut vert: Vector3<f64> = Vector3::new(0.0, 0.0, 0.0);
                    line_split.next();
                    for i in 0..3 {
                        let cur = line_split
                            .next()
                            .ok_or(anyhow::Error::msg("Expected value"))?;
                        vert[i] = cur.parse::<f64>()?;
                    }

                    vertices.push(vert);
                }
                if &line[..2] == "f " {
                    let mut line_split = line.split_whitespace();
                    let mut triangle: [usize; 3] = [0, 0, 0];
                    line_split.next();
                    for i in 0..3 {
                        let cur = line_split
                            .next()
                            .ok_or(anyhow::Error::msg("Expected value"))?;
                        let mut cur_split = cur.split('/');
                        let ind = cur_split
                            .next()
                            .ok_or(anyhow::Error::msg("Expected value"))?;
                        triangle[i] = ind.parse::<usize>()? - 1;
                    }

                    faces.push(vec![triangle[0], triangle[1], triangle[2]]);
                }
            }
        }
    }

    Ok(Mesh3D::new(vertices, faces))
}

/// Loads off file as manifold mesh
pub fn load_off_manifold(filename: &str) -> Result<Mesh3D> {
    let mut vertices = Vec::new();
    let mut faces = Vec::new();

    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();
    let mut opt_nb_vert = None;
    let mut opt_nb_triangle = None;
    let mut cur_vert = 0;
    let mut cur_triangle = 0;
    for line_ in lines {
        if let Ok(line) = line_ {
            if opt_nb_vert.is_none() {
                if line == "OFF" {
                    continue;
                }
                let mut line_split = line.split_whitespace();
                let nb_vert = line_split
                    .next()
                    .ok_or(anyhow::Error::msg("Expected value1"))?
                    .parse::<usize>()?;
                opt_nb_vert = Some(nb_vert);
                let nb_triangle = line_split
                    .next()
                    .ok_or(anyhow::Error::msg("Expected value2"))?
                    .parse::<usize>()?;
                opt_nb_triangle = Some(nb_triangle);
            } else {
                let nb_vert = opt_nb_vert.unwrap();
                let nb_triangle = opt_nb_triangle.unwrap();
                if cur_vert < nb_vert {
                    let mut line_split = line.split_whitespace();
                    let mut vert: Vector3<f64> = Vector3::new(0.0, 0.0, 0.0);
                    for i in 0..3 {
                        let ind = line_split
                            .next()
                            .ok_or(anyhow::Error::msg("Expected value3"))?
                            .parse::<f64>()?;
                        vert[i] = ind;
                    }

                    vertices.push(vert);
                    cur_vert = cur_vert + 1;
                } else if cur_vert < nb_triangle {
                    let mut line_split = line.split_whitespace();
                    let mut triangle = Vec::new();
                    let nbv = line_split
                        .next()
                        .ok_or(anyhow::Error::msg("Expected value4"))?
                        .parse::<usize>()?;
                    for _ in 0..nbv {
                        let ind = line_split
                            .next()
                            .ok_or(anyhow::Error::msg("Expected value5"))?
                            .parse::<usize>()?;
                        triangle.push(ind);
                    }
                    if triangle.len() != 3 {
                        return Err(anyhow::Error::msg("Triangle with more than 3 vertices"));
                    }
                    faces.push(vec![triangle[0], triangle[1], triangle[2]]);
                    cur_triangle = cur_triangle + 1;
                }
            }
        }
    }

    Ok(Mesh3D::new(vertices, faces))
}

/// Save manifold mesh as ply file
pub fn save_ply_manifold(filename: &str, mesh: &Mesh3D, header: Option<String>) -> Result<()> {
    let mut ply = Ply::<DefaultElement>::new();
    ply.header.encoding = Encoding::Ascii;
    if let Some(h) = header {
        ply.header.comments.push(h);
    }

    let vertex_header_element = mesh.get_vertex_properties().get_header_element();
    let face_header_element = mesh.get_face_properties().get_header_element();

    let vertices_payload_element = mesh.get_vertex_properties().get_payload_element();
    let faces_payload_element = mesh.get_face_properties().get_payload_element();

    ply.header.elements.add(vertex_header_element);
    ply.header.elements.add(face_header_element);

    ply.payload
        .insert("vertex".to_string(), vertices_payload_element);
    ply.payload
        .insert("face".to_string(), faces_payload_element);

    ply.make_consistent().unwrap();

    let mut file = File::create(filename)?;
    let w = Writer::new();
    w.write_ply(&mut file, &mut ply).unwrap();
    Ok(())
}
