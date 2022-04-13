use nom::character::complete::none_of;
use nom::multi::separated_list0;
use nom::sequence::tuple;
use nom::sequence::separated_pair;
use nom::combinator::opt;
use nom::bytes::complete::is_not;
use nom::character::complete::anychar;
use nom::error::ParseError;
use nom::character::complete::multispace1;
use nom::bytes::complete::take_while;
use nom::bytes::complete::take_till;
use nom::bytes::complete::take_until;
use nom::sequence::terminated;
use nom::multi::many0;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::character::complete::char;
use nom::branch::alt;
use nom::character::complete::multispace0;
use nom::sequence::delimited;
use nom::bytes::complete::tag;
use nom::IResult;

use super::descriptor::*;

type Message<'a> = (&'a str, Vec<MessageField<'a>>);

pub fn service_header(input: &str) -> IResult<&str, &str> {
    preceded(
        ws(tag("service")),
        take_while(char::is_alphanumeric),
    )(input)
}

pub fn message_header(input: &str) -> IResult<&str, &str> {
    preceded(
        ws(tag("message")),
        take_while(char::is_alphanumeric),
    )(input)
}

pub fn brace_delimited(input: &str) -> IResult<&str, &str> {
    delimited(
        ws(tag("{")),
        take_until("}"),
        ws(tag("}")),
    )(input)
}

pub fn service_method(input: &str) -> IResult<&str, (&str, (Option<&str>, &str), (Option<&str>, &str))> {
    tuple((
        preceded(
            ws(tag("rpc")),
            ws(take_while(char::is_alphanumeric)),
        ),
        service_method_type,
        terminated(
            preceded(
                ws(tag("returns")),
                service_method_type,
            ),
            ws(tag(";"))
        ),
    ))(input)
}

pub fn service_method_type(input: &str) -> IResult<&str, (Option<&str>, &str)> {
    delimited(
        ws(tag("(")),
        tuple((
            opt(ws(tag("stream"))),
            ws(take_while(char::is_alphanumeric)),
        )),
        ws(tag(")")),
    )(input)
}

type MessageField<'a> = (Option<&'a str>, &'a str, &'a str, &'a str);

pub fn message_field(input: &str) -> IResult<&str, MessageField> {
    tuple((
        opt(message_field_label),
        ws(take_while(char::is_alphanumeric)),
        ws(take_while(char::is_alphanumeric)),
        delimited(
            ws(tag("=")),
            take_while(char::is_alphanumeric),
            ws(tag(";")),
        ),
    ))(input)
}

pub fn message_field_label(input: &str) -> IResult<&str, &str> {
    alt((
        ws(tag("optional")),
        ws(tag("required")),
    ))(input)
}

pub fn syntax_statement(input: &str) -> IResult<&str, &str> {
    terminated(
        preceded(
            pair(
                ws(tag("syntax")),
                ws(tag("=")),
            ),
            syntax_literal
        ),
        ws(tag(";")),
    )(input)
}

pub fn syntax_literal(input: &str) -> IResult<&str, &str> {
    delimited(
        char('"'),
        alt((
            tag("proto2"),
            tag("proto3"),
        )),
        char('"'),
    )(input)
}

pub fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) -> impl Fn(&'a str) -> IResult<&'a str, O, E>
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_message_field_with_label() -> Result<(), anyhow::Error> {
        let input = "optional string name = 1;";
        let expected = ("", (Some("optional"), "string", "name", "1"));
        let actual = message_field(input)?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn can_parse_message_field_without_label() -> Result<(), anyhow::Error> {
        let input = "string name = 1;";
        let expected = ("", (None, "string", "name", "1"));
        let actual = message_field(input)?;
        assert_eq!(expected, actual);
        Ok(())
    }


    #[test]
    fn can_parse_message_field_label() -> Result<(), anyhow::Error> {
        let input = "required string name = 1";
        let expected = ("string name = 1", "required");
        let actual = message_field_label(input)?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn can_parse_syntax_statement() -> Result<(), anyhow::Error> {
        let input = "syntax=   \"proto3\";";
        let expected = ("", "proto3");
        let actual = syntax_statement(input)?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn can_parse_syntax_literal() -> Result<(), anyhow::Error> {
        let input = "\"proto3\"";
        let expected = ("", "proto3");
        let actual = syntax_literal(input)?;
        assert_eq!(expected, actual);
        Ok(())
    }
}