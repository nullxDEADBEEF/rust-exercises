fn unique(mut data: Vec<i32>) -> Vec<i32> {
    data.dedup_by_key(|key| *key);
    data
}

fn main() {
    println!("{:?}", unique(vec![1, 1, 3]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted_unique_list() {
        let list = vec![1, 1, 3];
        assert_eq!(unique(list), vec![1, 3]);
    }

    #[test]
    fn test_unsorted_list() {
        let list = vec![55, 22, 11, 33, 33, 20, 100];
        assert_eq!(unique(list), [55, 22, 11, 33, 20, 100]);
    }
}
