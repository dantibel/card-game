use rand::Rng;

use crate::table::Table;
use crate::utils::{Error, log, logln};
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

pub struct Game
{
    table                        : Table,
    players                      : Vec<Box<dyn Player>>,
    settings                     : Settings,
    winners_count                : usize,
    first_attacking_player_index : usize 
}

impl Game
{
    pub fn new(settings: Settings) -> Self
    {
        Self
        {
            table                        : Table::new(settings.card_deck),
            players                      : vec![],
            settings                     : settings,
            winners_count                : 0,
            first_attacking_player_index : 0,
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

    pub fn start(&mut self)
    {
        if self.players_count() < 2
        {
            logln!(0, "There are not enough players in this game to start (need {} more)\n", (2 - self.players_count()));
            return;
        }

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
        //self.attacking_player_index = rand::thread_rng().gen_range(0..self.players_count());
        self.first_attacking_player_index = 1; 

        logln!(0, "Game have started! ══════════════════════\n");

        // play until end
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
    fn process_player_attack(&mut self, player_index: usize, is_first_attack: bool) -> bool
    {
        if self.table.is_attack_finished()
        {
            return false;
        }

        let player = self.players[player_index].as_mut();
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
    fn process_player_defense(&mut self, player_index: usize) -> bool
    {
        let player = self.players[player_index].as_mut();
        if player.cards_count() == 0
        {
            return false;
        }
        
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
        let defending_player_index = (self.first_attacking_player_index + 1) % self.players_count();

        // attacking player starts the attack
        assert_eq!(self.process_player_attack(self.first_attacking_player_index, true), true, "First attack error");

        let mut is_defense_succeed = true;
        if !self.process_player_defense(defending_player_index)
        {
            is_defense_succeed = false;
        }
        else
        {

            // attacking player continues the attack
            /*
            let mut is_defense = true;
            while self.table.attack_cards().len() < cards::CARDS_IN_DECK_COUNT
            {
                logln!(0, "{}", (self.table));
                
                if is_defense
                {
                    if !self.process_player_defense(defendeing_player_index)
                    {
                        logln!(0, "{}", (self.table));
                        
                        is_defense_succeed = false;
                        break;
                }
            } 
            else 
            {
                if !self.process_player_attack(self.first_attacking_player_index, false)
                {
                    logln!(0, "{}", (self.table));
                    
                    break;
                }
            }
            is_defense = !is_defense;
            */
            
            let mut passes_count = 0usize;
            let mut attacking_player_index = self.first_attacking_player_index;
            while !self.table.is_attack_finished()
            {
                if attacking_player_index == defending_player_index
                {
                    attacking_player_index = (attacking_player_index + 1) % self.players_count();
                    continue;
                }

                logln!(0, "{}", (self.table));
                
                if self.process_player_attack(attacking_player_index, false)
                {
                    logln!(0, "{}", (self.table));

                    if !self.process_player_defense(defending_player_index)
                    {
                        is_defense_succeed = self.players[defending_player_index].cards_count() == 0;
                        break;
                    }
                }
                else
                {
                    passes_count += 1;
                    if attacking_player_index == self.first_attacking_player_index
                        && passes_count >= self.players_count() - 1
                    {
                        break;
                    }
                    
                    attacking_player_index = (attacking_player_index + 1) % self.players_count();
                }
            }
        }
            
        /*
        if self.players_count() > 2
        {
            // other players continue the round
            let mut round_is_going = true;
            let mut player_index = defendeing_player_index;
            while round_is_going
            {
                round_is_going = false;
                player_index = (player_index + 1) % self.players_count();
                
                if player_index != defendeing_player_index
                {
                    if self.table.attack_cards().len() < cards::CARDS_IN_DECK_COUNT
                        && self.process_player_attack(player_index, false)
                    {
                        round_is_going = true;
                    }
                }
                else
                {
                    if !self.process_player_defense(player_index)
                    {
                        is_defense_succeed = false;
                    }
                }
            }
        }
        */

        if is_defense_succeed
        {
            logln!(0, "{} beat attack\n", (self.players[defending_player_index].name()));
        }
        else 
        {
            logln!(0, "{} didn't beat attack\n", (self.players[defending_player_index].name()));
        }

        // attacking player draw cards
        let mut player = self.players[self.first_attacking_player_index].as_mut();
        player.take_cards(&mut self.table.draw_stock_cards((cards::CARDS_IN_DECK_COUNT_SINGNED - player.cards_count() as isize).max(0) as usize));

        logln!(0, "{}", (self.table));
        
        if self.players_count() > 2
        {
            // other players draw cards
            let mut next_index: usize;
            let mut player: &mut dyn Player;
            for i in 1 ..= self.players_count()
            {
                next_index = (defending_player_index + i) % self.players_count();
                player = self.players[next_index].as_mut();
                player.take_cards(&mut self.table.draw_stock_cards(cards::CARDS_IN_DECK_COUNT - player.cards_count()))
            }
        }

        // defending player draws cards
        let defending_player = self.players[defending_player_index].as_mut();
        if is_defense_succeed
        {
            self.table.discard_cards();
            defending_player.take_cards(&mut self.table.draw_stock_cards(cards::CARDS_IN_DECK_COUNT - defending_player.cards_count()));
        }
        else 
        {
            defending_player.take_cards(&mut self.table.draw_played_cards());
        }

        // choose next player
        self.first_attacking_player_index = defending_player_index;
    }

}