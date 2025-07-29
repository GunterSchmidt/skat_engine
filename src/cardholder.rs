use std::fmt::{self, Display};

use crate::{
    card::{Card, Rank, Suit},
    game::GameType,
};

/// A CardHolder is either a Player or the Skat. It can hold any number of cards.
#[derive(Debug)]
pub struct CardHolder {
    name: String,
    cards: Vec<Card>,
    // Vorhand = 0, Mittelhand = 1, Hinterhand = 2, Skat = any
    //  player_in_game: u16,
    /// The number of jacks contained in the cards.
    num_jacks: u8,
    /// Array with set 1 for Club, Spade, Heart, Diamond Jack.
    // TODO check if required, used only once for jack_factor
    has_jacks: [bool; 4],

    pub reizen_current: u16,
    reizen_max: u16,
    game_type: GameType,
}

impl CardHolder {
    pub fn new(name: &str, cards: Vec<Card>) -> CardHolder {
        let mut has_jacks: [bool; 4] = [false; 4];
        // let num_jacks = cards.iter().filter(|card| card.rank() == Rank::Jack).count();
        let mut num_jacks = 0;
        for card in cards.iter().filter(|card| card.rank() == Rank::Jack) {
            num_jacks += 1;
            has_jacks[card.suit_value()] = true;
        }
        CardHolder {
            name: name.to_string(),
            cards,
            num_jacks,
            has_jacks,
            reizen_current: 0,
            reizen_max: u16::MAX,
            game_type: GameType::None,
        }
    }

    /// This merges a players hand with the skat, so in total 12 cards.
    // TODO possibly just add the skat
    pub fn new_with_skat(player: &CardHolder, skat: &CardHolder) -> CardHolder {
        let mut cards = player.cards.clone();
        cards.extend(skat.cards.clone());
        CardHolder::new(&player.name, cards)
    }

    pub fn cards(&self) -> &[Card] {
        &self.cards
    }

    pub fn cards_total_points(&self) -> u16 {
        self.cards.iter().map(|card| card.rank().points()).sum()
    }

    pub fn game_type(&self) -> &GameType {
        &self.game_type
    }

    /// Checks if the CardHolder has a specific card in his hand.
    pub fn holds_card(&self, suit: Suit, rank: Rank) -> bool {
        for card in self.cards.iter() {
            if card.rank() == rank && card.suit() == suit {
                return true;
            }
        }
        false
    }

    // number of cards of that rank
    pub fn num_cards_rank(&self, rank: Rank) -> usize {
        self.cards
            .iter()
            .filter(|&card| card.rank() == rank)
            .count()
    }

    // number of cards of that suit without jacks
    pub fn num_cards_suit(&self, suit: Suit) -> usize {
        self.cards
            .iter()
            .filter(|&card| card.rank() != Rank::Jack && card.suit() == suit)
            .count()
    }

    // number of cards of all suits without jacks
    pub fn num_cards_all_suits(&self) -> (usize, usize, usize, usize) {
        let (mut clubs, mut spades, mut hearts, mut diamonds) = (0, 0, 0, 0);
        for card in self.cards.iter().filter(|card| card.rank() != Rank::Jack) {
            match card.suit() {
                Suit::Clubs => clubs += 1,
                Suit::Spades => spades += 1,
                Suit::Hearts => hearts += 1,
                Suit::Diamonds => diamonds += 1,
            }
        }
        (clubs, spades, hearts, diamonds)
    }

    pub fn num_jacks(&self) -> usize {
        self.num_jacks as usize
    }

    pub fn reizen_max(&mut self) -> u16 {
        if self.reizen_max == u16::MAX {
            _ = self.reizen_v1();
        }
        self.reizen_max
    }

