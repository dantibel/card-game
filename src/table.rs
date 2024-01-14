use itertools::chain;

use rand::Rng;
use rand::seq::SliceRandom;

use crate::cards;
use crate::utils::*;

pub struct Table
{
    attack_cards    : Vec<cards::Card>,
    defense_cards   : Vec<cards::Card>,
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
            defense_cards   : Vec::with_capacity(6),
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

    pub fn remain_cards_count(& self) -> usize
    {
        self.card_stock.len()
    }

    pub fn trump(& self) -> cards::Suit
    {
        self.trump
    }

    // --- consume player cards ---

    pub fn is_attack_finished(& self) -> bool
    {
        self.attack_cards.len() >= cards::CARDS_IN_DECK_COUNT
    }

    pub fn check_attack_card(& self, attack_card: & cards::Card, is_first_attack: bool) -> Result<(), Error>
    {
        if self.is_attack_finished()
        {
            return Err(Error::TooManyAttackCards);
        }

        if is_first_attack
        {
            return Ok(());
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

    pub fn can_beat(& self, defense_card: & cards::Card, attack_card_index: usize) -> bool
    {
        let attack_card = & self.attack_cards[attack_card_index];
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

    pub fn check_defense_card(& self, defense_card: & cards::Card, attack_card_index: usize) -> Result<(), Error>
    {
        if self.is_attack_beaten()
        {
            Err(Error::NoCardsToBeat)
        }
        else if attack_card_index >= self.attack_cards.len()
        {
            Err(Error::InvalidAttackIndex(attack_card_index))
        }
        else if !self.can_beat(& defense_card, attack_card_index)
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

    pub fn draw_stock_cards(&mut self, count: usize) -> Option<impl Iterator<Item = cards::Card> + '_>
    {
        if count == 0 || self.card_stock.is_empty()
        {
            return None;
        }
        let range = positive_sub_or_zero(self.card_stock.len(), count)..;
        assert!(range.start <= self.card_stock.len(), "usize substraction underflow (count: {count})");
        Some(self.card_stock.drain(range))
    }

    pub fn draw_played_cards(&mut self) -> impl Iterator<Item = cards::Card> + '_
    {
        if self.attack_cards.len() == 0
        {
            panic!("There isn't any attack card to draw");
        }
        let attack_cards = self.attack_cards.drain(..);
        let defense_cards = self.defense_cards.drain(..);
        attack_cards.chain(defense_cards).into_iter()
    }
}

impl std::fmt::Display for Table 
{
    fn fmt(& self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error>
    {
        writeln!(f, "Cards remain: {}, trump: {}", self.remain_cards_count(), self.trump())?;
        for _ in 0 .. self.attack_cards.len()
        {
            write!(f, " ┌────┐ ")?;
        }
        writeln!(f);

        for card in self.attack_cards.iter()
        {
            write!(f, " │{card} │ ")?;
        }
        writeln!(f);
              for _ in 0 .. self.defense_cards.len()
        {
            write!(f, " │┌───┴┐")?;
        }
 
        for _ in self.defense_cards.len() .. self.attack_cards.len()
        {
            write!(f, " │    │ ")?;
        }
        writeln!(f);
         
        for card in self.defense_cards.iter()
        {
            write!(f, " └┤{card} │")?;
        }
        
        for _ in self.defense_cards.len() .. self.attack_cards.len()
        {
            write!(f, " └────┘ ")?;
        }
        writeln!(f); 

 
        for _ in 0 .. self.defense_cards.len()
        {
            write!(f, "  │    │")?;
        }
        writeln!(f);
        
        for _ in 0 .. self.defense_cards.len()
        {
            write!(f, "  └────┘")?;
        }
        writeln!(f);

        for i in 0..cards::CARDS_IN_DECK_COUNT
        {
            write!(f, "   {:>2}   ", i);
        }
        writeln!(f, "\n")
    }
}