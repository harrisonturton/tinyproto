
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

pub fn tokenize(input: &str) -> Result<Vec<Token>, anyhow::Error> {
    let mut tokens = vec![];

    let mut remaining = input;
    while !remaining.is_empty() {
        // "{"
        if let Ok((rest, _)) = open_brace(&remaining) {
            tokens.push(Token::OpenBrace);
            remaining = rest;
            continue;
        }

        // "}"
        if let Ok((rest, _)) = close_brace(&remaining) {
            tokens.push(Token::CloseBrace);
            remaining = rest;
            continue;
        }

        // "("
        if let Ok((rest, _)) = open_parens(&remaining) {
            tokens.push(Token::OpenParens);
            remaining = rest;
            continue;
        }

        // ")"
        if let Ok((rest, _)) = close_parens(&remaining) {
            tokens.push(Token::CloseParens);
            remaining = rest;
            continue;
        }

        // "="
        if let Ok((rest, _)) = equals(&remaining) {
            tokens.push(Token::Equals);
            remaining = rest;
            continue;
        }

        // "syntax"
        if let Ok((rest, _)) = syntax(&remaining) {
            tokens.push(Token::Syntax);
            remaining = rest;
            continue;
        }

        // "message"
        if let Ok((rest, _)) = message(&remaining) {
            tokens.push(Token::Message);
            remaining = rest;
            continue;
        }

        // "service"
        if let Ok((rest, _)) = service(&remaining) {
            tokens.push(Token::Service);
            remaining = rest;
            continue;
        }

        // "rpc"
        if let Ok((rest, _)) = rpc(&remaining) {
            tokens.push(Token::Rpc);
            remaining = rest;
            continue;
        }

        // "returns"
        if let Ok((rest, _)) = returns(&remaining) {
            tokens.push(Token::Returns);
            remaining = rest;
            continue;
        }

        // "stream"
        if let Ok((rest, _)) = stream(&remaining) {
            tokens.push(Token::Stream);
            remaining = rest;
            continue;
        }

        // ";"
        if let Ok((rest, _)) = semicolon(&remaining) {
            tokens.push(Token::Semicolon);
            remaining = rest;
            continue;
        }

        // Double-quote string literal
        if let Ok((rest, literal)) = string_literal(&remaining) {
            let literal = Literal::String(literal.to_string());
            tokens.push(Token::Literal(literal));
            remaining = rest;
            continue;
        }
       
        // Non-keyword identifier
        if let Ok((rest, ident)) = ident(&remaining) {
            tokens.push(Token::Ident(ident.to_string()));
            remaining = rest;
            continue;
        }

        return Err(anyhow::anyhow!("Unexpected character: {}", remaining));
    }

    Ok(tokens)
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