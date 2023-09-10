use polyhedron::Polyhedron;
use polyhedron::cube2x2::Cube2x2Definition;

fn main() {
    let cube2x2_definition = Box::new(Cube2x2Definition::new());
    let mut cube2x2 = Polyhedron::create_polyhedron(cube2x2_definition);

    cube2x2.print_polyhedron();
    cube2x2.rotate("f");
    cube2x2.rotate_many(vec!["f", "f", "f"]);
    cube2x2.print_polyhedron();
}
