use std::collections::{HashMap};
use colorful::{Color};

use permutations::Permutation;

pub trait PolyhedronDefinition {
    fn start_state(&self) -> HashMap<usize, usize>;

    fn solved_state(&self) -> HashMap<usize, usize>;

    fn get_color(&self, face_name: String) -> Color;

    fn get_face_name(&self, position: usize) -> String;

    fn print_polyhedron(&self,
                        applied_permutations: &Permutation,
                        before_state: &HashMap<usize, usize>,
                        current_state: &HashMap<usize, usize>);

    fn get_permutation(&self, permutation_name: String) -> &Permutation;
}