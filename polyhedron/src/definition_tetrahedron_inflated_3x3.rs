use std::collections::HashMap;

use colorful::{Color, Colorful};

use permutations::Permutation;

use super::polyhedron::PolyhedronDefinition;

pub struct TetrahedronInflated3x3Definition {
    permitted_permutations: HashMap<String, Permutation>,
}

impl TetrahedronInflated3x3Definition {
    pub fn new() -> Self {
        let mut permitted_permutations = HashMap::new();

        let f = Permutation::create_permutation("f".to_string(), vec![vec![1, 6, 4], vec![2, 3, 5], vec![24, 34, 44], vec![26, 36, 46], vec![25, 35, 45]]);
        let l = Permutation::create_permutation("l".to_string(), vec![vec![41, 46, 44], vec![42, 43, 45], vec![1, 36, 21], vec![2, 33, 22], vec![4, 31, 24]]);
        let r = Permutation::create_permutation("r".to_string(), vec![vec![21, 26, 24], vec![22, 23, 25], vec![6, 46, 31], vec![3, 43, 32], vec![1, 41, 34]]);
        let d = Permutation::create_permutation("d".to_string(), vec![vec![31, 36, 34], vec![32, 33, 35], vec![4, 26, 41], vec![5, 23, 42], vec![6, 21, 44]]);

        let fi = f.inverse();
        let li = l.inverse();
        let ri = r.inverse();
        let di = d.inverse();

        permitted_permutations.insert("id".to_string(), Permutation::identity());
        permitted_permutations.insert("f".to_string(), f);
        permitted_permutations.insert("l".to_string(), l);
        permitted_permutations.insert("r".to_string(), r);
        permitted_permutations.insert("d".to_string(), d);

        permitted_permutations.insert("fi".to_string(), fi);
        permitted_permutations.insert("li".to_string(), li);
        permitted_permutations.insert("ri".to_string(), ri);
        permitted_permutations.insert("di".to_string(), di);

        return Self {
            permitted_permutations
        };
    }
}


impl PolyhedronDefinition for TetrahedronInflated3x3Definition {
    fn start_state(&self) -> HashMap<usize, usize> {
        return solved_state();
    }

    fn solved_state(&self) -> HashMap<usize, usize> {
        return solved_state();
    }

    fn get_color(&self, face_name: String) -> Color {
        return match face_name.as_str() {
            "f" => Color::Red,
            "r" => Color::Green,
            "d" => Color::Yellow,
            "l" => Color::Blue,
            _ => Color::Grey0
        };
    }

    fn get_face_name(&self, position: usize) -> String {
        return self.default_face_name(position);
    }

    fn print_polyhedron(&self,
                        applied_permutations: &Permutation,
                        before_state: &HashMap<usize, usize>,
                        current_state: &HashMap<usize, usize>) {
        let p = |position: usize| -> String {
            let before_facelet = before_state.get(&position).expect(format!("Couldn't find position {} in before state", position).as_str());
            let current_facelet = current_state.get(&position).expect(format!("Couldn't find position {} in current state", position).as_str());

            let facelet_number_str = format!("{:02}", current_facelet);
            let facename = self.get_face_name(*current_facelet);

            let color = self.get_color(facename);

            return if current_facelet == before_facelet {
                // facelet didnt move
                facelet_number_str.color(color).to_string()
            } else {
                facelet_number_str.color(Color::Black).bg_color(color).to_string()
            };
        };

        println!("");
        println!("Current permutation : {}", applied_permutations);
        println!(r"                                                 ");
        println!(r"  \---------------------//------\\---------------------/");
        println!(r"   \  {} /  {}  /  {}  //   {}   \\  {}  \  {}  \ {}  /", p(41), p(43), p(46), p(1), p(24), p(22), p(21));
        println!(r"    \   /  L   /      //----------\\      \   R  \   /");
        println!(r"     \ /      /      // {}   F  {} \\      \      \ /", p(2), p(3));
        println!(r"      \  {}  /  {}  //--------------\\  {}  \  {}  /", p(42), p(45), p(25), p(23));
        println!(r"       \    /      // {}    {}    {} \\      \    /", p(4), p(5), p(6));
        println!(r"        \  /  {}  //                  \\  {}  \  /", p(44), p(26));
        println!(r"         \/      //====================\\      \/");
        println!(r"                 \                     / ");
        println!(r"                  \   {}    {}    {}  /  ", p(36), p(35), p(34));
        println!(r"                   \  -------------- /   ");
        println!(r"                    \   {}   D  {}  /    ", p(33), p(32));
        println!(r"                     \   --------  /     ");
        println!(r"                      \     {}    /      ", p(31));
        println!(r"                       \  -----  /       ");


        if self.solved_state() == current_state.clone() && before_state.clone() != current_state.clone() {
            println!("{}", "...................................".gradient(Color::Red));
            println!("{}{}{}", "......".gradient(Color::Red),
                     "      Solved      ".color(Color::Red).blink(),
                     "...........".gradient(Color::Orange1));
            println!("{}", "...................................".gradient(Color::Red));
        }
    }

    fn get_permutation(&self, permutation_name: String) -> &Permutation {
        return self.permitted_permutations.get(&permutation_name)
            .expect(format!("unrecognized rotation name. {}", permutation_name).as_str());
    }
}

fn solved_state() -> HashMap<usize, usize> {
    return HashMap::from([
        // F
        (1, 1),
        (2, 2),
        (3, 3),
        (4, 4),
        (5, 5),
        (6, 6),

        // R
        (21, 21),
        (22, 22),
        (23, 23),
        (24, 24),
        (25, 25),
        (26, 26),

        // D
        (31, 31),
        (32, 32),
        (33, 33),
        (34, 34),
        (35, 35),
        (36, 36),

        // L
        (41, 41),
        (42, 42),
        (43, 43),
        (44, 44),
        (45, 45),
        (46, 46),
    ]);
}