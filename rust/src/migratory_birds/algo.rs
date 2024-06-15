use std::collections::HashMap;

#[allow(dead_code)]
pub fn migratory_birds(input: &[i32]) -> i32 {
    input
        .iter()
        .fold(HashMap::<i32, usize>::new(), |mut hash_map, current| {
            *hash_map.entry(*current).or_default() += 1;
            hash_map
        })
        .into_iter()
        .reduce(|acc, cur| {
            if acc.1 < cur.1 {
                cur
            } else if acc.1 == cur.1 && acc.0 > cur.0 {
                cur
            } else {
                acc
            }
        })
        .map(|(key, _value)| key)
        .unwrap()
}

#[cfg(test)]
pub mod test {
    use crate::migratory_birds::inputs::{FIRST_INPUT, SECOND_INPUT};

    use super::migratory_birds;

    pub fn test(input: &[i32], expected: i32) {
        let result = migratory_birds(input);
        assert_eq!(result, expected);
    }
    #[test]
    fn first_case() {
        test(&FIRST_INPUT, 4)
    }
    #[test]
    fn second_case() {
        test(&SECOND_INPUT, 3)
    }
}
