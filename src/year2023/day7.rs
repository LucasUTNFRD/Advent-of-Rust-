use std::{cmp::Ordering, collections::HashMap, fmt::Debug};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard(Cards), // Lowest
    OnePair(Cards),
    TwoPair(Cards),
    ThreeOfAKind(Cards),
    FullHouse(Cards),
    FourOfAKind(Cards),
    FiveOfAKind(Cards), // Highest
}

const JOKER: u8 = b'J';

impl HandType {
    pub fn new_with_jokers(cards: Cards) -> Self {
        let mut card_counter: HashMap<u8, usize> = HashMap::new();
        for cards in cards.inner {
            *card_counter.entry(cards).or_insert(0) += 1;
        }

        let joker_count = card_counter.remove(&JOKER).unwrap_or(0);

        if joker_count == 5 {
            // Case: JJJJJ (FiveOfAKind)
            return HandType::FiveOfAKind(cards);
        }

        let max_non_joker_card_entry = card_counter.iter_mut().max_by_key(|(_, count)| **count);

        match max_non_joker_card_entry {
            Some((_card, count)) => {
                *count += joker_count;
            }
            None => {
                unreachable!("card_counter should not be empty if joker_count < 5");
            }
        }

        let mut counts: Vec<usize> = card_counter.values().copied().collect();
        counts.sort_unstable_by(|a, b| b.cmp(a));

        match counts.as_slice() {
            [5] => HandType::FiveOfAKind(cards),
            [4, 1] => HandType::FourOfAKind(cards),
            [3, 2] => HandType::FullHouse(cards),
            [3, 1, 1] => HandType::ThreeOfAKind(cards),
            [2, 2, 1] => HandType::TwoPair(cards),
            [2, 1, 1, 1] => HandType::OnePair(cards),
            [1, 1, 1, 1, 1] => HandType::HighCard(cards),
            _ => unreachable!(),
        }
    }
}

impl From<Cards> for HandType {
    fn from(cards: Cards) -> Self {
        let mut card_counter: HashMap<u8, usize> = HashMap::new();
        for cards in cards.inner {
            *card_counter.entry(cards).or_insert(0) += 1;
        }

        let mut counts: Vec<usize> = card_counter.values().copied().collect();
        counts.sort_unstable_by(|a, b| b.cmp(a));

        match counts.as_slice() {
            [5] => HandType::FiveOfAKind(cards),
            [4, 1] => HandType::FourOfAKind(cards),
            [3, 2] => HandType::FullHouse(cards),
            [3, 1, 1] => HandType::ThreeOfAKind(cards),
            [2, 2, 1] => HandType::TwoPair(cards),
            [2, 1, 1, 1] => HandType::OnePair(cards),
            [1, 1, 1, 1, 1] => HandType::HighCard(cards),
            _ => unreachable!(),
        }
    }
}

// 'A' is the highest and '2' is the lowest
const CARDS: [u8; 13] = [
    b'A', b'K', b'Q', b'J', b'T', b'9', b'8', b'7', b'6', b'5', b'4', b'3', b'2',
];

const CARDS_WITH_JOKERS: [u8; 13] = [
    b'A', b'K', b'Q', b'T', b'9', b'8', b'7', b'6', b'5', b'4', b'3', b'2', b'J',
];

#[derive(Clone, Debug, Copy)]
struct Hand {
    pub hand: HandType,
    bid: u64,
}

impl Hand {
    pub fn new(cards: Cards, bid: u64) -> Self {
        Self {
            hand: HandType::from(cards),
            bid,
        }
    }

    pub fn new_with_jokers(cards: Cards, bid: u64) -> Self {
        Self {
            hand: HandType::new_with_jokers(cards),
            bid,
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct Cards {
    inner: [u8; 5],
    jokers: bool,
}

impl Debug for Cards {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = std::str::from_utf8(&self.inner).unwrap();
        write!(f, "{}", s)
    }
}

impl Ord for Cards {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let card_set = if self.jokers {
            CARDS_WITH_JOKERS
        } else {
            CARDS
        };

        for (card_a, card_b) in self.inner.iter().zip(other.inner.iter()) {
            let rank_a = get_index(*card_a, card_set);
            let rank_b = get_index(*card_b, card_set);
            let comparison = rank_b.cmp(&rank_a);
            if comparison != Ordering::Equal {
                return comparison;
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for Cards {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_index(card: u8, card_set: [u8; 13]) -> usize {
    for (idx, c) in card_set.iter().enumerate() {
        if card == *c {
            return idx;
        }
    }

    unreachable!()
}

pub fn part_1(input: &str) -> u64 {
    let mut hands: Vec<_> = input
        .lines()
        .map(|l| {
            let (hand, bid) = l.split_once(" ").unwrap();
            let bid: u64 = bid.parse().unwrap();
            let hand = Cards {
                inner: hand.as_bytes().try_into().unwrap(),
                jokers: false,
            };
            Hand::new(hand, bid)
        })
        .collect();

    hands.sort_unstable_by(|x, y| x.hand.cmp(&y.hand));

    hands
        .iter()
        .enumerate()
        .map(|(rank, h)| h.bid * (rank as u64 + 1))
        .sum()
}

pub fn part_2(input: &str) -> u64 {
    let mut hands: Vec<_> = input
        .lines()
        .map(|l| {
            let (hand, bid) = l.split_once(" ").unwrap();
            let bid: u64 = bid.parse().unwrap();
            let hand = Cards {
                inner: hand.as_bytes().try_into().unwrap(),
                jokers: true,
            };

            Hand::new_with_jokers(hand, bid)
        })
        .collect();

    hands.sort_unstable_by(|x, y| x.hand.cmp(&y.hand));

    hands
        .iter()
        .enumerate()
        .map(|(rank, h)| h.bid * (rank as u64 + 1))
        .sum()
}

#[cfg(test)]
mod test {
    use crate::year2023::day7;

    const SAMPLE_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_sample_input_parsing() {
        assert_eq!(6440, day7::part_1(SAMPLE_INPUT));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(5905, day7::part_2(SAMPLE_INPUT));
    }
}
