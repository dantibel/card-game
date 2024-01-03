use std::path::PrefixComponent;

use crate::utils::{Error, get_input, Input, log};
use crate::cards;
use crate::table;

pub trait Player
{
    // --- getters ---

    fn cards(& self) -> & Vec<cards::Card>;
    fn cards_mut(&mut self) -> &mut Vec<cards::Card>;
    fn name(& self) -> & str;

    // --- cards operations ---

    fn cards_count(& self) -> usize
    {
        self.cards().len()
    }

    fn take_cards(&mut self, cards: &mut dyn Iterator<Item = cards::Card>)
    {
        self.cards_mut().extend(cards);
    }

    fn show_cards(& self)
    {
        println!("{}'s cards:", self.name());
        cards::output_cards(& self.cards());
    }

    fn get_card(&mut self, card_index: usize) -> Result<& cards::Card, Error>
    {
        if card_index >= self.cards().len()
        {
            Err(Error::InvalidDeckIndex(card_index))
        }
        else
        {
            unsafe
            {
                Ok(self.cards_mut().get_unchecked(card_index))
            }
        }
    }

    // --- playing operations ---

    fn play_attack_card(&mut self, table: & table::Table, is_first_attack: bool) -> Option<cards::Card>;
    fn play_defense_card(&mut self, table: & table::Table) -> Option<(usize, cards::Card)>;
}

pub struct RealPlayer
 {
    cards : Vec<cards::Card>,   
    name  : String,
}

impl RealPlayer
{
    pub fn new(name: &str) -> Self
    {
        Self {cards: Vec::with_capacity(cards::CARDS_IN_DECK_COUNT), name: name.to_string()} 
    }
}

impl Player for RealPlayer
{
    fn name(& self) -> &str
    {
        & self.name
    }

    fn cards(& self) -> & Vec<cards::Card>
    {
        & self.cards
    }

    fn cards_mut(&mut self) -> &mut Vec<cards::Card>
    {
        &mut self.cards
    }

    fn cards_count(& self) -> usize
    {
        self.cards.len()
    }

    fn play_attack_card(&mut self, table: & table::Table, is_first_attack: bool) -> Option<cards::Card>
    {
        let card_index =
            loop
            {
                match get_input(1, if is_first_attack {"Choose the attack card"} else {"Choose the attack card (or type 'pass'):"})
                {
                    Input::String(string) => 
                        if !is_first_attack && string == "pass"
                        {
                            return None;
                        }
                        else
                        {
                            log!(2, "Inrecognized string answer");
                        },
                    Input::Number(number) =>
                        if number < self.cards().len()
                        {
                            break number;
                        }
                        else
                        {
                            log!(2, "You have only {} cards", (self.cards().len()));
                        },
                }
            };
    
        Some(self.cards.remove(card_index))
    }
    
    fn play_defense_card(&mut self, table: & table::Table) -> Option<(usize, cards::Card)>
    {
        let defense_card_index =
            loop
            {
                match get_input(1, "Choose the defense card (or type 'take'): ")
                {
                    Input::String(string) => 
                        if string == "take"
                        {
                            return None;
                        }
                        else
                        {
                            log!(2, "Inrecognized string answer");
                        },
                    Input::Number(number) =>
                        if number < self.cards().len()
                        {
                            break number;
                        }
                        else
                        {
                            log!(2, "You have only {} cards", (self.cards().len()));
                        },
                }
            };

        loop
            {
                match get_input(1, "Choose card to beat (or type 'take'): ")
                {
                    Input::String(string) => 
                        if string == "take"
                        {
                            return None;
                        }
                        else
                        {
                            log!(2, "Inrecognized string answer");
                        },
                    Input::Number(number) =>
                        if number < table.attack_cards().len()
                        {
                            let defense_card = & self.cards[defense_card_index];
                            let attack_card = & table.attack_cards()[number];
                            if table.can_beat(defense_card, attack_card)
                            {
                                return Some((number, self.cards.remove(defense_card_index)));
                            }
                            else
                            {
                                log!(2, "You can't beat {defense_card} with {attack_card}");    
                            }
                        }
                        else
                        {
                            log!(2, "There are only {} attack cards on the table", (table.attack_cards().len()));
                        },
                }
            };
    }
}

#[derive(PartialEq)]
pub enum BotDificulty
{
    Easy,
    Medium,
    Hard,
}

impl std::fmt::Display for BotDificulty
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error>
    {
        write!(f, "{}", match self
            {
                Self::Easy => "Easy",
                Self::Medium => "Medium",
                Self::Hard => "Hard",
            })
    }
}

pub struct Bot
 {
    cards          : Vec<cards::Card>,   
    name           : String,
    bot_difficulty : BotDificulty,
}

impl Bot
{
    pub fn new(difficulty: BotDificulty) -> Self
    {
        static mut BOT_COUNT: u8 = 0;
        unsafe 
        {
            BOT_COUNT += 1;
            Self {cards: vec![], name: format!("Bot #{BOT_COUNT} ({difficulty})"), bot_difficulty: difficulty} 
        }
    }
}

impl Player for Bot
{
    fn name(& self) -> &str
    {
        & self.name
    }

    fn cards(& self) -> & Vec<cards::Card>
    {
        & self.cards
    }

    fn cards_mut(&mut self) -> &mut Vec<cards::Card>
    {
        &mut self.cards
    }

    fn cards_count(& self) -> usize
    {
        self.cards.len()
    }

    fn play_attack_card(&mut self, table: & table::Table, is_first_attack: bool) -> Option<cards::Card>
    {
        None
    }
    
    fn play_defense_card(&mut self, table: & table::Table) -> Option<(usize, cards::Card)>
    {
        None
    }
}