use std::collections::{BTreeMap, HashMap};
use permutations::Permutation;
use crate::polyhedron::PolyhedronDefinition;

pub mod polyhedron;
pub mod cube2x2;

pub struct Polyhedron {
    polyhedron_definition: Box<dyn PolyhedronDefinition>,
    solved_state: HashMap<usize, usize>,
    start_state: HashMap<usize, usize>,
    current_state: HashMap<usize, usize>,
    before_state: HashMap<usize, usize>,

    applied_permutations: Permutation,
}

impl Polyhedron {
    pub fn create_polyhedron(poly_definition: Box<dyn PolyhedronDefinition>) -> Self {
        let solved_state = poly_definition.solved_state();
        let start_state = poly_definition.start_state();

        Self {
            polyhedron_definition: poly_definition,
            solved_state: solved_state.clone(),
            before_state: start_state.clone(),
            applied_permutations: Permutation::identity(),
            current_state: start_state.clone(),
            start_state,
        }
    }

    pub fn rotate(&mut self, perm: &str) {
        let permutation = self.polyhedron_definition.get_permutation(perm.to_string());

        self.applied_permutations = self.applied_permutations.compose(permutation);

        self.before_state = self.current_state.clone();

        for (key, _) in &self.solved_state {
            let old_key = permutation.apply(*key);
            let new_state = self.before_state.get(&old_key).unwrap();

            self.current_state.insert(*key, new_state.clone());
        }
    }

    pub fn rotate_many(&mut self, perms: Vec<&str>) {
        for perm in perms {
            self.rotate(perm);
        }
    }

    pub fn reset(&mut self) {
        self.before_state = self.start_state.clone();
        self.current_state = self.start_state.clone();
        self.applied_permutations = Permutation::identity();
    }

    pub fn print_polyhedron(&self) {
        self.polyhedron_definition.print_polyhedron(&self.applied_permutations, &self.before_state, &self.current_state);
    }

    pub fn print_current_state(&self) {
        let sorted_state = BTreeMap::from_iter(self.current_state.clone());

        println!("fn xxxxxxxxxxx() -> HashMap<usize, usize> {{ \
        return HashMap::from([");

        let mut face = "".to_string();
        for (key, value) in sorted_state {
            let current_face = self.polyhedron_definition.get_face_name(key);

            if current_face != face {
                println!("        // {}", current_face);
            }

            face = current_face;
            println!("{}", format!(" ({}, {}),", key, value));
        }

        println!("    ]); \
        }}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cube2x2::Cube2x2Definition;

    #[test]
    fn it_works() {
        let polyhedron_definition = Box::new(Cube2x2Definition::new());
        let mut cube2x2 = Polyhedron::create_polyhedron(polyhedron_definition);

        cube2x2.print_polyhedron();
        cube2x2.rotate("b_tech");
        cube2x2.print_polyhedron();
    }
}
