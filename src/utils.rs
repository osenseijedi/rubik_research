
pub fn next_false(table: &Vec<bool>, starting_position: usize) -> Option<usize>{
    for index in starting_position..table.len().clone() {
        if !table[index] {
            return Some(index.clone());
        }
    }

    return None;
}

