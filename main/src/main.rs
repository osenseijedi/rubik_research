use permutations::Permutation;

fn main() {
    let f = Permutation::create_permutation("f".to_string(), vec![vec![1, 4, 3, 2], vec![13, 42, 31, 24], vec![14, 43, 32, 21]]);
    let l = Permutation::create_permutation("l".to_string(), vec![vec![1, 11, 53, 31], vec![4, 14, 52, 34], vec![41, 44, 43, 42]]);

    println!("{}", permutations::compose_2(&f, &l).to_string_cycles());
}
