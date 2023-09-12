use std::collections::{HashMap};
use colorful::{Color};

use permutations::Permutation;

pub trait PolyhedronDefinition {
    fn start_state(&self) -> HashMap<usize, usize>;

    fn solved_state(&self) -> HashMap<usize, usize>;

    fn get_color(&self, face_name: String) -> Color;

    fn default_get_color(&self, face_name: String) -> Color {
        match face_name.as_str() {
            "f" => Color::Red,
            "u" => Color::Yellow,
            "r" => Color::Green,
            "d" => Color::White,
            "l" => Color::Blue,
            "b" => Color::Magenta,
            _ => Color::Grey0
        }
    }

    fn get_face_name(&self, position: usize) -> String;

    fn default_face_name(&self, position: usize) -> String {
        return match position / 10 {
            0 => { "f".to_string() }
            1 => { "u".to_string() }
            2 => { "r".to_string() }
            3 => { "d".to_string() }
            4 => { "l".to_string() }
            5 => { "b".to_string() }
            _ => { panic!("Unrecognized position value : {}", position) }
        };
    }

    fn print_polyhedron(&self,
                        applied_permutations: &Permutation,
                        before_state: &HashMap<usize, usize>,
                        current_state: &HashMap<usize, usize>);

    fn get_permutation(&self, permutation_name: String) -> &Permutation;
}