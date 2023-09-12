use std::collections::HashMap;

use colorful::{Color, Colorful};

use permutations::Permutation;

use super::polyhedron::PolyhedronDefinition;

pub struct Cube2x2Definition {
    permitted_permutations: HashMap<String, Permutation>,
}

impl Cube2x2Definition {
    pub fn new() -> Self {
        let mut permitted_permutations = HashMap::new();

        let f = Permutation::create_permutation("f".to_string(), vec![vec![1, 4, 3, 2], vec![13, 42, 31, 24], vec![14, 43, 32, 21]]);
        let l = Permutation::create_permutation("l".to_string(), vec![vec![1, 11, 53, 31], vec![4, 14, 52, 34], vec![41, 44, 43, 42]]);
        let u = Permutation::create_permutation("u".to_string(), vec![vec![1, 21, 51, 41], vec![2, 22, 52, 42], vec![11, 14, 13, 12]]);
        let r = Permutation::create_permutation("r".to_string(), vec![vec![2, 32, 54, 12], vec![3, 33, 51, 13], vec![21, 24, 23, 22]]);
        let d = Permutation::create_permutation("d".to_string(), vec![vec![3, 43, 53, 23], vec![4, 44, 54, 24], vec![31, 34, 33, 32]]);
        let b = Permutation::create_permutation("b".to_string(), vec![vec![11, 22, 33, 44], vec![12, 23, 34, 41], vec![51, 54, 53, 52]]);

        let fi = f.inverse();
        let li = l.inverse();
        let ui = u.inverse();
        let ri = r.inverse();
        let di = d.inverse();
        let bi = b.inverse();

        let a_tech_right = Permutation::create_permutation_from_composition("a_tech_right".to_string(), vec![&f, &di, &fi, &di, &ri, &d, &r]);
        let b_tech_right = Permutation::create_permutation_from_composition("b_tech_right".to_string(), vec![&f, &di, &fi, &di, &di, &ri, &d, &d, &r]);
        let a_left_left = Permutation::create_permutation_from_composition("a_left_left".to_string(), vec![&fi, &d, &f, &d, &l, &di, &li]);
        let b_tech_left = Permutation::create_permutation_from_composition("b_tech_left".to_string(), vec![&fi, &d, &f, &d, &d, &l, &di, &di, &li]);
        let c_tech_right = Permutation::create_permutation_from_composition("c_tech_right".to_string(), vec![&di, &fi, &d, &fi, &di, &f, &f, &d]);
        let d_tech_right = Permutation::create_permutation_from_composition("d_tech_right".to_string(), vec![&di, &fi, &fi, &d, &f, &di, &f, &d]);
        let c_tech_left = Permutation::create_permutation_from_composition("c_tech_left".to_string(), vec![]);
        let d_tech_left = Permutation::create_permutation_from_composition("d_tech_left".to_string(), vec![]);

        permitted_permutations.insert("id".to_string(), Permutation::identity());
        permitted_permutations.insert("f".to_string(), f);
        permitted_permutations.insert("u".to_string(), l);
        permitted_permutations.insert("r".to_string(), u);
        permitted_permutations.insert("d".to_string(), r);
        permitted_permutations.insert("l".to_string(), d);
        permitted_permutations.insert("b".to_string(), b);

        permitted_permutations.insert("fi".to_string(), fi);
        permitted_permutations.insert("li".to_string(), li);
        permitted_permutations.insert("ui".to_string(), ui);
        permitted_permutations.insert("ri".to_string(), ri);
        permitted_permutations.insert("di".to_string(), di);
        permitted_permutations.insert("bi".to_string(), bi);

        permitted_permutations.insert("a_tech_right".to_string(), a_tech_right);
        permitted_permutations.insert("b_tech_right".to_string(), b_tech_right);
        permitted_permutations.insert("a_left_left".to_string(), a_left_left);
        permitted_permutations.insert("b_tech_left".to_string(), b_tech_left);
        permitted_permutations.insert("c_tech_right".to_string(), c_tech_right);
        permitted_permutations.insert("d_tech_right".to_string(), d_tech_right);
        permitted_permutations.insert("c_tech_left".to_string(), c_tech_left);
        permitted_permutations.insert("d_tech_left".to_string(), d_tech_left);

        return Self {
            permitted_permutations
        };
    }
}

impl PolyhedronDefinition for Cube2x2Definition {
    fn start_state(&self) -> HashMap<usize, usize> {
        return solved_state();
        // return diagonal_fixed();
        // return flipped_corners();
    }
    fn solved_state(&self) -> HashMap<usize, usize> {
        return solved_state();
    }

    fn get_color(&self, face_name: String) -> Color {
        return self.default_get_color(face_name);
    }

    fn get_face_name(&self, position: usize) -> String {
        return self.default_face_name(position);
    }

    fn get_permutation(&self, permutation_name: String) -> &Permutation {
        return self.permitted_permutations.get(&permutation_name)
            .expect(format!("unrecognized rotation name. {}", permutation_name).as_str());
    }

    fn print_polyhedron(&self,
                        applied_permutations: &Permutation,
                        before_state: &HashMap<usize, usize>,
                        current_state: &HashMap<usize, usize>) {
        let p = |position: usize| -> String {
            let before_facelet = before_state.get(&position).unwrap();
            let current_facelet = current_state.get(&position).unwrap();

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
        println!("          +----+----+");
        println!("          | {} | {} |", p(11), p(12));
        println!("          +--- u ---+");
        println!("          | {} | {} |", p(14), p(13));
        println!("+----+----+----+----+----+----+----+----+");
        println!("| {} | {} | {} | {} | {} | {} | {} | {} |", p(41), p(42), p(1), p(2), p(21), p(22), p(51), p(52));
        println!("+--- l ---+--- f ---+--- r ---+--- b ---+");
        println!("| {} | {} | {} | {} | {} | {} | {} | {} |", p(44), p(43), p(4), p(3), p(24), p(23), p(54), p(53));
        println!("+----+----+----+----+----+----+----+----+");
        println!("          | {} | {} |", p(31), p(32));
        println!("          +--- d ---+");
        println!("          | {} | {} |", p(34), p(33));
        println!("          +----+----+");

        if self.solved_state() == current_state.clone() && before_state.clone() != current_state.clone() {
            println!("{}", "...................................".gradient(Color::Red));
            println!("{}{}{}", "......".gradient(Color::Red),
                     "      Solved      ".color(Color::Red).blink(),
                     "...........".gradient(Color::Orange1));
            println!("{}", "...................................".gradient(Color::Red));
        }
    }
}


fn solved_state() -> HashMap<usize, usize> {
    return HashMap::from([
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
}

#[allow(dead_code)]
fn diagonal_fixed() -> HashMap<usize, usize> {
    return HashMap::from([

        // F
        (1, 1),
        (2, 2),
        (3, 32),
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
        (24, 3),
        // D
        (31, 31),
        (32, 24),
        (33, 33),
        (34, 53),
        // L
        (41, 41),
        (42, 42),
        (43, 43),
        (44, 34),
        // B
        (51, 51),
        (52, 52),
        (53, 44),
        (54, 54),
    ]);
}

#[allow(dead_code)]
fn flipped_corners() -> HashMap<usize, usize> {
    return HashMap::from([
        // F
        (1, 1),
        (2, 2),
        (3, 54),
        (4, 4),
        // U
        (11, 11),
        (12, 12),
        (13, 13),
        (14, 14),
        // R
        (21, 21),
        (22, 22),
        (23, 32),
        (24, 33),
        // D
        (31, 31),
        (32, 23),
        (33, 24),
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
        (54, 3),
    ]);
}

