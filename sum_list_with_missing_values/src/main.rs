fn sum_with_missing(numbers: Vec<Option<i32>>) -> i32 {
    numbers.iter().map(|n| n.unwrap_or(0)).sum()
}

fn main() {
    let numbers = vec![Some(1), Some(5), Some(4)];
    println!("{}", sum_with_missing(numbers));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum_with_missing(vec![]), 0);
    }

    #[test]
    fn no_missing() {
        let numbers = vec![Some(1), Some(5), Some(4)];
        assert_eq!(sum_with_missing(numbers), 10);
    }

    #[test]
    fn some_missing() {
        let numbers = vec![None, Some(1), Some(5), Some(4), None, None];
        assert_eq!(sum_with_missing(numbers), 10);
    }

    #[test]
    fn all_missing() {
        let numbers = vec![None, None, None];
        assert_eq!(sum_with_missing(numbers), 0);
    }
}
