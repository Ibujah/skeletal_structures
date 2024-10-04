use anyhow::Result;

use crate::mesh3d::{IterFace, IterHalfEdge, ManifoldTriangularMesh3D};

/// Checks if edge is too sharp
fn check_too_sharp_edge(he: &IterHalfEdge) -> Result<bool> {
    let face = he.face();
    let face_opp = he.opposite().face();

    let normal = face.compute_normal();

    let [v1, v2, v3] = face_opp.vertices();
    let vert1 = v1.vertex();
    let vert2 = v2.vertex();
    let vert3 = v3.vertex();

    let vec1 = vert2 - vert1;
    let vec2 = vert3 - vert2;

    let normal_opp = vec1.cross(&vec2).normalize();

    let cos_ang = normal.dot(&normal_opp);

    Ok(cos_ang < -0.99)
}

/// Checks sharp edges of the mesh
pub fn has_sharp_edges(mesh: &ManifoldTriangularMesh3D) -> Result<bool> {
    for he in mesh.get_all_halfedges().iter() {
        let ind1 = he.first_vertex().index();
        let ind2 = he.last_vertex().index();
        if ind1 > ind2 {
            continue;
        }
        if check_too_sharp_edge(&he)? {
            return Ok(true);
        }
    }

    Ok(false)
}

/// Checks if triangle has mesh intersection with edges
fn check_triangle_mesh_inter(mesh: &ManifoldTriangularMesh3D, triangle: &IterFace) -> Result<bool> {
    let [v1, v2, v3] = triangle.vertices();
    let vert1 = v1.vertex();
    let vert2 = v2.vertex();
    let vert3 = v3.vertex();

    let normal = triangle.compute_normal();

    for ind_vert in 0..mesh.get_nb_vertices() {
        if ind_vert == v1.index() || ind_vert == v2.index() || ind_vert == v3.index() {
            continue;
        }

        let vertex = mesh.get_vertex(ind_vert).unwrap();
        let coo = vertex.vertex();

        // checks if vertex is above the triangle
        let above = (coo - vert1).dot(&normal) > 0.0;
        if !above {
            continue;
        }

        // checks for all edges if other extremity is below
        for edg in vertex.halfedges() {
            let vertex_ext2 = edg.last_vertex();
            let ind_vert_ext2 = vertex_ext2.index();
            if ind_vert_ext2 == v1.index()
                || ind_vert_ext2 == v2.index()
                || ind_vert_ext2 == v3.index()
            {
                continue;
            }
            let coo_ext2 = mesh.get_vertex(ind_vert_ext2)?.vertex();

            // checks if vertex is below the triangle
            let below = (coo_ext2 - vert1).dot(&normal) < 0.0;
            if !below {
                continue;
            }

            // computes intersection between edge and plane
            let line_vec = (coo_ext2 - coo).normalize();
            let d = (vert1 - coo).dot(&normal) / line_vec.dot(&normal);
            let point = coo + d * line_vec;

            // checks if intersection is in the triangle
            let vert1_mov = vert1 - point;
            let vert2_mov = vert2 - point;
            let vert3_mov = vert3 - point;

            let n1 = vert2_mov.cross(&vert3_mov);
            let n2 = vert3_mov.cross(&vert1_mov);
            let n3 = vert1_mov.cross(&vert2_mov);

            let in_trangle = n1.dot(&n2) > 0.0 && n1.dot(&n3) > 0.0;
            if in_trangle {
                println!("{}", (coo - vert1).dot(&normal));
                println!("{}", (point - vert1).dot(&normal));
                println!("{}", (coo_ext2 - vert1).dot(&normal));

                print!("triangle ({} {} {})  ", v1.index(), v2.index(), v3.index());
                print!("[{}, {}, {}], ", vert1[0], vert1[1], vert1[2]);
                print!("[{}, {}, {}], ", vert2[0], vert2[1], vert2[2]);
                println!("[{}, {}, {}]", vert3[0], vert3[1], vert3[2]);

                print!("edge ({} {})  ", ind_vert, ind_vert_ext2);
                print!("[{}, {}, {}], ", coo[0], coo[1], coo[2]);
                println!("[{}, {}, {}]", coo_ext2[0], coo_ext2[1], coo_ext2[2]);

                print!("triangle_mov  ");
                print!("[{}, {}, {}], ", vert1_mov[0], vert1_mov[1], vert1_mov[2]);
                print!("[{}, {}, {}], ", vert2_mov[0], vert2_mov[1], vert2_mov[2]);
                println!("[{}, {}, {}]", vert3_mov[0], vert3_mov[1], vert3_mov[2]);

                return Ok(true);
            }
        }
    }

    Ok(false)
}

/// Checks mesh intersections of the mesh
pub fn has_self_intersection(mesh: &ManifoldTriangularMesh3D) -> Result<bool> {
    for f in mesh.get_all_faces().iter() {
        if check_triangle_mesh_inter(mesh, f)? {
            return Ok(true);
        }
    }

    Ok(false)
}
