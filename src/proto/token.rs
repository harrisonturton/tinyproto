
use nom::bytes::complete::take_while1;
use nom::bytes::complete::take_while;
use nom::bytes::complete::is_not;
use nom::character::complete::char;
use nom::bytes::complete::tag;
use nom::sequence::delimited;
use nom::character::complete::multispace0;
use nom::error::ParseError;
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

pub fn tokenize(input: String) -> Result<Vec<Token>, anyhow::Error> {
    let mut tokens = vec![];

    let mut remaining = input;
    while !remaining.is_empty() {
        // "{"
        if let Ok((rest, _)) = open_brace(&remaining) {
            tokens.push(Token::OpenBrace);
            remaining = rest.to_string();
            continue;
        }

        // "}"
        if let Ok((rest, _)) = close_brace(&remaining) {
            tokens.push(Token::CloseBrace);
            remaining = rest.to_string();
            continue;
        }

        // "("
        if let Ok((rest, _)) = open_parens(&remaining) {
            tokens.push(Token::OpenParens);
            remaining = rest.to_string();
            continue;
        }

        // ")"
        if let Ok((rest, _)) = close_parens(&remaining) {
            tokens.push(Token::CloseParens);
            remaining = rest.to_string();
            continue;
        }

        // A string literal
        if let Ok((rest, literal)) = string_literal(&remaining) {
            let literal = Literal::String(literal.to_string());
            tokens.push(Token::Literal(literal));
            remaining = rest.to_string();
            continue;
        }
        
        if let Ok((rest, ident)) = ident(&remaining) {
            tokens.push(Token::Ident(ident.to_string()));
            remaining = rest.to_string();
            continue;
        }

        // "="
        if let Ok((rest, _)) = equals(&remaining) {
            tokens.push(Token::Equals);
            remaining = rest.to_string();
            continue;
        }

        // "syntax"
        if let Ok((rest, _)) = syntax(&remaining) {
            tokens.push(Token::Syntax);
            remaining = rest.to_string();
            continue;
        }

        // "message"
        if let Ok((rest, _)) = message(&remaining) {
            tokens.push(Token::Message);
            remaining = rest.to_string();
            continue;
        }

        // "service"
        if let Ok((rest, _)) = service(&remaining) {
            tokens.push(Token::Service);
            remaining = rest.to_string();
            continue;
        }

        // "rpc"
        if let Ok((rest, _)) = rpc(&remaining) {
            tokens.push(Token::Rpc);
            remaining = rest.to_string();
            continue;
        }

        // "returns"
        if let Ok((rest, _)) = returns(&remaining) {
            tokens.push(Token::Returns);
            remaining = rest.to_string();
            continue;
        }

        // "stream"
        if let Ok((rest, _)) = stream(&remaining) {
            tokens.push(Token::Stream);
            remaining = rest.to_string();
            continue;
        }

        // ";"
        if let Ok((rest, _)) = semicolon(&remaining) {
            tokens.push(Token::Semicolon);
            remaining = rest.to_string();
            continue;
        }

        return Err(anyhow::anyhow!("Unexpected character: {}", remaining));
    }

    Ok(tokens)
}

fn ident(input: &str) -> IResult<&str, &str> {
    ws(take_while1(char::is_alphanumeric))(input)
}

fn string_literal(input: &str) -> IResult<&str, &str> {
    delimited(
        char('"'),
        is_not("\""),
        char('"'),
    )(input)
}

fn open_brace(input: &str) -> IResult<&str, &str> {
    ws(tag("{"))(input)
}

fn close_brace(input: &str) -> IResult<&str, &str> {
    ws(tag("}"))(input)
}

fn open_parens(input: &str) -> IResult<&str, &str> {
    ws(tag("("))(input)
}

fn close_parens(input: &str) -> IResult<&str, &str> {
    ws(tag(")"))(input)
}

fn equals(input: &str) -> IResult<&str, &str> {
    ws(tag("="))(input)
}

fn syntax(input: &str) -> IResult<&str, &str> {
    ws(tag("syntax"))(input)
}

fn message(input: &str) -> IResult<&str, &str> {
    ws(tag("message"))(input)
}

fn service(input: &str) -> IResult<&str, &str> {
    ws(tag("service"))(input)
}

fn rpc(input: &str) -> IResult<&str, &str> {
    ws(tag("rpc"))(input)
}

fn returns(input: &str) -> IResult<&str, &str> {
    ws(tag("returns"))(input)
}

fn stream(input: &str) -> IResult<&str, &str> {
    ws(tag("stream"))(input)
}

fn semicolon(input: &str) -> IResult<&str, &str> {
    ws(tag(";"))(input)
}

fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) -> impl Fn(&'a str) -> IResult<&'a str, O, E>
  where
  F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
  move |i| {
    delimited(
      multispace0,
      &inner,
      multispace0
    )(i)
  }
}
