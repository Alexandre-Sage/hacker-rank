fn lily_beautiful_days(starting_day: i32, end_day: i32, divisor: i32) -> i32 {
    let mut count = 0;
    for day in starting_day..end_day + 1 {
        let reversed = day.to_string().chars().rev().collect::<String>();
        let reversed = reversed
            .parse::<i32>()
            .expect(format!("Unable to parse: {:?}", day).as_str());
        let evenly: f32 = (day - reversed) as f32 / divisor as f32;
        if evenly.fract() == 0.0 {
            count += 1
        }
    }
    count
}

#[cfg(test)]
mod test {
    use crate::beautiful_day::lily_beautiful_days;

    #[test]
    fn case() {
        let starting_day = 20;
        let end_day = 23;
        let divisor = 6;
        assert_eq!(lily_beautiful_days(starting_day, end_day, divisor), 2)
    }
}
