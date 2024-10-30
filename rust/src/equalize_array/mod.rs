use std::{collections::HashMap, usize};

pub fn equalize_array(arr: &[i32]) -> i32 {
    let mut frequencies: HashMap<&i32, usize> = HashMap::new();
    for num in arr {
        *frequencies.entry(num).or_default() += 1;
    }
    let max = frequencies.into_iter().max_by_key(|(_, val)| *val);
    (arr.len() - max.unwrap().1) as i32
}
