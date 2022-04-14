
use nom::bytes::complete::take_while1;
use nom::bytes::complete::is_not;
use nom::character::complete::char;
use nom::bytes::complete::tag;
use nom::sequence::delimited;
use nom::character::complete::multispace0;
use nom::IResult;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Ident(String),
    Literal(Literal),

    Syntax,
    Message,
    Service,
    Equals,
    Semicolon,
    OpenBrace,
    CloseBrace,
    Rpc,
    OpenParens,
    CloseParens,
    Returns,
    Stream,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    String(String),
}

pub struct Tokenizer<'a> {
    input: &'a str
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }
}

impl<'a> IntoIterator for Tokenizer<'a> {
    type Item = Token;
    type IntoIter = TokenizerIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        TokenizerIterator {
            input: self.input
        }
    }
}

pub struct TokenizerIterator<'a> {
    input: &'a str
}

impl<'a> Iterator for TokenizerIterator<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.input.is_empty() {
            return None;
        }

        // "{"
        if let Ok((rest, _)) = open_brace(&self.input) {
            self.input = rest;
            return Some(Token::OpenBrace);
        }

        // "}"
        if let Ok((rest, _)) = close_brace(&self.input) {
            self.input = rest;
            return Some(Token::CloseBrace);
        }

        // "("
        if let Ok((rest, _)) = open_parens(&self.input) {
            self.input = rest;
            return Some(Token::OpenParens);
        }

        // ")"
        if let Ok((rest, _)) = close_parens(&self.input) {
            self.input = rest;
            return Some(Token::CloseParens);
        }

        // "="
        if let Ok((rest, _)) = equals(&self.input) {
            self.input = rest;
            return Some(Token::Equals);
        }

        // "syntax"
        if let Ok((rest, _)) = syntax(&self.input) {
            self.input = rest;
            return Some(Token::Syntax);
        }

        // "message"
        if let Ok((rest, _)) = message(&self.input) {
            self.input = rest;
            return Some(Token::Message);
        }

        // "service"
        if let Ok((rest, _)) = service(&self.input) {
            self.input = rest;
            return Some(Token::Service);
        }

        // "rpc"
        if let Ok((rest, _)) = rpc(&self.input) {
            self.input = rest;
            return Some(Token::Rpc);
        }

        // "returns"
        if let Ok((rest, _)) = returns(&self.input) {
            self.input = rest;
            return Some(Token::Returns);
        }

        // "stream"
        if let Ok((rest, _)) = stream(&self.input) {
            self.input = rest;
            return Some(Token::Stream);
        }

        // ";"
        if let Ok((rest, _)) = semicolon(&self.input) {
            self.input = rest;
            return Some(Token::Semicolon);
        }

        // Double-quote string literal
        if let Ok((rest, literal)) = string_literal(&self.input) {
            self.input = rest;
            let literal = Literal::String(literal.to_string());
            return Some(Token::Literal(literal));
        }
       
        // Non-keyword identifier
        if let Ok((rest, ident)) = ident(&self.input) {
            self.input = rest;
            return Some(Token::Ident(ident.to_string()));
        }

        return None;
    }
}

fn ident(input: &str) -> IResult<&str, &str> {
    delimited(
        multispace0,
        take_while1(char::is_alphanumeric),
        multispace0,
    )(input)
}

fn string_literal(input: &str) -> IResult<&str, &str> {
    delimited(
        char('"'),
        is_not("\""),
        char('"'),
    )(input)
}

macro_rules! keyword {
    ($name:ident, $tag:expr) => {
        fn $name(input: &str) -> IResult<&str, &str> {
            delimited(
                multispace0,
                tag($tag),
                multispace0,
            )(input)
        } 
    };
}

keyword!(open_brace, "{");
keyword!(close_brace, "}");
keyword!(open_parens, "(");
keyword!(close_parens, ")");
keyword!(equals, "=");
keyword!(semicolon, ";");
keyword!(syntax, "syntax");
keyword!(message, "message");
keyword!(service, "service");
keyword!(rpc, "rpc");
keyword!(returns, "returns");
keyword!(stream, "stream");