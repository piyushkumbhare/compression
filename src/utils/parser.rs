use regex::Regex;
use sscanf::sscanf;

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub enum Token {
    Delim,
    Char(char),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Tokens(pub Vec<Token>);

impl Tokens {

    // Takes a raw uncompressed (pre-BWT) string and parses it into Tokens.
    // Also appends a delim to the end.
    pub fn from_string(string: &str) -> Self {
        // When reading in an uncompressed string, simply parse each char as a char.
        let mut tokens: Vec<Token> = string.chars().map(|c| Token::Char(c)).collect();
        // Then append a delim token to the end. This string is now ready for the BWT.
        tokens.push(Token::Delim);
        Tokens(tokens)
    }

    // Parses a post-BWT string back into Tokens
    pub fn from_bwt(string: &str) -> Self {
        let re = Regex::new(r"\((\d+)\)([\w\W\n]*)").unwrap();
        let captures = re.captures(string).unwrap();

        let delim_pos: usize = usize::from_str_radix(&captures[1], 10).unwrap();
        let string = &captures[2];

        let mut tokens: Vec<Token> = string.chars().map(|c| Token::Char(c)).collect();
        tokens.insert(delim_pos, Token::Delim);
        Tokens(tokens)
    }

}
