use rand::Rng;

use crate::table::Table;
use crate::utils::*;
use crate::{cards, player};
use crate::player::Player;


pub struct SettingsBuilder
{
    pub card_deck              : cards::Deck,
    pub cheats_allowed         : bool,
    pub finish_after_first_win : bool,
}

impl SettingsBuilder
{
    pub fn new() -> Self
    {
        Self
        {
            card_deck: cards::Deck::Standart,
            cheats_allowed: false,
            finish_after_first_win: true,
        }
    }
    
    pub fn card_deck(mut self, card_deck: cards::Deck) -> Self
    {
        self.card_deck = card_deck;
        self
    }
    
    pub fn cheats_allowed(mut self, cheats_allowed: bool) -> Self
    {
        self.cheats_allowed = cheats_allowed;
        self
    }
    
    pub fn finish_after_first_win(mut self, finish_after_first_win: bool) -> Self
    {
        self.finish_after_first_win = finish_after_first_win;
        self
    }
    
    pub fn build(& self) -> Settings
    {
        Settings
        {
            card_deck: self.card_deck,
            max_players_count: self.card_deck as usize / cards::CARDS_IN_DECK_COUNT,
            cheats_allowed: self.cheats_allowed,
            finish_after_first_win: self.finish_after_first_win,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Settings
{
    card_deck              : cards::Deck,
    max_players_count      : usize,
    cheats_allowed         : bool,
    finish_after_first_win : bool,
}

#[derive(Default)]
struct RoundInfo
{
    is_defense_succeed: bool,
    first_attacking_index: usize,
    attacking_index: usize,
    last_not_passed_index: usize,
    defending_index: usize,
    passes_count: usize,
}

impl RoundInfo
{
}

pub struct Game
{
    table                        : Table,
    players                      : Vec<Box<dyn Player>>,
    settings                     : Settings,
    winners_count                : usize,
    //first_attacking_player_index : usize,
    round_info: RoundInfo, 
}

impl Game
{
    const MIN_PLAYERS_COUNT: usize = 2;
    
    pub fn new(settings: Settings) -> Self
    {
        Self
        {
            table                        : Table::new(settings.card_deck),
            players                      : vec![],
            settings                     : settings,
            winners_count                : 0,
            //first_attacking_player_index : 0,
            round_info: Default::default(), 
        }
    }

    pub fn max_players_count(& self) -> usize
    {
        self.settings.max_players_count 
    }

    pub fn players_count(& self) -> usize
    {
        self.players.len()
    }

    pub fn add_player(&mut self, player: Box<dyn Player>) -> Result<(), Error>
    {
        if self.players_count() == self.max_players_count()
        {
            Err(Error::TooManyPlayers(self.max_players_count()))
        }
        else
        {
            logln!(0, "{} joined the game!\n", (player.name()));
            self.players.push(player);
            Ok(())
        }
    }

    /// # For test only!
    pub fn table(&mut self) -> &mut Table
    {
        &mut self.table
    }

    pub fn prepare(&mut self)
    {
        logln!(0, "Current settings: {} cards, {}, {}\n",
            (self.settings.card_deck as usize),
            (if self.settings.cheats_allowed {"cheats are allowed"} else {"cheats are forbiden"}),
            (if self.settings.finish_after_first_win {"playnig until first win"} else {"playing until one player remain"}));
        logln!(0, "Shufling deck...\n");
        self.table.reset();
        
        logln!(0, "Serving cards...\n");
        for player in self.players.iter_mut()
        {
            player.take_cards(&mut self.table.draw_stock_cards(cards::CARDS_IN_DECK_COUNT).unwrap());
            player.show_cards();
        }
    
        logln!(0, "Choosing starting player...\n");
        self.round_info.first_attacking_index = rand::thread_rng().gen_range(0..self.players_count());
    }

    pub fn start(&mut self)
    {
        if self.players_count() < Self::MIN_PLAYERS_COUNT
        {
            logln!(0, "There are not enough players in this game to start (need {} more)\n", (Self::MIN_PLAYERS_COUNT - self.players_count()));
            return;
        }

        self.prepare();
        
        logln!(0, "Game have started! ══════════════════════\n");
        if self.settings.finish_after_first_win
        {
            while self.winners_count == 0
            {
                self.play_round();
            }
        }
        else
        {
            while self.winners_count < self.players_count() - 1
            {
                self.play_round();
            }
        }
    }

    /// Returns whether player played a card
    fn process_player_attack(&mut self, is_first_attack: bool) -> bool
    {
        debug_assert!(!self.table.is_attack_finished());

        let player = self.players[self.round_info.attacking_index].as_mut();
        if player.cards_count() == 0
        {
            return false;
        }
        
        match player.play_attack_card(& self.table, is_first_attack)
        {
            Some(card) =>
            {
                if is_first_attack
                {
                    logln!(0, "{} started attack with the {card}\n", (player.name()));
                    self.table.take_attack_card(card);
                    return true;
                }
                match self.table.check_attack_card(& card, is_first_attack)
                {
                    Ok(_) =>
                    {
                        logln!(0, "{} continue attack with the {card}\n", (player.name()));
                        self.table.take_attack_card(card);

                        if !player.has_cards()
                        {
                            self.winners_count += 1;
                            logln!(0, "{} won! ({} winners in total)\n", (player.name()), (self.winners_count));       
                        }
                        return true;
                    },
                    Err(error) => panic!("{error}"),
                }
            },
            None =>
            {
                logln!(0, "{} passed\n", (player.name()));
                false
            }
        }
    } 

    /// Returns whether player played a card
    fn process_player_defense(&mut self) -> bool
    {
        let player = self.players[self.round_info.defending_index].as_mut();
        debug_assert!(player.has_cards());
        
        match player.play_defense_card(& self.table)
        {
            Some((attack_card_index, defense_card)) =>
            {
                logln!(0, "{} beat the {} with the {}\n", (player.name()), (self.table.attack_cards()[attack_card_index]), (defense_card));
                match self.table.check_defense_card(& defense_card, attack_card_index)
                {
                    Ok(_) => 
                    {
                        self.table.take_defense_card(defense_card, attack_card_index);
                        true
                    },
                    Err(error) => panic!("{error}"),
                }
            },
            None => 
            {
                logln!(0, "{} is taking the cards\n", (player.name()));
                false
            }
        }
    } 

    fn play_round(&mut self)
    {
        logln!(0, "New round started! ──────────────────────\n");
        logln!(0, "{}", (self.table));
        self.round_info.defending_index = (self.round_info.first_attacking_index + 1) % self.players_count();
        self.round_info.is_defense_succeed = true;

        // attacking player starts the attack
        debug_assert!(self.process_player_attack(true), "First attack error");

        if !self.process_player_defense()
        {
            self.round_info.is_defense_succeed = false;
        }
        else
        {
            debug_assert!(self.players[self.round_info.defending_index].has_cards());

            while !self.table.is_attack_finished()
            {
                if self.round_info.attacking_index == self.round_info.defending_index
                {
                    self.round_info.attacking_index = (self.round_info.attacking_index + 1) % self.players_count();
                    continue;
                }

                logln!(0, "{}", (self.table));
                
                if self.process_player_attack(false)
                {   
                    logln!(0, "{}", (self.table));
                    self.round_info.last_not_passed_index = self.round_info.attacking_index;
                    self.round_info.passes_count = 0;

                    if !self.process_player_defense() 
                    {
                        // player hasn't beaten attacking card
                        self.round_info.is_defense_succeed = false;
                        break;
                    }
                    else if !self.players[self.round_info.defending_index].has_cards()
                    {
                        // player has beaten attacking card with his/her last card
                        self.round_info.is_defense_succeed = true;
                        logln!(0, "{} won! ({} winners in total)\n", (self.players[self.round_info.defending_index].name()), (self.winners_count));
                        self.winners_count += 1;
                        break;
                    }
                }
                else
                {
                    self.round_info.passes_count += 1;
                    if self.round_info.attacking_index == self.round_info.last_not_passed_index
                    && self.round_info.passes_count >= self.players_count() - 1
                    {
                        break;
                    }
                    
                    self.round_info.attacking_index = (self.round_info.attacking_index + 1) % self.players_count();
                }
            }
        }
            
        if self.round_info.is_defense_succeed
        {
            logln!(0, "{} beat attack\n", (self.players[self.round_info.defending_index].name()));
        }
        else 
        {
            logln!(0, "{} didn't beat attack\n", (self.players[self.round_info.defending_index].name()));
        }

        logln!(0, "{}", (self.table));
        
        // attacing players draw cards
        let mut next_index: usize;
        let mut player: &mut dyn Player;
        for i in 0 .. self.players_count()
        {
            next_index = (self.round_info.first_attacking_index + i) % self.players_count();
            if next_index == self.round_info.defending_index
            {
                continue;
            }

            player = self.players[next_index].as_mut();
            if let Some(mut cards) =
                self.table.draw_stock_cards(player.missing_cards_count())
            {
                player.take_cards(&mut cards);
            }
        }

        // defending player draws cards
        let defending_player = self.players[self.round_info.defending_index].as_mut();
        if self.round_info.is_defense_succeed
        {
            self.table.discard_cards();
            if let Some(mut cards) = 
                self.table.draw_stock_cards(defending_player.missing_cards_count())
            {
                defending_player.take_cards(&mut cards);
            }

            // choose next player
            self.round_info.first_attacking_index = self.round_info.defending_index;
        }
        else 
        {
            defending_player.take_cards(&mut self.table.draw_played_cards());

            // choose next player
            self.round_info.first_attacking_index = (self.round_info.defending_index + 1) % self.players_count();
        }
    }

}