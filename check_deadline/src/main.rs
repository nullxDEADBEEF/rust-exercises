use chrono::{Date, Local, TimeZone};

trait Deadline {
    fn is_passed(&self) -> bool;
}

struct ImportantEvent {
    name: String,
    date: Date<Local>,
}

impl Deadline for ImportantEvent {
    fn is_passed(&self) -> bool {
        self.date < Local::today()
    }
}

fn main() {
    let missed_christmas = ImportantEvent {
        name: String::from("Christmas"),
        date: Local.ymd(2021, 12, 24),
    };

    if missed_christmas.is_passed() {
        println!("We will try again next year");
    } else {
        println!("☃︎");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn in_past() {
        let event = ImportantEvent {
            name: String::from("Friend's birthday"),
            date: Local::today() - Duration::hours(25),
        };

        assert!(event.is_passed());
    }

    #[test]
    fn in_future() {
        let event = ImportantEvent {
            name: String::from("Friend's birthday"),
            date: Local::today() + Duration::hours(25),
        };

        assert!(!event.is_passed())
    }
}
