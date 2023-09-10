use std::collections::{BTreeMap, HashMap};
use colorful::{Color, Colorful};
use permutations::Permutation;
use phf::phf_map;

mod cube2x2;

static COLOR_MAP: phf::Map<&'static str, Color> = phf_map! {
    "F" => Color::Red,
    "U" => Color::Yellow,
    "R" => Color::Green,
    "D" => Color::White,
    "L" => Color::Blue,
    "B" => Color::Magenta,
};


pub struct Cube {
    solved_state: HashMap<usize, usize>,
    start_state: HashMap<usize, usize>,
    current_state: HashMap<usize, usize>,
    before_state: HashMap<usize, usize>,

    applied_permutations: Permutation,
    possible_permutations: Vec<Permutation>,

}

impl Cube {
    pub fn create_cube() -> Self {
        let solved_state: HashMap<usize, usize> = HashMap::from([
            // F
            (1, 1),
            (2, 2),
            (3, 3),
            (4, 4),

            // U
            (11, 11),
            (12, 12),
            (13, 13),
            (14, 14),

            // R
            (21, 21),
            (22, 22),
            (23, 23),
            (24, 24),

            // D
            (31, 31),
            (32, 32),
            (33, 33),
            (34, 34),

            // L
            (41, 41),
            (42, 42),
            (43, 43),
            (44, 44),

            // B
            (51, 51),
            (52, 52),
            (53, 53),
            (54, 54)
        ]);

        Self {
            solved_state: solved_state.clone(),
            before_state: solved_state.clone(),
            applied_permutations: Permutation::identity(),
            start_state: solved_state.clone(),
            current_state: solved_state.clone(),
            possible_permutations: vec![],
        }
    }

    pub fn rotate(&mut self, perm: Permutation) {
        self.applied_permutations = self.applied_permutations.compose(&perm);

        self.before_state = self.current_state.clone();

        for (key, _) in self.solved_state {
            let old_key = perm.apply(key);
            let new_state = self.before_state.get(&old_key).unwrap();

            self.current_state.insert(key, new_state.clone());
        }
    }

    fn get_face_name(position: usize) -> String {
        return match position / 10 {
            0 => { "F".to_string() }
            1 => { "U".to_string() }
            2 => { "R".to_string() }
            3 => { "D".to_string() }
            4 => { "L".to_string() }
            5 => { "B".to_string() }
            _ => { panic!("Unrecognized position value : {}", position) }
        };
    }

    fn p(&self, position: usize) -> String {
        let before_facelet = self.before_state.get(&position).unwrap();
        let current_facelet = self.current_state.get(&position).unwrap();

        let facelet_number_str = format!("{:02}", current_facelet);
        let facename = Cube::get_face_name(position);

        let color = COLOR_MAP.get(facename.as_str()).unwrap();

        return if current_facelet == before_facelet {
            facelet_number_str.color(*color).to_string()
        } else {
            facelet_number_str.color(*color).bg_color(Color::Black).to_string()
        };
    }

    pub fn print_cube(&self) {
        println!("Current permutation : {}", self.applied_permutations);
        println!("          +----+----+");
        println!("          | {} | {} |", self.p(11), self.p(12));
        println!("          +--- U ---+");
        println!("          | {} | {} |", self.p(14), self.p(13));
        println!("+----+----+----+----+----+----+----+----+");
        println!("| {} | {} | {} | {} | {} | {} | {} | {} |", self.p(41), self.p(42), self.p(1), self.p(2), self.p(21), self.p(22), self.p(51), self.p(52));
        println!("+--- L ---+--- F ---+--- R ---+--- B ---+");
        println!("| {} | {} | {} | {} | {} | {} | {} | {} |", self.p(44), self.p(43), self.p(4), self.p(3), self.p(24), self.p(23), self.p(54), self.p(53));
        println!("+----+----+----+----+----+----+----+----+");
        println!("          | {} | {} |", self.p(31), self.p(32));
        println!("          +--- D ---+");
        println!("          | {} | {} |", self.p(34), self.p(33));
        println!("          +----+----+");
    }

    pub fn print_current_state(&self) {
        #[cfg(windows)]
        control::set_virtual_terminal(true);

        let sorted_state = BTreeMap::from_iter(self.current_state.clone());

        println!("let solved_state: HashMap<usize, usize> = HashMap::from([");

        let mut face = "".to_string();
        for (key, value) in sorted_state {
            let current_face = Cube::get_face_name(key);

            if current_face != face {
                println!("        // {}", current_face);
            }

            face = current_face;
            println!("{}", format!(" ({}, {}),", key, value));
        }

        println!("]);")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let cube = Cube::create_cube();

        cube.print_cube();

    }
}
