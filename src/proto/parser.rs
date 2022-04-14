use nom::combinator::opt;
use nom::sequence::tuple;
use nom::error::ParseError;
use nom::bytes::complete::take_while;
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

pub fn syntax(input: &str) -> IResult<&str, SyntaxDescriptor> {
    let mut parser = terminated(
        preceded(
            pair(
                ws(tag("syntax")),
                ws(tag("=")),
            ),
            syntax_literal
        ),
        ws(tag(";")),
    );
    let (rest, syntax) = parser(input)?;
    let descriptor = match syntax {
        "proto2" => SyntaxDescriptor::Proto2,
        "proto3" => SyntaxDescriptor::Proto3,
        unknown => SyntaxDescriptor::Unknown(unknown),
    };
    Ok((rest, descriptor))
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

pub fn message(input: &str) -> IResult<&str, MessageDescriptor> {
    let mut parser = tuple((
        preceded(
            ws(tag("message")),
            take_while(char::is_alphanumeric),
        ),
        delimited(
            ws(tag("{")),
            many0(message_field),
            ws(tag("}")),
        ),
    ));
    let (rest, (ident, fields)) = parser(input)?;
    let descriptor = MessageDescriptor {
        name: ident,
        fields: fields,
    };
    Ok((rest, descriptor))
}

pub fn message_field(input: &str) -> IResult<&str, FieldDescriptor> {
    let mut parser = tuple((
        opt(message_field_label),
        message_field_type,
        ws(take_while(char::is_alphanumeric)),
        delimited(
            ws(tag("=")),
            take_while(char::is_alphanumeric),
            ws(tag(";")),
        ),
    ));
    let (rest, (label, typ, name, number)) = parser(input)?;
    let descriptor = FieldDescriptor {
        label: label,
        typ: typ,
        name: name,
        number: number,
    };
    Ok((rest, descriptor))
}

pub fn message_field_label(input: &str) -> IResult<&str, FieldDescriptorLabel> {
    let mut parser = alt((
        ws(tag("optional")),
        ws(tag("required")),
        ws(tag("repeated")),
    ));
    let (rest, label) = parser(input)?;
    let descriptor = match label {
        "optional" => FieldDescriptorLabel::Optional,
        "required" => FieldDescriptorLabel::Required,
        "repeated" => FieldDescriptorLabel::Repeated,
        unknown => FieldDescriptorLabel::Unknown(unknown),
    };
    Ok((rest, descriptor))
}

pub fn message_field_type(input: &str) -> IResult<&str, FieldDescriptorType> {
    let parser = ws(take_while(char::is_alphanumeric));
    let (rest, typ) = parser(input)?;
    let descriptor = match typ {
        "string" => FieldDescriptorType::String,
        ident => FieldDescriptorType::Message(ident),
    };
    Ok((rest, descriptor))
}

pub fn service(input: &str) -> IResult<&str, ServiceDescriptor> {
    let mut parser = tuple((
        preceded(
            ws(tag("service")),
            take_while(char::is_alphanumeric),
        ),
        delimited(
            ws(tag("{")),
            many0(service_method),
            ws(tag("}")),
        ),
    ));
    let (rest, (name, methods)) = parser(input)?;
    let descriptor = ServiceDescriptor {
        name: name,
        methods: methods
    };
    Ok((rest, descriptor))
}

pub fn service_method(input: &str) -> IResult<&str, MethodDescriptor> {
    let mut parser = tuple((
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
    ));
    let (rest, (name, input, output)) = parser(input)?;
    let (client_streaming, input_type) = input;
    let (server_streaming, output_type) = output;
    let descriptor = MethodDescriptor {
        name: name,
        input_type: input_type,
        output_type: output_type,
        client_streaming: client_streaming,
        server_streaming: server_streaming,
    };
    Ok((rest, descriptor))
}

pub fn service_method_type(input: &str) -> IResult<&str, (bool, &str)> {
    let mut parser = delimited(
        ws(tag("(")),
        tuple((
            opt(ws(tag("stream"))),
            ws(take_while(char::is_alphanumeric)),
        )),
        ws(tag(")")),
    );
    let (rest, (streaming, ident)) = parser(input)?;
    Ok((rest, (streaming.is_some(), ident)))
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