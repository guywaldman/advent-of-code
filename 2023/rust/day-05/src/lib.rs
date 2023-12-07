use std::{collections::HashMap, error::Error, hash::Hasher};

#[derive(Clone, Copy)]
pub struct Card {
    value: u8,
    joker_mode: bool,
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self.value {
            14 => 'A',
            13 => 'K',
            12 => 'Q',
            11 => 'J',
            10 => 'T',
            _ => self.value.to_string().chars().next().unwrap(),
        };
        write!(f, "{}", value)
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Card { value: 11, joker_mode: true }, Card { value: c, joker_mode: true }) if *c != 11 => std::cmp::Ordering::Less,
            (Card { value: c, joker_mode: true }, Card { value: 11, joker_mode: true }) if *c != 11 => std::cmp::Ordering::Greater,
            (Card { value: c1, joker_mode: _ }, Card { value: c2, joker_mode: _ }) => c1.cmp(c2),
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        let (Card { value: c1, joker_mode: _ }, Card { value: c2, joker_mode: _ }) = (self, other);
        c1 == c2
    }
}

impl Eq for Card {}

impl std::hash::Hash for Card {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl Card {
    pub fn new(card: char, joker_mode: bool) -> Self {
        let value = match card {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _ => card.to_digit(10).unwrap() as u8,
        };
        Self { value, joker_mode }
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

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
    bid: usize,
}

impl Hand {
    pub fn new(cards: Vec<Card>, bid: usize, joker_mode: bool) -> Self {
        let mut cards_map: HashMap<Card, usize> = HashMap::new();

        for card in &cards {
            if joker_mode && card.value == 11 {
                continue;
            }
            cards_map.insert(*card, cards_map.get(card).unwrap_or(&0) + 1);
        }

        let mut cards_map = cards_map.iter().collect::<Vec<_>>();
        cards_map.sort_by_key(|(_, &count)| count);
        cards_map.reverse();
        let cards_map_values = cards_map.iter().map(|(_, &count)| count).collect::<Vec<_>>();

        let mut hand_type = match &cards_map_values[..] {
            [5] | [5, ..] => HandType::FiveOfAKind,
            [4, 1] | [4, 1, ..] => HandType::FourOfAKind,
            [4] | [4, ..] => HandType::FourOfAKind,
            [3, 2] | [3, 2, ..] => HandType::FullHouse,
            [3, 1] | [3, 1, ..] => HandType::ThreeOfAKind,
            [3] | [3, ..] => HandType::ThreeOfAKind,
            [2, 2] | [2, 2, ..] => HandType::TwoPair,
            [2, 1] | [2, 1, ..] => HandType::OnePair,
            [2] | [2, ..] => HandType::OnePair,
            _ => HandType::HighCard,
        };

        if joker_mode {
            let jokers_count = cards.iter().filter(|c| c.value == 11).count();

            hand_type = match (hand_type, jokers_count) {
                (HandType::HighCard, 1) => HandType::OnePair,
                (HandType::HighCard, 2) => HandType::ThreeOfAKind,
                (HandType::HighCard, 3) => HandType::FourOfAKind,
                (HandType::HighCard, c) if c >= 4 => HandType::FiveOfAKind,
                (HandType::OnePair, 1) => HandType::ThreeOfAKind,
                (HandType::OnePair, 2) => HandType::FourOfAKind,
                (HandType::OnePair, 3) => HandType::FiveOfAKind,
                (HandType::TwoPair, 1) => HandType::FullHouse,
                (HandType::ThreeOfAKind, 1) => HandType::FourOfAKind,
                (HandType::ThreeOfAKind, 2) => HandType::FiveOfAKind,
                (HandType::FourOfAKind, 1) => HandType::FiveOfAKind,
                _ => hand_type,
            };
        }

        Self { cards, hand_type, bid }
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

fn parse_line(line: &str, joker_mode: bool) -> Result<Hand, Box<dyn Error>> {
    let mut split = line.split_whitespace();
    let (hand, bid) = (split.next().unwrap(), split.next().unwrap());
    let cards = &hand.chars().map(|c| Card::new(c, joker_mode)).collect::<Vec<_>>();
    let bid = bid.parse::<usize>()?;
    Ok(Hand::new(cards.to_vec(), bid, joker_mode))
}

pub fn solve_part_1(input: &str) -> Result<String, Box<dyn Error>> {
    let mut hands = input.lines().map(|line| parse_line(line, false).unwrap()).collect::<Vec<_>>();
    hands.sort();
    let result: usize = hands.iter().enumerate().map(|(index, hand)| hand.bid * (index + 1)).sum();
    Ok(result.to_string())
}

pub fn solve_part_2(input: &str) -> Result<String, Box<dyn Error>> {
    let mut hands = input.lines().map(|line| parse_line(line, true).unwrap()).collect::<Vec<_>>();
    hands.sort();
    let result: usize = hands.iter().enumerate().map(|(index, hand)| hand.bid * (index + 1)).sum();
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_parsing_nojoker_test() {
        let hand = parse_line("32T3K 765", false).unwrap();
        assert_eq!(hand.hand_type, HandType::OnePair);
        assert_eq!(hand.bid, 765);

        let hand = parse_line("T55J5 684", false).unwrap();
        assert_eq!(hand.hand_type, HandType::ThreeOfAKind);
        assert_eq!(hand.bid, 684);

        let hand = parse_line("JJJ77 42", false).unwrap();
        assert_eq!(hand.hand_type, HandType::FullHouse);
        assert_eq!(hand.bid, 42);
    }

    #[test]
    fn line_parsing_joker_test() {
        assert_eq!(parse_line("32T3J 42", true).unwrap().hand_type, HandType::ThreeOfAKind);
        assert_eq!(parse_line("32TJJ 42", true).unwrap().hand_type, HandType::ThreeOfAKind);
        assert_eq!(parse_line("32JJJ 42", true).unwrap().hand_type, HandType::FourOfAKind);
        assert_eq!(parse_line("33KKJ 42", true).unwrap().hand_type, HandType::FullHouse);
        assert_eq!(parse_line("JJJJJ 42", true).unwrap().hand_type, HandType::FiveOfAKind);
        assert_eq!(parse_line("JJJJ1 42", true).unwrap().hand_type, HandType::FiveOfAKind);
        assert_eq!(parse_line("JJJ11 42", true).unwrap().hand_type, HandType::FiveOfAKind);
        assert_eq!(parse_line("JJ111 42", true).unwrap().hand_type, HandType::FiveOfAKind);
        assert_eq!(parse_line("J1111 42", true).unwrap().hand_type, HandType::FiveOfAKind);
        assert_eq!(parse_line("J1122 42", true).unwrap().hand_type, HandType::FullHouse);
        assert_eq!(parse_line("JJ122 42", true).unwrap().hand_type, HandType::FourOfAKind);
        assert_eq!(parse_line("JJ1JJ 42", true).unwrap().hand_type, HandType::FiveOfAKind);
        assert_eq!(parse_line("3111J 42", true).unwrap().hand_type, HandType::FourOfAKind);
        assert_eq!(parse_line("JJJ12 42", true).unwrap().hand_type, HandType::FourOfAKind);
        assert_eq!(parse_line("12123 42", true).unwrap().hand_type, HandType::TwoPair);
        assert_eq!(parse_line("33322 42", true).unwrap().hand_type, HandType::FullHouse);
        assert_eq!(parse_line("3332J 42", true).unwrap().hand_type, HandType::FourOfAKind);
    }

    #[test]
    fn hand_cmp_joker_test() {
        let hand1 = parse_line("J1122 42", true).unwrap();
        let hand2 = parse_line("2J211 42", true).unwrap();
        assert!(hand2 > hand1);
    }
}
