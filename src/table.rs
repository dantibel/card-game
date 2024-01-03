use rand::Rng;
use rand::seq::SliceRandom;

use crate::cards;
use crate::utils::Error;

pub struct Table
{
    attack_cards    : Vec<cards::Card>,
    defense_cards    : Vec<cards::Card>,
    discarded_cards : Vec<cards::Card>,
    card_stock      : Vec<cards::Card>,
    trump           : cards::Suit,
}

impl Table
{
    pub fn new(card_deck: cards::Deck) -> Self
    {
        let card_deck = card_deck as usize;
        Self
        {
            attack_cards    : Vec::with_capacity(6),
            defense_cards    : Vec::with_capacity(6),
            discarded_cards : Vec::with_capacity(card_deck),
            card_stock      : Vec::with_capacity(card_deck),
            trump           : cards::Suit::Spade,
        }
    }

    pub fn reset(&mut self)
    {
        self.discarded_cards.clear();
        self.attack_cards.clear();
        self.defense_cards.clear();
        self.card_stock.clear();

        for i in 2..=14
        {
            let card_value = cards::Value::from_usize(i);
            self.card_stock.push(cards::Card::new(card_value, cards::Suit::Club));
            self.card_stock.push(cards::Card::new(card_value, cards::Suit::Spade));
            self.card_stock.push(cards::Card::new(card_value, cards::Suit::Heart));
            self.card_stock.push(cards::Card::new(card_value, cards::Suit::Diamond));
        }
        let mut rng = rand::thread_rng();
        self.card_stock.shuffle(&mut rng);

        self.trump = self.card_stock[0].suit();

    }

    // --- getters ---

    pub fn attack_cards(& self) -> & Vec<cards::Card>
    {
        & self.attack_cards
    }

    pub fn defense_cards(& self) -> & Vec<cards::Card>
    {
        & self.defense_cards
    }

    // --- consume player cards ---

    pub fn check_attack_card(& self, attack_card: & cards::Card) -> Result<(), Error>
    {
        if self.attack_cards.len() >= cards::CARDS_IN_DECK_COUNT
        {
            return Err(Error::TooManyAttackCards);
        }

        for played_card in self.defense_cards.iter().chain(self.attack_cards.iter())
        {
            if attack_card.value() == played_card.value()
            {
                return Ok(());
            }
        }
        Err(Error::AbsentCardValue(attack_card.value()))
    }

    pub fn take_attack_card(&mut self, attack_card: cards::Card)
    {
        self.attack_cards.push(attack_card);
    }

    pub fn is_attack_beaten(& self) -> bool
    {
        self.attack_cards.len() == self.defense_cards.len()
    }

    pub fn can_beat(& self, defense_card: & cards::Card, attack_card: & cards::Card) -> bool
    {
        if defense_card.suit() != self.trump
        {
            defense_card.suit() == attack_card.suit() &&
            defense_card.value() > attack_card.value()
        }
        else if attack_card.suit() != self.trump
        {
            true
        }
        else
        {
            defense_card.value() > attack_card.value()  
        }
    }

    pub fn check_defense_card(&mut self, defense_card: & cards::Card, attack_card_index: usize) -> Result<(), Error>
    {
        if self.is_attack_beaten()
        {
            Err(Error::NoCardsToBeat)
        }
        else if attack_card_index >= self.attack_cards.len()
        {
            Err(Error::InvalidAttackIndex(attack_card_index))
        }
        else if !self.can_beat(& defense_card, & self.attack_cards[attack_card_index])
        {
            Err(Error::IncorrectDefense)
        }
        else
        {
            Ok(())
        }
    }

    pub fn take_defense_card(&mut self, defense_card: cards::Card, attack_card_index: usize)
    {
        self.defense_cards.insert(attack_card_index, defense_card);
    }

    // --- transfer cards ---

    pub fn discard_cards(&mut self)
    {
        self.discarded_cards.append(&mut self.attack_cards);
        self.discarded_cards.append(&mut self.defense_cards);
    }

    fn draw_cards_from_back(source: &mut Vec<cards::Card>, count: usize) -> Option<impl Iterator<Item = cards::Card> + '_>
    {
        if source.is_empty()
        {
            return None;
        }
        Some(source.drain((std::cmp::max(0, source.len() - count)) .. source.len()))
    }

    pub fn draw_stock_cards(&mut self, count: usize) -> Option<impl Iterator<Item = cards::Card> + '_>
    {
        Self::draw_cards_from_back(&mut self.card_stock, count)
    }

    pub fn draw_played_cards(&mut self) -> Option<impl Iterator<Item = cards::Card> + '_>
    {
        let count = self.attack_cards.len();
        let attack_cards = Self::draw_cards_from_back(&mut self.attack_cards, count);
        let count = self.defense_cards.len();
        let defense_cards = Self::draw_cards_from_back(&mut self.defense_cards, count);
        
        if attack_cards.is_some() && defense_cards.is_some()
        {
            unsafe
            {
                let a = attack_cards.unwrap_unchecked().chain(defense_cards.unwrap_unchecked().into_iter());
                Some(a)
            }
        }
        else
        {
            None
        }
    }
}