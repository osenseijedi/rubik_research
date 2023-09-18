use polyhedron::Polyhedron;

fn main() {
    {
        let cube2x2 = Polyhedron::create_polyhedron(Box::new(polyhedron::cube2x2::Cube2x2Definition::new()));
        bevy_viewer::show_in_bevy(cube2x2);

        // cube2x2.print_polyhedron();
        // cube2x2.rotate("f");
        // cube2x2.rotate_many(vec!["f", "f", "f"]);
        // cube2x2.print_polyhedron();
        // println!("---------------------------------------");
        // println!("---------------------------------------");
        // println!("---------------------------------------");
        // println!("---------------------------------------");
    }

    // {
    //     use polyhedron::tetrahedron_inflated_3x3::TetrahedronInflated3x3Definition;
    //     let tetra = Box::new(polyhedron::tetrahedron_inflated_3x3::TetrahedronInflated3x3Definition::new());
    //     let mut tetra3x3 = Polyhedron::create_polyhedron(tetra);
    //
    //     tetra3x3.print_polyhedron();
    //     tetra3x3.rotate_many(vec!["f", "l", "r", "d"]);
    //     tetra3x3.print_polyhedron();
    //     println!("---------------------------------------");
    //     println!("---------------------------------------");
    //     println!("---------------------------------------");
    //     println!("---------------------------------------");
    // }



}
