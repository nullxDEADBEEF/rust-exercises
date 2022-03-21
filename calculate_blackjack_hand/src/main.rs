#[derive(Debug, PartialEq)]
pub enum Card {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Jack,
    Queen,
    King,
}

impl Card {
    fn value(&self) -> u8 {
        match self {
            Card::Ace => 11,
            Card::Two => 2,
            Card::Three => 3,
            Card::Four => 4,
            Card::Five => 5,
            Card::Six => 6,
            Card::Seven => 7,
            Card::Eight => 8,
            Card::Nine => 9,
            Card::Jack | Card::Queen | Card::King => 10,
        }
    }
}

struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn new() -> Self {
        Self { cards: vec![] }
    }

    fn add(&mut self, card: Card) {
        self.cards.push(card);
    }

    fn value(&self) -> usize {
        let mut value = self.cards.iter().map(|card| card.value()).sum::<u8>() as usize;
        if self.cards.contains(&Card::Ace) && value > 21 {
            value -= 10;
        }

        value
    }

    fn is_loosing_hand(&self) -> bool {
        self.value() > 21
    }
}

fn main() {
    let mut hand = Hand::new();
    hand.add(Card::King);
    hand.add(Card::Ace);
    println!("Total value of hand: {}", hand.value());
    println!("Loosing hand: {}", hand.is_loosing_hand());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_hand() {
        let hand = Hand::new();
        assert_eq!(hand.value(), 0);
    }

    #[test]
    fn strong_hand() {
        let mut hand = Hand::new();
        hand.add(Card::Queen);
        hand.add(Card::Ace);
        assert_eq!(hand.value(), 21);
    }

    #[test]
    fn risky_hand() {
        let mut hand = Hand::new();
        hand.add(Card::King);
        hand.add(Card::Queen);
        hand.add(Card::Ace);
        assert_eq!(hand.value(), 21);
    }

    #[test]
    fn oops() {
        let mut hand = Hand::new();
        hand.add(Card::King);
        hand.add(Card::Seven);
        hand.add(Card::Five);

        assert!(hand.is_loosing_hand());
        assert_eq!(hand.value(), 22);
    }
}
