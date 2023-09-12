use polyhedron::Polyhedron;

fn main() {
    {
        use polyhedron::cube2x2::Cube2x2Definition;
        let cube2x2_definition = Box::new(Cube2x2Definition::new());
        let mut cube2x2 = Polyhedron::create_polyhedron(cube2x2_definition);

        cube2x2.print_polyhedron();
        cube2x2.rotate("f");
        cube2x2.rotate_many(vec!["f", "f", "f"]);
        cube2x2.print_polyhedron();
        println!("---------------------------------------");
        println!("---------------------------------------");
        println!("---------------------------------------");
        println!("---------------------------------------");
    }

    {
        use polyhedron::tetrahedron_inflated_3x3::TetrahedronInflated3x3Definition;
        let tetra = Box::new(TetrahedronInflated3x3Definition::new());
        let mut tetra3x3 = Polyhedron::create_polyhedron(tetra);

        tetra3x3.print_polyhedron();
        tetra3x3.rotate("f");
        tetra3x3.rotate_many(vec!["f", "f", "f"]);
        tetra3x3.print_polyhedron();
        println!("---------------------------------------");
        println!("---------------------------------------");
        println!("---------------------------------------");
        println!("---------------------------------------");
    }
}
