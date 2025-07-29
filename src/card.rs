//! This crate contains the playing [Card], its [Suit] and its [Rank].

#![allow(dead_code)]
use std::fmt::{self, Display};

// pub const MAX_BIDDING_VALUE: usize = 96;

// pub enum BiddingValue {
//     B18 = 18,
//     B20 = 20,
//     B22 = 22,
//     B23 = 23,
//     B24 = 24,
//     B27 = 27,
//     B30 = 30,
//     B33 = 33,
//     B35 = 35,
//     B36 = 36,
//     B40 = 40,
//     B44 = 44,
//     B45 = 45,
//     B46 = 46,
//     B48 = 48,
//     B50 = 50,
//     B54 = 54,
//     B55 = 55,
//     B59 = 59,
//     B60 = 60,
//     B72 = 72,
//     B96 = 96,
// }

/// This enum represents the four colors of the game Skat and their properties.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Suit {
    Clubs = 0,
    Spades = 1,
    Hearts = 2,
    Diamonds = 3,
}

impl Suit {
    pub fn suit_reiz_factor(&self) -> i16 {
        match self {
            Suit::Clubs => 12,
            Suit::Spades => 11,
            Suit::Hearts => 10,
            Suit::Diamonds => 9,
        }
    }

    pub fn suit_name(&self) -> String {
        match self {
            Suit::Clubs => "Clubs".to_string(),
            Suit::Spades => "Spades".to_string(),
            Suit::Hearts => "Hearts".to_string(),
            Suit::Diamonds => "Diamonds".to_string(),
        }
    }

    pub fn from_usize(value: usize) -> Suit {
        match value {
            0 => Suit::Clubs,
            1 => Suit::Spades,
            2 => Suit::Hearts,
            3 => Suit::Diamonds,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

impl TryFrom<char> for Suit {
    type Error = String;

    fn try_from(suit_char: char) -> Result<Self, String> {
        match suit_char {
            'K' => Ok(Self::Clubs),
            'P' => Ok(Self::Spades),
            'H' => Ok(Self::Hearts),
            'C' => Ok(Self::Diamonds),
            _ => Err(
                "Unknown symbol: use K = Club, P = Spades, H = Hearts, C = Diamonds".to_string(),
            ),
        }
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Self::Clubs => 'K',  // 'C',
            Self::Spades => 'P', // 'S',
            Self::Hearts => 'H',
            Self::Diamonds => 'C', // 'D',
                                   // Suit::Clubs => '\u{2663}', //'K', // 'C',
                                   // Suit::Spades => '\u{2660}',   // 'P', 'S',
                                   // Suit::Hearts => '\u{2665}',   // 'H',
                                   // Suit::Diamonds => '\u{2666}', //'C',      // 'D',
        };
        write!(f, "{c}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rank {
    Seven,
    Eight,
    Nine,
    Jack,
    Queen,
    King,
    Ten,
    Ace,
}

impl Rank {
    /// Points when counting the results of the game.
    pub fn points(&self) -> u16 {
        match self {
            Self::Seven => 0,
            Self::Eight => 0,
            Self::Nine => 0,
            Self::Jack => 2,
            Self::Queen => 3,
            Self::King => 4,
            Self::Ten => 10,
            Self::Ace => 11,
        }
    }
}

impl TryFrom<char> for Rank {
    type Error = String;

    fn try_from(rank_char: char) -> Result<Self, String> {
        match rank_char {
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            'B' => Ok(Self::Jack),
            'J' => Ok(Self::Jack),
            'D' => Ok(Self::Queen),
            'Q' => Ok(Self::Queen),
            'K' => Ok(Self::King),
            'Z' => Ok(Self::Ten),
            'T' => Ok(Self::Ten),
            'A' => Ok(Self::Ace),
            _ => Err(
                "Unknown symbol: use 7,8,9, Z/T = ten, B/J = Jack, D/Q = Queen, K = King, A = Ace"
                    .to_string(),
            ),
        }
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Self::Seven => '7',
            Self::Eight => '8',
            Self::Nine => '9',
            Self::Jack => 'B',  // 'J',
            Self::Queen => 'D', // 'Q',
            Self::King => 'K',
            Self::Ten => 'Z', // 'T',
            Self::Ace => 'A',
        };
        write!(f, "{c}")
    }
}

/// Single card of the game. \
/// Shuffles and deals the cards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Card {
    suit: Suit,
    rank: Rank,
    points: u16,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Card {
        let points = rank.points();
        Card { suit, rank, points }
    }

    /// High Cards: Jack, Ace, Ten, King  \
    /// Low Cards:  Queen, Nine, Eight, Seven
    pub fn is_high_card(&self) -> bool {
        match self.rank {
            Rank::Seven => false,
            Rank::Eight => false,
            Rank::Nine => false,
            Rank::Jack => true,
            Rank::Queen => false,
            Rank::King => true,
            Rank::Ten => true,
            Rank::Ace => true,
        }
    }

    pub fn suit(&self) -> Suit {
        self.suit
    }

    /// Returns a value between 0 and 3 (0 = Clubs, 1 = Spades, 2 = Hearts, 3 = Diamonds)
    pub fn suit_value(&self) -> usize {
        self.suit as usize
    }

    pub fn rank(&self) -> Rank {
        self.rank
    }

    pub fn points(&self) -> usize {
        self.points as usize
    }
}

/// Allows input like "KB"/"CJ" for Jack of Club
impl TryFrom<&str> for Card {
    type Error = String;

    fn try_from(card_named: &str) -> Result<Self, String> {
        if card_named.len() != 2 {
            return Err("Need two symbols".to_string());
        }
        let suit = Suit::try_from(card_named.as_bytes()[0] as char)?;
        let rank = Rank::try_from(card_named.as_bytes()[1] as char)?;

        Ok(Card::new(suit, rank))
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.suit, self.rank)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn try_from() {
        let card = Card::new(Suit::Clubs, Rank::Jack);
        let card_test = Card::try_from("KB").unwrap();
        assert_eq!(card, card_test);
    }
}
