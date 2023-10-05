///
/// Crate that modelise permutations.
///
/// ```
/// let f = Permutation::create_permutation("f".to_string(), vec![vec![1, 4, 3, 2], vec![13, 42, 31, 24], vec![14, 43, 32, 21]]);
/// let l = Permutation::create_permutation("l".to_string(), vec![vec![1, 11, 53, 31], vec![4, 14, 52, 34], vec![41, 44, 43, 42]]);///
///
/// assert_eq!(compose(&f, &l),
///                    Permutation::create_permutation(
///                        "f * l".to_string(),
///                        vec![vec![1, 11, 53, 24, 13, 42, 41, 44, 32, 21, 14, 52, 34, 3, 2], vec![4, 43, 31]])
///         );
/// ```
///
/// Composition doesn't follow the "standard" in the sense that if you want to run f then g, instead of writing you would write
/// ```
/// compose(f, g)
/// ```
/// It is equivalent to the mathematical g(f()).
///
/// You can create permutation from one line, cycle notation or a composition of permutations.
///
///

use std::cmp;
use std::cmp::max;
use std::fmt::{Display, Formatter};


#[derive(PartialEq, Debug)]
pub struct Permutation {
    name: String,

    one_line_permutation: Vec<usize>,
    is_identity: bool,
    internal_degree: usize,
}


pub fn compose(p1: &Permutation, p2: &Permutation) -> Permutation {
    let new_degree = max(p1.degree(), p2.degree());
    let mut result_one_line = vec![0; new_degree];

    #[allow(clippy::needless_range_loop)]
    for i in 0..new_degree {
        result_one_line[i] = p1.apply(p2.apply(i));
    }

    let name = format!("{} * {}", p1.name.clone(), p2.name.clone());
    return Permutation::create_permutation_one_line(name, result_one_line);
}

pub fn compose_2(p1: &Permutation, p2: &Permutation) -> Permutation {
    return compose(p1, p2);
}

pub fn compose_3(p1: &Permutation,
                 p2: &Permutation,
                 p3: &Permutation) -> Permutation {
    return compose(&compose(p1, p2), p3);
}

pub fn compose_4(p1: &Permutation,
                 p2: &Permutation,
                 p3: &Permutation,
                 p4: &Permutation) -> Permutation {
    return compose(&compose(&compose(p1, p2), p3), p4);
}

pub fn compose_5(p1: &Permutation,
                 p2: &Permutation,
                 p3: &Permutation,
                 p4: &Permutation,
                 p5: &Permutation) -> Permutation {
    return compose(&compose(&compose(&compose(p1, p2), p3), p4), p5);
}

pub fn compose_6(p1: &Permutation,
                 p2: &Permutation,
                 p3: &Permutation,
                 p4: &Permutation,
                 p5: &Permutation,
                 p6: &Permutation) -> Permutation {
    return compose(&compose(&compose(&compose(&compose(p1, p2), p3), p4), p5), p6);
}

pub fn compose_7(p1: &Permutation,
                 p2: &Permutation,
                 p3: &Permutation,
                 p4: &Permutation,
                 p5: &Permutation,
                 p6: &Permutation,
                 p7: &Permutation) -> Permutation {
    return compose(&compose(&compose(&compose(&compose(&compose(p1, p2), p3), p4), p5), p6), p7);
}

pub fn compose_8(p1: &Permutation,
                 p2: &Permutation,
                 p3: &Permutation,
                 p4: &Permutation,
                 p5: &Permutation,
                 p6: &Permutation,
                 p7: &Permutation,
                 p8: &Permutation) -> Permutation {
    return compose(&compose(&compose(&compose(&compose(&compose(&compose(p1, p2), p3), p4), p5), p6), p7), p8);
}

pub fn compose_9(p1: &Permutation,
                 p2: &Permutation,
                 p3: &Permutation,
                 p4: &Permutation,
                 p5: &Permutation,
                 p6: &Permutation,
                 p7: &Permutation,
                 p8: &Permutation,
                 p9: &Permutation) -> Permutation {
    return compose(&compose(&compose(&compose(&compose(&compose(&compose(&compose(p1, p2), p3), p4), p5), p6), p7), p8), p9);
}