    // Reizwert 0 = passe
    pub fn reizen_v1(&mut self) -> u16 {
        // Regeln:
        // Mindestens 5 Trümpfe auf der Hand und 1 Ass in einer Beifarbe oder mindestens 6 Trümpfe
        // Trumpfkarten mindestens 10 Punkte
        // Alle Karten mindestens 35 Punkte
        self.reizen_max = 0;
        // self.game_type = GameType::Null;

        let mut suit_card_count = [0; 4];
        let mut suit_card_points = [0; 4];
        for card in self.cards.iter().filter(|&card| card.rank() != Rank::Jack) {
            let suit_id = card.suit() as usize;
            suit_card_count[suit_id] += 1;
            suit_card_points[suit_id] += card.points();
        }
        let max_count = *suit_card_count.iter().max().unwrap();
        // Check if at least 5 trump
        let trump_count = self.num_jacks + max_count;
        if trump_count < 5 {
            return 0;
        }

        let mut aces_count = 0;
        let mut aces = [0; 4];
        for card in self.cards.iter().filter(|&card| card.rank() == Rank::Ace) {
            aces_count += 1;
            aces[card.suit() as usize] = 1;
        }
        if trump_count == 5 && aces_count == 0 {
            return 0;
        }

        let mut max_suit = 0;
        // let mut max_points = 0;
        for i in 0..4 {
            // only check larger, if it is the same, then then current value is the higher suit
            if suit_card_count[i] == max_count
                && (trump_count > 5 || suit_card_points[i] >= 10 || self.num_jacks > 2)
            {
                if max_suit == 0 {
                    max_suit = i;
                    // max_points = suit_card_points[i];
                } else {
                    // choose suit without aces if other suit has an ace
                    if aces[i] < aces[max_suit] {
                        max_suit = i;
                    }
                    // TODO same ace count, then choose what? better points for non-trump color?
                }
            }
        }
        if max_suit == 0 {
            return 0;
        }
        if self.num_jacks + max_count == 5 && aces_count - aces[max_suit] == 0 {
            return 0;
        }

        // Calc Reizwert
        let jack_factor = self.reiz_factor();
        let factor = jack_factor.abs() + 1;
        let suit = Suit::from_usize(max_suit);
        let suit_reiz_factor = suit.suit_reiz_factor();

        self.game_type = GameType::from_suit(suit);
        self.reizen_max = (suit_reiz_factor * factor) as u16;
        self.reizen_max
    }

    /// Calc Factor with or without jacks.
    /// # Returns
    /// A positive number if the first jack is held, else a negative number,\
    /// e.g. 3rd ('without two')= -2.
    fn reiz_factor(&self) -> i16 {
        match self.num_jacks {
            0 => -4,
            4 => 4,
            _ => {
                if self.has_jacks[0] {
                    if self.has_jacks[1] {
                        if self.has_jacks[2] {
                            3
                        } else {
                            2
                        }
                    } else {
                        1
                    }
                } else if !self.has_jacks[1] {
                    if !self.has_jacks[2] {
                        -3
                    } else {
                        -2
                    }
                } else {
                    -1
                }
            }
        }
    }

    pub fn sort_cards(&mut self) {
        // TODO move rank_order to Rank, rank_order_color, rank_order_null
        self.cards.sort_by(|a, b| {
            let rank_order = |rank: &Rank| match rank {
                Rank::Jack => 9,
                Rank::Ace => 8,
                Rank::Ten => 7,
                Rank::King => 6,
                Rank::Queen => 5,
                Rank::Nine => 3,
                Rank::Eight => 2,
                Rank::Seven => 1,
            };

            // TODO move suit_order to Suit or remove as it has a value already
            let suit_order = |suit: &Suit| match suit {
                Suit::Clubs => 4,
                Suit::Spades => 3,
                Suit::Hearts => 2,
                Suit::Diamonds => 1,
            };

            if a.rank() == Rank::Jack && b.rank() == Rank::Jack {
                suit_order(&b.suit()).cmp(&suit_order(&a.suit()))
            } else if a.rank() == Rank::Jack {
                std::cmp::Ordering::Less
            } else if b.rank() == Rank::Jack {
                std::cmp::Ordering::Greater
            } else if a.suit() == b.suit() {
                rank_order(&b.rank()).cmp(&rank_order(&a.rank()))
            } else {
                suit_order(&b.suit()).cmp(&suit_order(&a.suit()))
            }
        });
    }

