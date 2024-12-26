use anyhow::Result;
use rand::Rng;
use std::time::Instant;

use skeletal_structures::graph_structure::simplicial2::Simplicial2;

fn generate_random_simplicial_2(
    nb_vert: usize,
    register_node_halfedges: bool,
) -> Result<Simplicial2> {
    let mut rng = rand::thread_rng();

    let mut simpl_2 = Simplicial2::new(register_node_halfedges);

    simpl_2.insert_first_triangle([0, 1, 2])?;

    for ind_nod in 3..nb_vert {
        let ind_tri = rng.gen_range(0..simpl_2.get_nb_triangles());
        simpl_2.insert_node_within_triangle(ind_nod, ind_tri)?;
    }

    Ok(simpl_2)
}

fn main() -> Result<()> {
    env_logger::init();

    let nb_vert_array = [1000000];

    for nb_vert in nb_vert_array {
        println!("nb_vert: {}", nb_vert);
        for register in [true, false] {
            if register {
                println!("  register: true");
            } else {
                println!("  register: false");
            }
            let simpl = generate_random_simplicial_2(nb_vert, register)?;

            let nb_tests = 1000;
            let ind_nod0 = nb_vert - 1;
            let nod0 = simpl.find_node(ind_nod0).unwrap();
            let he = nod0.halfedges()[0];
            let ind_nod1 = he.last_node().value();
            let ind_nod2 = he.next().last_node().value();

            // measure time for find_node
            let now = Instant::now();
            for _ in 0..nb_tests {
                simpl.find_node(ind_nod0);
            }
            let duration = now.elapsed();
            let milli = duration.as_millis();

            println!("    find_node (x{}): {}ms", nb_tests, milli);

            // measure time for find_halfedge
            let now = Instant::now();
            for _ in 0..nb_tests {
                simpl.find_halfedge(ind_nod0, ind_nod1);
            }
            let duration = now.elapsed();
            let milli = duration.as_millis();

            println!("    find_halfedge (x{}): {}ms", nb_tests, milli);

            // measure time for find_triangle
            let now = Instant::now();
            for _ in 0..nb_tests {
                simpl.find_triangle(ind_nod0, ind_nod1, ind_nod2);
            }
            let duration = now.elapsed();
            let milli = duration.as_millis();

            println!("    find_triangle (x{}): {}ms", nb_tests, milli);

            let nb_tests = 10000000;
            // measure time for creating halfedge
            let ind_he = he.index();
            let now = Instant::now();
            for _ in 0..nb_tests {
                simpl.get_halfedge_from_index(ind_he)?;
            }
            let duration = now.elapsed();
            let milli = duration.as_millis();
            println!(
                "    simpl.get_halfedge_from_index() (x{}): {}ms",
                nb_tests, milli
            );

            let mut ind_he_tst = he.index();
            let mut he_tst = simpl.get_halfedge_from_index(ind_he_tst)?;
            // measure time for next halfedge (with iterators)
            let now = Instant::now();
            for _ in 0..nb_tests {
                he_tst = he_tst.next();
            }
            let duration = now.elapsed();
            let milli = duration.as_millis();

            println!("    IterHalfedge2.next() (x{}): {}ms", nb_tests, milli);

            // measure time for next halfedge (without iterators)
            let now = Instant::now();
            for _ in 0..nb_tests {
                ind_he_tst = simpl.halfedge_next_index(ind_he_tst);
            }
            let duration = now.elapsed();
            let milli = duration.as_millis();

            println!("    halfedge_next_index() (x{}): {}ms", nb_tests, milli);

            // measure time for previous halfedge (with iterators)
            let mut ind_he_tst = he.index();
            let mut he_tst = simpl.get_halfedge_from_index(ind_he_tst)?;
            let now = Instant::now();
            for _ in 0..nb_tests {
                he_tst = he_tst.previous();
            }
            let duration = now.elapsed();
            let milli = duration.as_millis();

            println!("    IterHalfedge2.previous() (x{}): {}ms", nb_tests, milli);

            // measure time for previous halfedge (without iterators)
            let now = Instant::now();
            for _ in 0..nb_tests {
                ind_he_tst = simpl.halfedge_previous_index(ind_he_tst);
            }
            let duration = now.elapsed();
            let milli = duration.as_millis();

            println!("    halfedge_previous_index() (x{}): {}ms", nb_tests, milli);

            // measure time for opposite halfedge (with iterators)
            let mut ind_he_tst = he.index();
            let mut he_tst = simpl.get_halfedge_from_index(ind_he_tst)?;
            let now = Instant::now();
            for _ in 0..nb_tests {
                he_tst = he_tst.opposite();
            }
            let duration = now.elapsed();
            let milli = duration.as_millis();

            println!("    IterHalfedge2.opposite() (x{}): {}ms", nb_tests, milli);

            // measure time for opposite halfedge (without iterators)
            let now = Instant::now();
            for _ in 0..nb_tests {
                ind_he_tst = simpl.halfedge_opposite_index(ind_he_tst);
            }
            let duration = now.elapsed();
            let milli = duration.as_millis();

            println!("    halfedge_opposite_index() (x{}): {}ms", nb_tests, milli);
        }
    }

    Ok(())
}
