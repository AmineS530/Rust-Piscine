use std::time::{ SystemTime, UNIX_EPOCH };

fn random_gen(max: u8) -> u8 {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();

    let value = (now % (max as u128)) + 1;
    value as u8
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rank {
    Number(u8),
    Ace,
    King,
    Queen,
    Jack,
}

#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Suit {
    pub fn random() -> Suit {
        Suit::translate(random_gen(4))
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            4 => Suit::Club,
            _ => panic!("Invalid value for Suit: {}", value),
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        Rank::translate(random_gen(13))
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            2..=10 => Rank::Number(value),
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => panic!("Invalid value for Rank: {}", value),
        }
    }
}

pub fn winner_card(card: &Card) -> bool {
    card.suit == Suit::Spade && card.rank == Rank::Ace
}
