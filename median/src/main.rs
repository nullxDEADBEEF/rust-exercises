fn median(data: Vec<f32>) -> Option<f32> {
    if data.is_empty() {
        return None;
    }
    // even list
    if data.len() & 1 != 1 {
        return Some((data[data.len() / 2] + data[(data.len() / 2) - 1]) / 2.0);
    } else {
        let middle = data.len() as f32 / 2.0;
        Some(data[middle as usize])
    }
}

fn main() {
    println!("{:?}", median(vec![]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_median_empty_list() {
        assert_eq!(None, median(vec![]));
    }

    #[test]
    fn test_median_even_list() {
        let list = vec![1.5, 3.0, 5.0, 8.8];
        assert_eq!(median(list), Some(4.0));
    }

    #[test]
    fn test_median_uneven_list() {
        let list = vec![1.0, 4.0, 5.0];
        assert_eq!(median(list), Some(4.0));
    }
}
