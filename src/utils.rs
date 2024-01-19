use std::{io::Write, num::{ParseIntError, IntErrorKind}};

#[derive(Debug)]
pub enum Error
{
    TooManyPlayers(usize),
    TooManyAttackCards,
    AbsentCardValue(crate::cards::Value),
    NoCardsToBeat,
    InvalidAttackIndex(usize),
    InvalidDeckIndex(usize),
    IncorrectDefense,
}

impl std::fmt::Display for Error
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error>
    {
        write!(f, "{}", match self 
            {
                Self::TooManyPlayers(max_count) => format!("Can't add more than {max_count} players to this game"),
                Self::TooManyAttackCards        => "Maximum 6 attack cards".to_string(),
                Self::AbsentCardValue(value)    => format!("There isn't such cards with value '{value}' on the table"),
                Self::NoCardsToBeat             => "There isn't any card to beat".to_string(),
                Self::InvalidAttackIndex(index) => format!("There isn't attack card at #{index}"),
                Self::InvalidDeckIndex(index)   => format!("You haven't card at #{index}"),
                Self::IncorrectDefense           => "Given defense card can't beat given attack card".to_string(),
            })
    }
}

impl std::error::Error for Error
{
}

pub const INDENT_SIZE: usize = 3;

macro_rules! log
{
    ($indent:literal, $format:literal) =>
    {
        print!("{2:1$}{0}", $format, $indent * INDENT_SIZE, "")
    };

    ($indent:literal, $format:literal, $($arg:tt), +) =>
    {
        print!("{2:1$}{0}", format!($format, $($arg), +), $indent * INDENT_SIZE, "")
    };

    ($indent:ident, $format:ident) =>
    {
        print!("{2:1$}{0}", $format, $indent * INDENT_SIZE, "")
    };

    ($indent:ident, $format:ident, $($arg:tt), +) =>
    {
        print!("{2:1$}{0}", format!($format, $($arg), +), $indent * INDENT_SIZE, "")
    };
}

macro_rules! logln
{
    () =>
    {
      println!()  
    };

    ($indent:literal, $format:literal) =>
    {
        println!("{2:1$}{0}", $format, $indent * INDENT_SIZE, "")
    };

    ($indent:literal, $format:literal, $($arg:tt), +) =>
    {
        println!("{2:1$}{0}", format!($format, $($arg), +), $indent * INDENT_SIZE, "")
    };

    ($indent:ident, $format:ident) =>
    {
        println!("{2:1$}{0}", $format, $indent * INDENT_SIZE, "")
    };

    ($indent:ident, $format:ident, $($arg:tt), +) =>
    {
        println!("{2:1$}{0}", format!($format, $($arg), +), $indent * INDENT_SIZE, "")
    };

}

pub(crate) use {log, logln}; 

pub enum Input
{
    String(String),
    Number(usize),
}

pub fn get_input(indent_level: usize, message: & str) -> Input
{
    log!(indent_level, message);
    std::io::stdout().flush();

    let mut string = String::new();
    loop
    {
        std::io::stdin().read_line(&mut string).expect("'read_line' error");  
        string.retain(|c| !c.is_whitespace());

        match string.parse()
        {
            Ok(number) => return Input::Number(number),
            Err(error) =>
                match error.kind()
                {
                    IntErrorKind::InvalidDigit => return Input::String(string),
                    _ => println!("Parse error: {error}"),
                }
        }
    }
}

pub fn positive_sub_or_zero<T: num::traits::PrimInt + num::traits::Unsigned>(lhs: T, rhs: T) -> T
{
    if lhs <= rhs
    {
        return T::zero();
    }
    lhs - rhs
}