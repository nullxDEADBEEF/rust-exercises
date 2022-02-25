fn sort_usernames(users: &mut Vec<&str>) {
    users.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
}

fn main() {
    let mut users = vec!["Todd", "amy"];
    sort_usernames(&mut users);
    println!("{:?}", users);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_sort() {
        let mut users = vec!["Todd", "amy"];
        sort_usernames(&mut users);
        assert_eq!(users, vec!["amy", "Todd"]);
    }
}