    // pub fn sort_for_color(cards: &mut [Card]) {
    //     // TODO move rank_order to Rank, rank_order_color, rank_order_null
    //     cards.sort_by(|a, b| {
    //         let rank_order = |rank: &Rank| match rank {
    //             Rank::Jack => 9,
    //             Rank::Ace => 8,
    //             Rank::Ten => 7,
    //             Rank::King => 6,
    //             Rank::Queen => 5,
    //             Rank::Nine => 3,
    //             Rank::Eight => 2,
    //             Rank::Seven => 1,
    //         };
    //
    //         // TODO move suit_order to Suit or remove as it has a value already
    //         let suit_order = |suit: &Suit| match suit {
    //             Suit::Clubs => 4,
    //             Suit::Spades => 3,
    //             Suit::Hearts => 2,
    //             Suit::Diamonds => 1,
    //         };
    //
    //         if a.rank == Rank::Jack && b.rank == Rank::Jack {
    //             suit_order(&b.suit).cmp(&suit_order(&a.suit))
    //         } else if a.rank == Rank::Jack {
    //             std::cmp::Ordering::Less
    //         } else if b.rank == Rank::Jack {
    //             std::cmp::Ordering::Greater
    //         } else if a.suit == b.suit {
    //             rank_order(&b.rank).cmp(&rank_order(&a.rank))
    //         } else {
    //             suit_order(&b.suit).cmp(&suit_order(&a.suit))
    //         }
    //     });
    // }

    /// Finds the trump cards and their points (jacks count as card and 2). \
    /// Trump cards are all jacks and the suit with the most number of cards. \
    /// If multiple suits have the same card count, then the one with the highest card points. \
    /// If still the same, the rank.
    /// # Returns
    /// (suit (color), the card count, the total points of these cards)
    pub fn trump_suit_cards(&self) -> (Suit, usize, usize) {
        let mut suit_card_count = [0; 4];
        let mut suit_card_points = [0; 4];
        // let mut jacks_count = 0;
        for card in self.cards.iter().filter(|&card| card.rank() != Rank::Jack) {
            let suit_id = card.suit() as usize;
            suit_card_count[suit_id] += 1;
            suit_card_points[suit_id] += card.points();
        }
        let max_count = *suit_card_count.iter().max().unwrap();
        let mut max_suit = 0;
        let mut max_points = 0;
        for i in 0..4 {
            // only check larger, if it is the same, then then current value is the higher suit
            if suit_card_count[i] == max_count && suit_card_points[i] > max_points {
                max_suit = i;
                max_points = suit_card_points[i];
            }
        }

        let suit = Suit::from_usize(max_suit);
        // (suit, max_count + jacks_count, max_value + jacks_count * 2)
        (suit, max_count, max_points)
    }

    pub fn cards_to_string(&self) -> String {
        let cards_str: Vec<String> = self.cards.iter().map(|c| c.to_string()).collect();
        cards_str.join(", ")
    }
}

/// Allows input like "KB"/"CJ" for Jack of Club
impl TryFrom<(&str, &Vec<&str>)> for CardHolder {
    type Error = String;

    fn try_from(tuple: (&str, &Vec<&str>)) -> Result<Self, String> {
        let mut cards = Vec::new();
        for &card_name in tuple.1 {
            let card = Card::try_from(card_name)?;
            cards.push(card);
        }

        let card_holder = Self::new(tuple.0, cards);
        Ok(card_holder)
    }
}

impl Display for CardHolder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: [{}]", self.name, self.cards_to_string())
    }
}
