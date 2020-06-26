use nom::{
    IResult,
    Err,
    branch::alt,
    bytes::complete::{tag, take_till},
    error::ErrorKind,
    sequence::{delimited, preceded},
    character::complete::{char, digit1, space1},
};

use super::styled_text::{eat_text, StyledText};

pub fn txt_arg(given: &str) -> IResult<&str, StyledText> {
    let eat_quoted_arg = delimited(tag("\""), take_till(|c| c == '"'), tag("\""));
    let eat_word = take_till(|c| c == ' ');

    let (rest, arg) = preceded(space1, alt((eat_quoted_arg, eat_word)))(given)?;
    let (_, styled_text) = eat_text(arg)?;

    Ok((rest, styled_text))
}

pub fn num_arg(given: &str) -> IResult<&str, u8> {
    let (rest, num_as_str) = preceded(space1, digit1)(given)?;

    match num_as_str.parse::<u8>() {
        Ok(num) => Ok((rest, num)),
        Err(_) => Err(Err::Error((rest, ErrorKind::Digit)))
    }
}
