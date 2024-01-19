use std::path::PrefixComponent;
use std::time::Duration;
use rand::Rng;

use crate::utils::*;
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
    
    fn has_cards(& self) -> bool
    {
        self.cards_count() > 0
    }

    fn missing_cards_count(& self) -> usize
    {
        //(cards::CARDS_IN_DECK_COUNT_SINGNED - self.cards_count() as isize).max(0) as usize
        positive_sub_or_zero(cards::CARDS_IN_DECK_COUNT, self.cards_count())
        //cards::CARDS_IN_DECK_COUNT.wrapping_sub(self.cards_count())
    }

    fn take_cards(&mut self, cards: &mut dyn Iterator<Item = cards::Card>)
    {
        log!(0, "{} take cards: ", (self.name()));
        let old_cards_count = self.cards_count();
        self.cards_mut().extend(cards);
        for card_index in old_cards_count .. self.cards_count() 
        {
            log!(0, "{}, ", (self.cards()[card_index]));
        }
        logln!(0, "\n");
        // self.cards_mut().sort_by(|lhs, rhs| -> std::cmp::Ordering lhs.value().cmp(rhs.value())});
        self.cards_mut().sort();
    }

    fn show_cards(& self)
    {
        logln!(0, "{}'s cards:", (self.name()));
        cards::output_cards(& self.cards());
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
        self.show_cards();
        let card_index =
            loop
            {
                match get_input(1, if is_first_attack {"Choose the attack card: "} else {"Choose the attack card (or type 'pass'): "})
                {
                    Input::String(string) => 
                        if !is_first_attack && string == "pass"
                        {
                            return None;
                        }
                        else
                        {
                            logln!(2, "Inrecognized string answer");
                        },
                    Input::Number(index) =>
                        if index < self.cards_count()
                        {
                            break index;
                        }
                        else
                        {
                            logln!(2, "You have only {} cards", (self.cards_count()));
                        },
                }
            };
    
        logln!();
        Some(self.cards.remove(card_index))
    }
    
    fn play_defense_card(&mut self, table: & table::Table) -> Option<(usize, cards::Card)>
    {
        self.show_cards();
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
                            logln!(2, "Inrecognized string answer");
                        },
                    Input::Number(index) =>
                        if index < self.cards_count()
                        {
                            break index;
                        }
                        else
                        {
                            logln!(2, "You have only {} cards", (self.cards_count()));
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
                        logln!(2, "Inrecognized string answer");
                    },
                Input::Number(index) =>
                    if index < table.attack_cards().len()
                    {
                        let defense_card = & self.cards[defense_card_index];
                        let attack_card = & table.attack_cards()[index];
                        if table.can_beat(defense_card, index)
                        {
                            return Some((index, self.cards.remove(defense_card_index)));
                        }
                        else
                        {
                            logln!(2, "You can't beat {defense_card} with {attack_card}");    
                        }
                    }
                    else
                    {
                        logln!(2, "There are only {} attack cards on the table", (table.attack_cards().len()));
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
        write!(f, "{}", 
            match self
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
        std::thread::sleep(Duration::from_millis(500));
        
        let lowest_cards_indecies: [usize; 3] = [0; 3]; // trump isn't taken
        for i in 0 .. self.cards_count()
        {
            match table.check_attack_card(& self.cards[i], is_first_attack)
            {
                Ok(()) => return Some(self.cards.remove(i)),
                Err(Error::AbsentCardValue(_)) => continue,
                Err(error) => panic!("Bot attack error: {error}"),
            }
        };
        None
    }
    
    fn play_defense_card(&mut self, table: & table::Table) -> Option<(usize, cards::Card)>
    {
        std::thread::sleep(Duration::from_millis(500));

        let mut non_trump_index: Option<usize> = None;
        let mut trump_index: Option<usize> = None;
        
        let attack_card_index = table.defense_cards().len();
        for defense_card_index in (0 .. self.cards_count()).rev()
        {
            match table.check_defense_card(& self.cards[defense_card_index], attack_card_index)
            {
                Ok(()) =>
                    if self.cards[defense_card_index].suit() == table.trump()
                    {
                        trump_index = Some(defense_card_index);
                    }
                    else
                    {
                        non_trump_index = Some(defense_card_index);
                    },
                Err(Error::IncorrectDefense) => continue,
                Err(error) => panic!("{} defense error: {error}", self.name)
            }
        }
        
        match non_trump_index   
        {
            Some(index) => Some((attack_card_index, self.cards.remove(index))),
            None =>
                match trump_index
                {
                    Some(index) => Some((attack_card_index, self.cards.remove(index))),
                    None => None
                }
        }
    }
}