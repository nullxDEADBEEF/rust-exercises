use chrono::NaiveDate;

fn weeks_between(a: &str, b: &str) -> i32 {
    let date1 = NaiveDate::parse_from_str(a, "%Y-%m-%d").unwrap();
    let date2 = NaiveDate::parse_from_str(b, "%Y-%m-%d").unwrap();

    (date2 - date1).num_weeks() as i32
}

fn main() {
    let weeks = weeks_between("2022-03-21", "2022-03-28");
    println!("Weeks between: {weeks}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_day() {
        let weeks = weeks_between("1010-10-10", "1010-10-10");
        assert_eq!(weeks, 0);
    }

    #[test]
    fn one_week() {
        let weeks = weeks_between("1010-10-10", "1010-10-18");
        assert_eq!(weeks, 1);
    }

    #[test]
    fn past() {
        let weeks = weeks_between("1010-10-18", "1010-10-10");
        assert_eq!(weeks, -1);
    }
}
