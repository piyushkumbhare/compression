use std::fmt::{Debug, Write};

/*
    THIS IS CURRENTLY AN UNUSED FILE IN THIS PROJECT. 
    
    It may see some use in the future, but honestly 
    it was just created so I could get some practice with
    writing some Impl blocks.
*/

#[derive(Debug)]
pub enum Token {
    Char(char),
    DELIM,
}

pub struct Tokens(pub Vec<Token>);

impl From<String> for Tokens {
    fn from(value: String) -> Self {
        let v: Vec<Token> = value.chars().map(|c| Token::Char(c)).collect();
        Tokens(v)
    }
}

impl From<&String> for Tokens {
    fn from(value: &String) -> Self {
        let v: Vec<Token> = value.chars().map(|c| Token::Char(c)).collect();
        Tokens(v)
    }
}

impl Debug for Tokens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for curr in self.0.iter() {
            match curr {
                Token::Char(c) => f.write_char(*c),
                Token::DELIM => f.write_str("(DELIM)"),
            };
        }
        Ok(())
    }
}