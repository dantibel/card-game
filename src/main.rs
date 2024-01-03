#![allow(unused)]

mod utils;
mod cards;
mod player;
mod table;
mod game;

use crate::cards::{Card, Deck, Value, Suit, CARDS_IN_DECK_COUNT, output_cards};
use crate::utils::{Error, get_input, Input, log};
use crate::player::{BotDificulty, Bot, RealPlayer, Player};
use crate::game::{Game, SettingsBuilder};
use crate::table::Table;

#[cfg(test)]
mod tests {

    use crate::cards::{Card, Deck, Value, Suit, CARDS_IN_DECK_COUNT, output_cards};
    use crate::player::{BotDificulty, Bot, RealPlayer, Player};
    use crate::game::{Game, SettingsBuilder};
    use crate::table::Table;
 
    fn add_players_to_game(card_deck: Deck)
    {
        let mut game = Game::new(SettingsBuilder::new().card_deck(card_deck).build());
    
        for i in 1..=(card_deck as usize / CARDS_IN_DECK_COUNT)
        {
            let bot = Box::new(Bot::new(BotDificulty::Easy));
            assert!(game.add_player(bot).is_ok());
              
        }
        let bot = Box::new(Bot::new(BotDificulty::Hard));
        assert!(game.add_player(bot).is_err());
        println!("game has {} players", game.players_count());
    }
        
    #[test]
    fn add_players()
    {
        add_players_to_game(Deck::Reduced);
        add_players_to_game(Deck::Standart);
        add_players_to_game(Deck::Full);
        add_players_to_game(Deck::Extended);
    }

    #[test]
    fn choose_cards()
    {
        /*
        let mut player = RealPlayer::new("foo");
        let mut cards = vec![Card::new(Value::Seven, Suit::Heart), Card::new(Value::Seven, Suit::Spade), Card::new(Value::Seven, Suit::Diamond), Card::new(Value::Seven, Suit::Club)];
        player.take_cards(&mut cards.into_iter());
        let mut table = Table::new(Deck::Standart);
        
        assert_eq!(player.play_attack_card(& table), None); // pass
        assert_eq!(player.play_attack_card(& table), Some(Card::new(Value::Seven, Suit::Heart))); // 0
        assert_eq!(player.play_attack_card(& table), Some(Card::new(Value::Seven, Suit::Club))); // 2
        */
    }
}

fn main() {
    let mut game = game::Game::new(game::SettingsBuilder::new().build());

    let bot = Box::new(player::Bot::new(player::BotDificulty::Hard));
    let _ = game.add_player(bot);

    let player = Box::new(player::RealPlayer::new("FOO"));
    let _ = game.add_player(player);

    game.start();
    // let mut cards = vec![Card::new(Value::Seven, Suit::Heart), Card::new(Value::Seven, Suit::Spade), Card::new(Value::Seven, Suit::Diamond), Card::new(Value::Seven, Suit::Club)];
    // output_cards(& cards);
}
