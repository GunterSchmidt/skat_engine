use rand::seq::SliceRandom;

use crate::{
    card::{Card, Rank, Suit},
    cardholder::CardHolder,
};

/// The deck contains the 32 cards of the game. \
/// It is used to create all cards and deal them to the players. \
/// After dealing the cards the deck is gone as the cards are moved to the players.
#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    /// Creates a deck with 32 sorted/shuffled playing cards.
    pub fn new(shuffled: bool) -> Deck {
        let mut cards = Vec::new();
        for suit in &[Suit::Clubs, Suit::Spades, Suit::Hearts, Suit::Diamonds] {
            for rank in &[
                Rank::Seven,
                Rank::Eight,
                Rank::Nine,
                Rank::Jack,
                Rank::Queen,
                Rank::King,
                Rank::Ten,
                Rank::Ace,
            ] {
                cards.push(Card::new(*suit, *rank));
            }
        }
        let mut deck = Deck { cards };
        if shuffled {
            deck.shuffle();
        }

        deck
    }

    /// Returns the cards of this deck
    pub fn cards(&self) -> &[Card] {
        &self.cards
    }

    /// Deals the cards to the players and the Skat. After this the deck is gone as the cards are moved to the [CardHolder]s.
    pub fn deal(&mut self) -> (CardHolder, CardHolder, CardHolder, CardHolder) {
        let mut cards_player1 = Vec::new();
        let mut cards_player2 = Vec::new();
        let mut cards_player3 = Vec::new();
        let mut cards_skat = Vec::new();

        for _ in 0..3 {
            cards_player1.push(self.cards.pop().unwrap());
            cards_player2.push(self.cards.pop().unwrap());
            cards_player3.push(self.cards.pop().unwrap());
        }
        for _ in 0..2 {
            cards_skat.push(self.cards.pop().unwrap());
        }
        for _ in 0..4 {
            cards_player1.push(self.cards.pop().unwrap());
            cards_player2.push(self.cards.pop().unwrap());
            cards_player3.push(self.cards.pop().unwrap());
        }
        for _ in 0..3 {
            cards_player1.push(self.cards.pop().unwrap());
            cards_player2.push(self.cards.pop().unwrap());
            cards_player3.push(self.cards.pop().unwrap());
        }

        let mut player1 = CardHolder::new("Player 1", cards_player1);
        player1.sort_cards();
        let player2 = CardHolder::new("Player 2", cards_player2);
        let player3 = CardHolder::new("Player 3", cards_player3);
        let skat = CardHolder::new("Skat", cards_skat);

        (player1, player2, player3, skat)
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::rng();
        // use rand::seq::SliceRandom;
        self.cards.shuffle(&mut rng);
    }
}
