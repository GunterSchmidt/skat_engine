use std::fmt;
use std::fmt::Display;

use crate::cardholder::CardHolder;
use crate::deck::Deck;

/// The Skat-game a player announced.
#[derive(Debug, Default)]
pub enum GameType {
    Grand,
    Clubs,
    Spades,
    Hearts,
    Diamonds,
    Null,
    Ramsch,
    #[default]
    None,
}
impl GameType {
    pub(crate) fn from_suit(suit: crate::card::Suit) -> GameType {
        match suit {
            crate::card::Suit::Clubs => GameType::Clubs,
            crate::card::Suit::Spades => GameType::Spades,
            crate::card::Suit::Hearts => GameType::Hearts,
            crate::card::Suit::Diamonds => GameType::Diamonds,
        }
    }
}

impl Display for GameType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameType::Grand => write!(f, "Grand"),
            GameType::Clubs => write!(f, "Clubs"),
            GameType::Spades => write!(f, "Spades"),
            GameType::Hearts => write!(f, "Hearts"),
            GameType::Diamonds => write!(f, "Diamonds"),
            GameType::Null => write!(f, "Null"),
            GameType::Ramsch => write!(f, "Ramsch"),
            GameType::None => write!(f, "None"),
        }
    }
}

/// This is the structure for one game with 3 players and the Skat.
/// A new game creates a game and deals the cards to the players. \
// TODO Further functionality like reizen needs to be extended.
#[derive(Debug)]
pub struct Game {
    player: [CardHolder; 3],
    skat: CardHolder,
    pub vorhand: usize,
    // pub mittelhand: usize,
    // pub hinterhand: usize,
    pub player_playing: Option<usize>,
}

impl Game {
    /// Creates a new game. \
    /// A card deck is created, shuffled and the cards are dealt to the players.
    pub fn new(vorhand: usize) -> Self {
        let mut deck = Deck::new(true);
        let (player1, player2, player3, skat) = deck.deal();
        // let (mittelhand, hinterhand) = match vorhand {
        //     0 => (1, 2),
        //     1 => (2, 0),
        //     2 => (0, 1),
        //     _ => panic!("Invalid player number"),
        // };
        Self {
            player: [player1, player2, player3],
            skat,
            vorhand,
            // mittelhand,
            // hinterhand,
            player_playing: Option::None,
        }
    }

    // pub fn new_defined(
    //     player1: CardHolder,
    //     player2: CardHolder,
    //     player3: CardHolder,
    //     skat: CardHolder,
    // ) -> Self {
    //     Self {
    //         player1,
    //         player2,
    //         player3,
    //         skat,
    //     }
    // }

    /// Return the player who did the highest reizen
    // TODO actual steps and reizen value
    pub fn reizen(&mut self) -> Option<usize> {
        let (mittelhand, hinterhand) = match self.vorhand {
            0 => (1, 2),
            1 => (2, 0),
            2 => (0, 1),
            _ => panic!("Invalid player number"),
        };

        let mut player = self.vorhand;
        if self.player[mittelhand].reizen_max() > self.player[self.vorhand].reizen_max() {
            player = mittelhand;
        }
        if self.player[hinterhand].reizen_max() > self.player[player].reizen_max() {
            player = hinterhand;
        }
        if self.player[player].reizen_max() == 0 {
            return None;
        }
        self.player[player].reizen_current = self.player[player].reizen_max();
        self.player_playing = Some(player);
        Some(player)
    }

    // identify the player with the most jacks
    pub fn reizen_simple(&self) -> &CardHolder {
        // let jacks_remaining = 4 - self.skat.num_jacks;
        let mut player = &self.player[0];
        if self.player[1].num_jacks() > self.player[0].num_jacks() {
            player = &self.player[1];
        }
        if self.player[2].num_jacks() > player.num_jacks() {
            player = &self.player[2];
        }

        player
    }

    // return true if any player has 4 jacks (without skat, with skat)
    pub fn play_with_4_jacks(&self) -> (bool, bool) {
        let without_skat = self.player.iter().any(|p| p.num_jacks() == 4);

        if without_skat || self.skat.num_jacks() == 0 {
            return (without_skat, false);
        }

        let with_skat = self
            .player
            .iter()
            .any(|p| self.skat.num_jacks() + p.num_jacks() == 4);
        (without_skat, with_skat)
    }

    /// Sorts the cards for each card holder for better display.
    pub fn sort_cards(&mut self) {
        for i in 0..3 {
            self.player[i].sort_cards();
        }
        self.skat.sort_cards();
    }

    pub fn player(&self) -> &[CardHolder; 3] {
        &self.player
    }

    pub fn player_id(&self, id: usize) -> &CardHolder {
        &self.player[id]
    }

    // TODO maybe move reizen
    pub fn player_id_as_mut(&mut self, id: usize) -> &mut CardHolder {
        &mut self.player[id]
    }

    pub fn skat(&self) -> &CardHolder {
        &self.skat
    }
}