pub fn compose_n(permutations: Vec<&Permutation>) -> Permutation {
    if permutations.is_empty() {
        return Permutation::identity();
    }

    let first_element = permutations[0];
    let mut result = Permutation::create_permutation_one_line(first_element.name.clone(), first_element.one_line_permutation.clone());

    if permutations.len() > 1 {
        for permutation in &permutations[1..] {
            result = compose(&result, permutation);
        }
    }

    return result;
}


impl Permutation {
    pub fn identity() -> Self {
        return Permutation::create_permutation_one_line("id".to_string(), vec![]);
    }

    pub fn create_permutation_one_line(name: String, one_line_notation: Vec<usize>) -> Self {
        return Self {
            name,
            is_identity: is_identity(&one_line_notation),
            internal_degree: internal_degree(&one_line_notation),
            one_line_permutation: one_line_notation,
        };
    }

    pub fn create_permutation(name: String, cycles: Vec<Vec<usize>>) -> Self {
        let one_line_notation = convert_cycles_to_one_line(&cycles);
        return Permutation::create_permutation_one_line(name, one_line_notation);
    }

    pub fn create_permutation_from_composition(name: String, permutations: Vec<&Permutation>) -> Self {
        let mut p = compose_n(permutations);
        p.set_name(name);

        return p;
    }

    fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }

    pub fn degree(&self) -> usize {
        return internal_degree(&self.one_line_permutation);
    }


    pub fn inverse(&self) -> Self {
        if self.is_identity {
            return Permutation::create_permutation_one_line(self.name.clone(), vec![]);
        }

        let mut inv_one_line = vec![0; self.one_line_permutation.len()];

        for i in (0..(self.one_line_permutation.len())).rev() {
            inv_one_line[self.one_line_permutation[i]] = i;
        }

        return Permutation::create_permutation_one_line(format!("{}i", self.name.clone()), inv_one_line);
    }

    pub fn apply(&self, i: usize) -> usize {
        return if i < self.internal_degree {
            self.one_line_permutation[i]
        } else {
            i
        };
    }

    pub fn compose(&self, rhs: &Permutation) -> Self {
        return compose(self, rhs);
    }

    // Returns conjugation of specified element by this permutation, i.e. self * rhs * self^-1
    pub fn conjugate(&self, rhs: &Self) -> Self {
        return compose_3(self, rhs, &self.inverse());
    }

    // Returns commutator of this and specified permutation, i.e. rhs * self * rhs^-1 * self^-1
    pub fn commutator(&self, rhs: &Self) -> Self {
        return compose_4(rhs, self, &rhs.inverse(), &self.inverse());
    }

    pub fn print(&self) -> String {
        return self.to_string();
    }

    pub fn print_cycles(&self) -> String {
        return format!("{:?}", convert_one_line_to_cycles(&self.one_line_permutation));
    }

    pub fn print_one_line(&self) -> String {
        return format!("{:?}", &self.one_line_permutation);
    }

    // TODO
    // pub fn print_two_lines(&self) -> String {
    //
    // }
}


fn next_false(table: &Vec<bool>, starting_position: usize) -> Option<usize> {
    return (starting_position..table.len()).find(|&index| !table[index]);
}


fn convert_cycles_to_one_line(cycles: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut degree: usize = 0;
    for cycle in cycles {
        if let Some(max) = cycle.iter().max() {
            degree = cmp::max(degree, *max);
        }
    }

    if degree == 0 {
        return Vec::new();
    }

    degree += 1;

    let mut permutation = Vec::from_iter(0..degree);

    for current_cycle in cycles {
        if current_cycle.is_empty() { // ID
            continue;
        }

        if current_cycle.len() == 1 {
            panic!("Illegal use of cycle notation: {:?}", current_cycle);
        }

        for k in 0..(current_cycle.len() - 1) {
            permutation[current_cycle[k]] = current_cycle[k + 1]
        }
        permutation[current_cycle[current_cycle.len() - 1]] = current_cycle[0];
    }

    return permutation;
}

