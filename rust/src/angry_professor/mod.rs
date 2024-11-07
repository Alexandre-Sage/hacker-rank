fn angry_professor(arrival_threshold: i32, students: &[i32]) -> String {
    let on_time_arrival = students
        .to_vec()
        .into_iter()
        .filter(|x| *x <= 0)
        .collect::<Vec<i32>>()
        .len() as i32;
    if on_time_arrival >= arrival_threshold {
        "NO".to_owned()
    } else {
        "YES".to_owned()
    }
}
#[cfg(test)]
mod test {
    use crate::angry_professor::angry_professor;

    #[test]
    fn first_test() {
        let arrival_threshold = 3;
        let students = &[-1, -3, 4, 2];
        assert_eq!(angry_professor(arrival_threshold, students), "YES")
    }
    #[test]
    fn second_test() {
        let arrival_threshold = 2;
        let students = &[0, -1, 2, 1];
        assert_eq!(angry_professor(arrival_threshold, students), "NO")
    }
}
