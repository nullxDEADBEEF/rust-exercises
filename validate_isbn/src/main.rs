use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum ParseIsbnError {
    TooLong,
    TooShort,
    FailedChecksum,
}

#[derive(Debug, PartialEq)]
struct Isbn {
    raw: String,
    digits: Vec<u8>,
}

impl FromStr for Isbn {
    type Err = ParseIsbnError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let isbn = Self {
            raw: String::from(s),
            digits: s
                .chars()
                .into_iter()
                .filter(|character| character.is_digit(10))
                .map(|character| character.to_digit(10).unwrap() as u8)
                .collect(),
        };

        match isbn.digits.len().cmp(&13) {
            Ordering::Greater => return Err(Self::Err::TooLong),
            Ordering::Less => return Err(Self::Err::TooShort),
            _ => {}
        }

        if isbn.digits[12] != calculate_check_digit(&isbn.digits) {
            return Err(Self::Err::FailedChecksum);
        }

        Ok(isbn)
    }
}

impl std::fmt::Display for Isbn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

// https://en.wikipedia.org/wiki/International_Standard_Book_Number#ISBN-13_check_digit_calculation
fn calculate_check_digit(digits: &[u8]) -> u8 {
    let weights: Vec<u8> = [1, 3].repeat(6);
    // apply weights and sum them up
    let weighted_digits_sum = digits
        .iter()
        .zip(weights)
        .map(|(number, weight)| number * weight)
        .sum::<u8>();
    // get remainder from weighted digits
    let check_digit = 10 - (weighted_digits_sum % 10);

    match check_digit {
        10 => 0,
        _ => check_digit,
    }
}

fn main() {
    let command_line_rust: Isbn = "9781098109431".parse().unwrap();
    println!(
        "Command-line Rust's ISBN-13 ({}) is valid!",
        command_line_rust
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_correctly_calculate_check_digits() {
        let cases = [
            ([9, 7, 8, 1, 8, 6, 1, 9, 7, 8, 7, 6], 9),
            ([9, 7, 8, 3, 1, 6, 1, 4, 8, 4, 1, 0], 0),
        ];

        for (case, check) in cases.iter() {
            let actual = calculate_check_digit(case);
            println!("{:?} -> {}?  {}", &case, check, actual);
            assert_eq!(calculate_check_digit(case), *check);
        }
    }

    #[test]
    fn command_line_rust() {
        "978-1098109431".parse::<Isbn>().unwrap();
    }

    #[test]
    fn rust_in_action() {
        "978-3-16-148410-0".parse::<Isbn>().unwrap();
    }

    #[test]
    fn too_long_isbn_length() {
        assert_eq!(
            Err(ParseIsbnError::TooLong),
            "99978-1098109431".parse::<Isbn>()
        );
    }
}