fn convert_one_line_to_cycles(one_line: &Vec<usize>) -> Vec<Vec<usize>> {
    let mut cycles: Vec<Vec<usize>> = Vec::new();

    let mut seen = vec![false; one_line.len()];

    let mut counter = 0;
    while counter < one_line.len() {
        let mut start = next_false(&seen, 0).expect("Coudn't find next false item");

        if one_line[start] == start {
            counter += 1;
            seen[start] = true;
            continue;
        }

        let mut cycle = Vec::new();
        while !seen[start] {
            seen[start] = true;
            counter += 1;

            cycle.push(start);
            start = one_line[start];
        }

        cycles.push(cycle);
    }

    return cycles;
}

impl Display for Permutation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.name);
    }
}


/**
 * Returns true if specified permutation, written in one-line notation, is identity and false otherwise.
 */
#[allow(clippy::needless_range_loop)]
fn is_identity(one_line: &Vec<usize>) -> bool {
    for i in 0..one_line.len() {
        if i != one_line[i] {
            return false;
        }
    }

    return true;
}

/**
 * Calculates <i>degree</i> of permutation, i.e.larges point moved by specified permutation plus one.
 */
fn internal_degree(one_line: &Vec<usize>) -> usize {
    let mut i = one_line.len() as i32 - 1;

    while i >= 0 {
        let index = i as usize;

        if one_line[index] != index {
            break;
        }

        i -= 1;
    }

    return (i + 1) as usize;
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn to_one_line() {
        let id_one_line: Vec<usize> = vec![];
        let id_cycles: Vec<Vec<usize>> = vec![];

        let f_one_line = vec![0, 4, 1, 2, 3, 5, 6, 7, 8, 9, 10, 11, 12, 42, 43, 15, 16, 17, 18, 19, 20, 14, 22,
                              23, 13, 25, 26, 27, 28, 29, 30, 24, 21, 33, 34, 35, 36, 37, 38, 39, 40, 41, 31, 32];
        let f_cycles = vec![vec![1, 4, 3, 2], vec![13, 42, 31, 24], vec![14, 43, 32, 21]];

        let b_one_line = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 22, 23, 13, 14, 15, 16, 17, 18, 19, 20, 21, 33,
                              34, 24, 25, 26, 27, 28, 29, 30, 31, 32, 44, 41, 35, 36, 37, 38, 39, 40, 12, 42, 43,
                              11, 45, 46, 47, 48, 49, 50, 54, 51, 52, 53];
        let b_cycles = vec![vec![11, 22, 33, 44], vec![12, 23, 34, 41], vec![51, 54, 53, 52]];

        assert_eq!(convert_cycles_to_one_line(&id_cycles), id_one_line);
        assert_eq!(convert_cycles_to_one_line(&vec![vec![]]), id_one_line);
        assert_eq!(convert_cycles_to_one_line(&vec![vec![], vec![]]), id_one_line);

        assert_eq!(convert_cycles_to_one_line(&f_cycles), f_one_line);
        assert_eq!(convert_cycles_to_one_line(&b_cycles), b_one_line);
    }

    #[test]
    fn one_line_to_cycles() {
        let id_one_line: Vec<usize> = vec![];
        let id_cycles: Vec<Vec<usize>> = vec![];

        let f_one_line = vec![0, 4, 1, 2, 3, 5, 6, 7, 8, 9, 10, 11, 12, 42, 43, 15, 16, 17, 18, 19, 20, 14, 22,
                              23, 13, 25, 26, 27, 28, 29, 30, 24, 21, 33, 34, 35, 36, 37, 38, 39, 40, 41, 31, 32];
        let f_cycles = vec![vec![1, 4, 3, 2], vec![13, 42, 31, 24], vec![14, 43, 32, 21]];

        let b_one_line = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 22, 23, 13, 14, 15, 16, 17, 18, 19, 20, 21, 33,
                              34, 24, 25, 26, 27, 28, 29, 30, 31, 32, 44, 41, 35, 36, 37, 38, 39, 40, 12, 42, 43,
                              11, 45, 46, 47, 48, 49, 50, 54, 51, 52, 53];
        let b_cycles = vec![vec![11, 22, 33, 44], vec![12, 23, 34, 41], vec![51, 54, 53, 52]];

        assert_eq!(convert_one_line_to_cycles(&id_one_line), id_cycles);
        assert_eq!(convert_one_line_to_cycles(&f_one_line), f_cycles);
        assert_eq!(convert_one_line_to_cycles(&b_one_line), b_cycles);
    }

    #[test]
    fn test_identity() {
        let id_one_line = vec![];
        let f_one_line = vec![0, 4, 1, 2, 3, 5, 6, 7, 8, 9, 10, 11, 12, 42, 43, 15, 16, 17, 18, 19, 20, 14, 22,
                              23, 13, 25, 26, 27, 28, 29, 30, 24, 21, 33, 34, 35, 36, 37, 38, 39, 40, 41, 31, 32];
        let b_one_line = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 22, 23, 13, 14, 15, 16, 17, 18, 19, 20, 21, 33,
                              34, 24, 25, 26, 27, 28, 29, 30, 31, 32, 44, 41, 35, 36, 37, 38, 39, 40, 12, 42, 43,
                              11, 45, 46, 47, 48, 49, 50, 54, 51, 52, 53];

        assert!(is_identity(&id_one_line));
        assert!(!is_identity(&f_one_line));
        assert!(!is_identity(&b_one_line));
    }

    #[test]
    fn test_internal_degree() {
        let id_one_line = vec![];
        let f_one_line = vec![0, 4, 1, 2, 3, 5, 6, 7, 8, 9, 10, 11, 12, 42, 43, 15, 16, 17, 18, 19, 20, 14, 22,
                              23, 13, 25, 26, 27, 28, 29, 30, 24, 21, 33, 34, 35, 36, 37, 38, 39, 40, 41, 31, 32];
        let b_one_line = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 22, 23, 13, 14, 15, 16, 17, 18, 19, 20, 21, 33,
                              34, 24, 25, 26, 27, 28, 29, 30, 31, 32, 44, 41, 35, 36, 37, 38, 39, 40, 12, 42, 43,
                              11, 45, 46, 47, 48, 49, 50, 54, 51, 52, 53];


        assert_eq!(internal_degree(&id_one_line), 0);
        assert_eq!(internal_degree(&f_one_line), 44);
        assert_eq!(internal_degree(&b_one_line), 55);
    }

    #[test]
    fn to_string_cycles() {
        let id: Permutation = Permutation::identity();
        let f: Permutation = Permutation::create_permutation("f".to_string(), vec![vec![1, 4, 3, 2], vec![13, 42, 31, 24], vec![14, 43, 32, 21]]);
        let l: Permutation = Permutation::create_permutation("l".to_string(), vec![vec![1, 11, 53, 31], vec![4, 14, 52, 34], vec![41, 44, 43, 42]]);
        let u: Permutation = Permutation::create_permutation("u".to_string(), vec![vec![1, 21, 51, 41], vec![2, 22, 52, 42], vec![11, 14, 13, 12]]);
        let r: Permutation = Permutation::create_permutation("r".to_string(), vec![vec![2, 32, 54, 12], vec![3, 33, 51, 13], vec![21, 24, 23, 22]]);
        let d: Permutation = Permutation::create_permutation("d".to_string(), vec![vec![3, 43, 53, 23], vec![4, 44, 54, 24], vec![31, 34, 33, 32]]);
        let b: Permutation = Permutation::create_permutation("b".to_string(), vec![vec![11, 22, 33, 44], vec![12, 23, 34, 41], vec![51, 54, 53, 52]]);

        assert_eq!("[]".to_string(), id.print_cycles());
        assert_eq!("[[1, 4, 3, 2], [13, 42, 31, 24], [14, 43, 32, 21]]".to_string(), f.print_cycles());
        assert_eq!("[[1, 11, 53, 31], [4, 14, 52, 34], [41, 44, 43, 42]]".to_string(), l.print_cycles());
        assert_eq!("[[1, 21, 51, 41], [2, 22, 52, 42], [11, 14, 13, 12]]".to_string(), u.print_cycles());
        assert_eq!("[[2, 32, 54, 12], [3, 33, 51, 13], [21, 24, 23, 22]]".to_string(), r.print_cycles());
        assert_eq!("[[3, 43, 53, 23], [4, 44, 54, 24], [31, 34, 33, 32]]".to_string(), d.print_cycles());
        assert_eq!("[[11, 22, 33, 44], [12, 23, 34, 41], [51, 54, 53, 52]]".to_string(), b.print_cycles());

        let idi: Permutation = id.inverse();
        let fi: Permutation = f.inverse();
        let li: Permutation = l.inverse();
        let ui: Permutation = u.inverse();
        let ri: Permutation = r.inverse();
        let di: Permutation = d.inverse();
        let bi: Permutation = b.inverse();

        assert_eq!("[]".to_string(), idi.print_cycles());
        assert_eq!("[[1, 2, 3, 4], [13, 24, 31, 42], [14, 21, 32, 43]]".to_string(), fi.print_cycles());
        assert_eq!("[[1, 31, 53, 11], [4, 34, 52, 14], [41, 42, 43, 44]]".to_string(), li.print_cycles());
        assert_eq!("[[1, 41, 51, 21], [2, 42, 52, 22], [11, 12, 13, 14]]".to_string(), ui.print_cycles());
        assert_eq!("[[2, 12, 54, 32], [3, 13, 51, 33], [21, 22, 23, 24]]".to_string(), ri.print_cycles());
        assert_eq!("[[3, 23, 53, 43], [4, 24, 54, 44], [31, 32, 33, 34]]".to_string(), di.print_cycles());
        assert_eq!("[[11, 44, 33, 22], [12, 41, 34, 23], [51, 52, 53, 54]]".to_string(), bi.print_cycles());
    }

    #[test]
    fn test_compose() {
        let id: Permutation = Permutation::create_permutation("id".to_string(), vec![]);
        let f: Permutation = Permutation::create_permutation("f".to_string(), vec![vec![1, 4, 3, 2], vec![13, 42, 31, 24], vec![14, 43, 32, 21]]);
        let l: Permutation = Permutation::create_permutation("l".to_string(), vec![vec![1, 11, 53, 31], vec![4, 14, 52, 34], vec![41, 44, 43, 42]]);
        let b: Permutation = Permutation::create_permutation("b".to_string(), vec![vec![11, 22, 33, 44], vec![12, 23, 34, 41], vec![51, 54, 53, 52]]);

        assert_eq!(compose(&id, &id), Permutation { name: "id * id".to_string(), one_line_permutation: vec![], is_identity: true, internal_degree: 0 });
        assert_eq!(compose(&f, &l),
                   Permutation::create_permutation(
                       "f * l".to_string(),
                       vec![vec![1, 11, 53, 24, 13, 42, 41, 44, 32, 21, 14, 52, 34, 3, 2], vec![4, 43, 31]])
        );

        // f and b are disjoint, so the result should be the union of both
        assert_eq!(compose(&f, &b),
                   Permutation::create_permutation(
                       "f * b".to_string(),
                       vec![vec![1, 4, 3, 2], vec![11, 22, 33, 44], vec![12, 23, 34, 41],
                            vec![13, 42, 31, 24], vec![14, 43, 32, 21], vec![51, 54, 53, 52]])
        );
    }

    #[test]
    fn test_compose_3() {
        let f: Permutation = Permutation::create_permutation("f".to_string(), vec![vec![1, 4, 3, 2], vec![13, 42, 31, 24], vec![14, 43, 32, 21]]);
        let l: Permutation = Permutation::create_permutation("l".to_string(), vec![vec![1, 11, 53, 31], vec![4, 14, 52, 34], vec![41, 44, 43, 42]]);
        let b: Permutation = Permutation::create_permutation("b".to_string(), vec![vec![11, 22, 33, 44], vec![12, 23, 34, 41], vec![51, 54, 53, 52]]);

        assert_eq!(compose_3(&f, &b, &l),
                   Permutation::create_permutation(
                       "f * b * l".to_string(),
                       vec![vec![1, 22, 33, 44, 32, 21, 14, 51, 54, 53, 24, 13, 42, 12, 23, 34, 3, 2], vec![4, 43, 31], vec![11, 52, 41]])
        );
    }

    #[test]
    fn test_a_tech() {
        let f: Permutation = Permutation::create_permutation("f".to_string(), vec![vec![1, 4, 3, 2], vec![13, 42, 31, 24], vec![14, 43, 32, 21]]);
        let r: Permutation = Permutation::create_permutation("r".to_string(), vec![vec![2, 32, 54, 12], vec![3, 33, 51, 13], vec![21, 24, 23, 22]]);
        let d: Permutation = Permutation::create_permutation("d".to_string(), vec![vec![3, 43, 53, 23], vec![4, 44, 54, 24], vec![31, 34, 33, 32]]);

        let fi: Permutation = f.inverse();
        let ri: Permutation = r.inverse();
        let di: Permutation = d.inverse();

        let a_tech = compose_7(&f, &di, &fi, &di, &ri, &d, &r);

        assert_eq!(a_tech,
                   Permutation::create_permutation(
                       "f * di * fi * di * ri * d * r".to_string(),
                       vec![vec![3, 23, 32, 33, 24, 54], vec![34, 53, 44]])
        );
    }

    #[test]
    fn test_compose_n() {
        let f: Permutation = Permutation::create_permutation("f".to_string(), vec![vec![1, 4, 3, 2], vec![13, 42, 31, 24], vec![14, 43, 32, 21]]);
        let l: Permutation = Permutation::create_permutation("l".to_string(), vec![vec![1, 11, 53, 31], vec![4, 14, 52, 34], vec![41, 44, 43, 42]]);
        let b: Permutation = Permutation::create_permutation("b".to_string(), vec![vec![11, 22, 33, 44], vec![12, 23, 34, 41], vec![51, 54, 53, 52]]);

        assert_eq!(compose_n(vec![&f, &b, &l, &f, &b, &l, &f]),
                   Permutation::create_permutation("f * b * l * f * b * l * f".to_string(),
                                                   vec![vec![1, 31, 42, 43, 14, 4], vec![2, 33, 32, 51, 53, 13, 23, 3, 22, 44, 21, 54, 24, 12, 34], vec![11, 41, 52]])
        )
    }

    #[test]
    fn test_conjugate() {
        let f: Permutation = Permutation::create_permutation("f".to_string(), vec![vec![1, 4, 3, 2], vec![13, 42, 31, 24], vec![14, 43, 32, 21]]);
        let l: Permutation = Permutation::create_permutation("l".to_string(), vec![vec![1, 11, 53, 31], vec![4, 14, 52, 34], vec![41, 44, 43, 42]]);

        assert_eq!(f.conjugate(&l),
                   Permutation::create_permutation("f * l * fi".to_string(),
                                                   vec![vec![3, 43, 52, 34], vec![4, 11, 53, 24], vec![31, 41, 44, 32]])
        )
    }

    #[test]
    fn test_to_string_one_line() {
        let id: Permutation = Permutation::create_permutation("id".to_string(), vec![]);
        let f: Permutation = Permutation::create_permutation("f".to_string(), vec![vec![1, 4, 3, 2], vec![13, 42, 31, 24], vec![14, 43, 32, 21]]);
        let l: Permutation = Permutation::create_permutation("l".to_string(), vec![vec![1, 11, 53, 31], vec![4, 14, 52, 34], vec![41, 44, 43, 42]]);
        let b: Permutation = Permutation::create_permutation("b".to_string(), vec![vec![11, 22, 33, 44], vec![12, 23, 34, 41], vec![51, 54, 53, 52]]);


        assert_eq!(id.print_one_line(),
                   "[]".to_string());
        assert_eq!(f.print_one_line(),
                   "[0, 4, 1, 2, 3, 5, 6, 7, 8, 9, 10, 11, 12, 42, 43, 15, 16, 17, 18, 19, 20, 14, 22, \
                   23, 13, 25, 26, 27, 28, 29, 30, 24, 21, 33, 34, 35, 36, 37, 38, 39, 40, 41, 31, 32]".to_string());
        assert_eq!(l.print_one_line(),
                   "[0, 11, 2, 3, 14, 5, 6, 7, 8, 9, 10, 53, 12, 13, 52, 15, 16, 17, 18, 19, 20, 21, 22, \
                   23, 24, 25, 26, 27, 28, 29, 30, 1, 32, 33, 4, 35, 36, 37, 38, 39, 40, 44, 41, 42, 43, \
                   45, 46, 47, 48, 49, 50, 51, 34, 31]".to_string());
        assert_eq!(b.print_one_line(),
                   "[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 22, 23, 13, 14, 15, 16, 17, 18, 19, 20, 21, 33, \
                   34, 24, 25, 26, 27, 28, 29, 30, 31, 32, 44, 41, 35, 36, 37, 38, 39, 40, 12, 42, 43, \
                   11, 45, 46, 47, 48, 49, 50, 54, 51, 52, 53]".to_string());
    }
}