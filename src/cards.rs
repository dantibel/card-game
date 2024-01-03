pub const CARDS_IN_DECK_COUNT: usize = 6;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Value
{
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
    Joker,
}

impl Value
{
    pub fn from_usize(number: usize) -> Value
    {
        match number
        {
             2 => Value::Two,
             3 => Value::Three,
             4 => Value::Four,
             5 => Value::Five,
             6 => Value::Six,
             7 => Value::Seven,
             8 => Value::Eight,
             9 => Value::Nine,
            10 => Value::Ten,
            11 => Value::Jack,
            12 => Value::Queen,
            13 => Value::King,
            14 => Value::Ace,
            15 => Value::Joker,
            _ => panic!("Can't create card value from number '{}'", number),
        }
    }
}

impl std::fmt::Display for Value
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error>
    {
        write!(f, "{}", match self 
            {
                Value::Two   => " 2",
                Value::Three => " 3",
                Value::Four  => " 4",
                Value::Five  => " 5",
                Value::Six   => " 6",
                Value::Seven => " 7",
                Value::Eight => " 8",
                Value::Nine  => " 9",
                Value::Ten   => "10",
                Value::Jack  => " J",
                Value::Queen => " Q",
                Value::King  => " K",
                Value::Ace   => " A",
                Value::Joker => "🃏",
            })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Suit
{
    Club,
    Spade,
    Heart,
    Diamond,
}

impl Suit
{
    pub fn same_color_as(self, other: Suit) -> bool
    {
        match self
        {
            Suit::Club | Suit::Spade => other == Suit::Club || other == Suit::Spade,
            Suit::Heart | Suit::Diamond => other == Suit::Heart || other == Suit::Diamond,
        }
    }

    pub fn from_usize(number: usize) -> Suit
    {
        match number
        {
            0 => Self::Club,
            1 => Self::Spade,
            2 => Self::Heart,
            3 => Self::Diamond,
            _ => panic!("Can't create card suit from number '{}'", number),
        }
    }
}

impl std::fmt::Display for Suit
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error>
    {
        write!(f, "{}", match self 
            {
                Self::Club => "♣",
                Self::Spade => "♠",
                Self::Heart => "♡",
                Self::Diamond => "♢",
            })
    }
}

#[derive(Debug, PartialEq)]
pub struct Card
{
    value : Value,
    suit  : Suit,
}

impl Card
{
    pub fn new(value: Value, suit: Suit) -> Self
    {
        Self {suit, value}
    }

    pub fn value(& self) -> Value
    {
        self.value
    }

    pub fn suit(& self) -> Suit
    {
        self.suit
    }

    pub fn signature(& self) -> String
    {
        format!("│{}{} │", self.value, self.suit)
    }
}

impl std::fmt::Display for Card
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error>
    {
        /*
        write!(f, "┌────┐
│{}{} │
│    │
└────┘\n", self.value, self.suit)
        */
        write!(f, "{}{}", self.value, self.suit)
    }
}

#[derive(Clone, Copy)]
pub enum Deck
{
    Reduced = 24,
    Standart = 36,
    Full = 52,
    Extended = 54,
}

pub fn output_cards(cards: & Vec<Card>)
{
    for _ in 0..cards.len()
    {
        print!("┌────┐");
    }
    println!();

    for card in cards
    {
        print!("|{card} |");
    }
    println!();

    for _ in 0..cards.len()
    {
        print!("│    │");
    }
    println!();

    for _ in 0..cards.len()
    {
        print!("└────┘");
    }
    println!();

    for i in 0..cards.len()
    {
        print!("  {:>2}  ", i);
    }
    println!();

}