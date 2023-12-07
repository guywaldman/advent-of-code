use std::{collections::HashMap, error::Error};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Card(u8);

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self.0 {
            14 => 'A',
            13 => 'K',
            12 => 'Q',
            11 => 'J',
            10 => 'T',
            _ => self.0.to_string().chars().next().unwrap(),
        };
        write!(f, "{}", value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

impl Card {
    pub fn new(card: char) -> Self {
        let value = match card {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _ => card.to_digit(10).unwrap() as u8,
        };
        Self(value)
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
    bid: usize,
}

impl Hand {
    pub fn new(cards: Vec<Card>, bid: usize) -> Self {
        // Calculate hand type.
        let mut cards_map: HashMap<Card, usize> = HashMap::new();
        for card in &cards {
            cards_map.insert(*card, cards_map.get(card).unwrap_or(&0) + 1);
        }

        let mut cards_map = cards_map.iter().collect::<Vec<_>>();
        cards_map.sort_by_key(|(_, &count)| count);
        let cards_map_values = cards_map
            .iter()
            .map(|(_, &count)| count)
            .collect::<Vec<_>>();

        let hand_type = match &cards_map_values[..] {
            [1, 1, 1, 1, 1] => HandType::HighCard,
            [1, 1, 1, 2] => HandType::OnePair,
            [1, 2, 2] => HandType::TwoPair,
            [1, 1, 3] => HandType::ThreeOfAKind,
            [1, 4] => HandType::FourOfAKind,
            [2, 3] => HandType::FullHouse,
            [5] => HandType::FiveOfAKind,
            _ => panic!(),
        };

        Self {
            cards,
            hand_type,
            bid,
        }
    }

    pub fn tie_breaker(&self, other: &Self) -> std::cmp::Ordering {
        for (card, other_card) in self.cards.iter().zip(other.cards.iter()) {
            match card.cmp(other_card) {
                std::cmp::Ordering::Equal => continue,
                ordering => return ordering,
            }
        }
        std::cmp::Ordering::Equal
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            std::cmp::Ordering::Equal => self.tie_breaker(other),
            ordering => ordering,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.tie_breaker(other) == std::cmp::Ordering::Equal
    }
}

impl Eq for Hand {}

fn parse_line(line: &str) -> Result<Hand, Box<dyn Error>> {
    let mut split = line.split_whitespace();
    let (hand, bid) = (split.next().unwrap(), split.next().unwrap());
    let cards = &hand.chars().map(Card::new).collect::<Vec<_>>();
    let bid = bid.parse::<usize>()?;
    Ok(Hand::new(cards.to_vec(), bid))
}

pub fn solve_part_1(input: &str) -> Result<String, Box<dyn Error>> {
    let mut hands = input
        .lines()
        .map(|line| parse_line(line).unwrap())
        .collect::<Vec<_>>();
    hands.sort();
    dbg!(&hands);
    let result: usize = hands
        .iter()
        .enumerate()
        .map(|(index, hand)| hand.bid * (index + 1))
        .sum();
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_parsing_test() {
        let hand = parse_line("32T3K 765").unwrap();
        assert_eq!(hand.hand_type, HandType::OnePair);
        assert_eq!(hand.bid, 765);

        let hand = parse_line("T55J5 684").unwrap();
        assert_eq!(hand.hand_type, HandType::ThreeOfAKind);
        assert_eq!(hand.bid, 684);

        let hand = parse_line("JJJ77 42").unwrap();
        assert_eq!(hand.hand_type, HandType::FullHouse);
        assert_eq!(hand.bid, 42);
    }
}
