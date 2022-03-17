trait MorseCode {
    fn to_morse_code(&self) -> Message;
}

impl std::fmt::Display for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pulse::Long => write!(f, "_"),
            Pulse::Short => write!(f, "."),
        }
    }
}

type Message = Vec<Letter>;
type Letter = Vec<Pulse>;

#[derive(Debug, PartialEq)]
enum Pulse {
    Short,
    Long,
}

impl MorseCode for String {
    fn to_morse_code(&self) -> Message {
        self.chars()
            .into_iter()
            .map(|character| match character {
                'A' | 'a' => vec![Pulse::Short, Pulse::Long],
                'B' | 'b' => vec![Pulse::Long, Pulse::Short, Pulse::Short, Pulse::Short],
                'C' | 'c' => vec![Pulse::Long, Pulse::Short, Pulse::Long, Pulse::Short],
                'D' | 'd' => vec![Pulse::Long, Pulse::Short, Pulse::Short],
                'E' | 'e' => vec![Pulse::Short],
                'F' | 'f' => vec![Pulse::Short, Pulse::Short, Pulse::Long, Pulse::Short],
                'G' | 'g' => vec![Pulse::Long, Pulse::Long, Pulse::Short],
                'H' | 'h' => vec![Pulse::Short, Pulse::Short, Pulse::Short, Pulse::Short],
                'I' | 'i' => vec![Pulse::Short, Pulse::Short],
                'J' | 'j' => vec![Pulse::Short, Pulse::Long, Pulse::Long, Pulse::Long],
                'K' | 'k' => vec![Pulse::Long, Pulse::Short, Pulse::Long],
                'L' | 'l' => vec![Pulse::Short, Pulse::Long, Pulse::Short, Pulse::Short],
                'M' | 'm' => vec![Pulse::Long, Pulse::Long],
                'N' | 'n' => vec![Pulse::Long, Pulse::Short],
                'O' | 'o' => vec![Pulse::Long, Pulse::Long, Pulse::Long],
                'P' | 'p' => vec![Pulse::Short, Pulse::Long, Pulse::Long, Pulse::Short],
                'Q' | 'q' => vec![Pulse::Long, Pulse::Long, Pulse::Short, Pulse::Long],
                'R' | 'r' => vec![Pulse::Short, Pulse::Long, Pulse::Short],
                'S' | 's' => vec![Pulse::Short, Pulse::Short, Pulse::Short],
                'T' | 't' => vec![Pulse::Long],
                'U' | 'u' => vec![Pulse::Short, Pulse::Short, Pulse::Long],
                'V' | 'v' => vec![Pulse::Short, Pulse::Short, Pulse::Short, Pulse::Long],
                'W' | 'w' => vec![Pulse::Short, Pulse::Long, Pulse::Long],
                'X' | 'x' => vec![Pulse::Long, Pulse::Short, Pulse::Short, Pulse::Long],
                'Y' | 'y' => vec![Pulse::Long, Pulse::Short, Pulse::Long, Pulse::Long],
                'Z' | 'z' => vec![Pulse::Long, Pulse::Long, Pulse::Short, Pulse::Short],
                '1' => vec![
                    Pulse::Short,
                    Pulse::Long,
                    Pulse::Long,
                    Pulse::Long,
                    Pulse::Long,
                ],
                '2' => vec![
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Long,
                    Pulse::Long,
                    Pulse::Long,
                ],
                '3' => vec![
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Long,
                    Pulse::Long,
                ],
                '4' => vec![
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Long,
                ],
                '5' => vec![
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Short,
                ],
                '6' => vec![
                    Pulse::Long,
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Short,
                ],
                '7' => vec![
                    Pulse::Long,
                    Pulse::Long,
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Short,
                ],
                '8' => vec![
                    Pulse::Long,
                    Pulse::Long,
                    Pulse::Long,
                    Pulse::Short,
                    Pulse::Short,
                ],
                '9' => vec![
                    Pulse::Long,
                    Pulse::Long,
                    Pulse::Long,
                    Pulse::Long,
                    Pulse::Short,
                ],
                '0' => vec![
                    Pulse::Long,
                    Pulse::Long,
                    Pulse::Long,
                    Pulse::Long,
                    Pulse::Long,
                ],
                _ => vec![],
            })
            .collect()
    }
}

fn main() {
    for sentence in "Hello World".to_string().to_morse_code() {
        for character in sentence {
            print!("{character}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Message, MorseCode, Pulse};

    #[test]
    fn test_morse_code_for_a() {
        let expected: Message = vec![vec![Pulse::Short, Pulse::Long]];
        let actual = "A".to_string().to_morse_code();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_morse_code_for_hello_world() {
        let expected: Message = vec![
            vec![Pulse::Short, Pulse::Short, Pulse::Short, Pulse::Short],
            vec![Pulse::Short],
            vec![Pulse::Short, Pulse::Long, Pulse::Short, Pulse::Short],
            vec![Pulse::Short, Pulse::Long, Pulse::Short, Pulse::Short],
            vec![Pulse::Long, Pulse::Long, Pulse::Long],
            vec![],
            vec![Pulse::Short, Pulse::Long, Pulse::Long],
            vec![Pulse::Long, Pulse::Long, Pulse::Long],
            vec![Pulse::Short, Pulse::Long, Pulse::Short],
            vec![Pulse::Short, Pulse::Long, Pulse::Short, Pulse::Short],
            vec![Pulse::Long, Pulse::Short, Pulse::Short],
        ];
        let actual = "Hello World".to_string().to_morse_code();
        assert_eq!(expected, actual);
    }
}
