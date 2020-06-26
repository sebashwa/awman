use nom::{
    IResult,
    branch::alt,
    bytes::complete::{tag, take_till},
    sequence::{delimited, preceded},
    character::complete::space1,
};

use super::styled_text::{eat_text, StyledText};

pub fn txt_arg(given: &str) -> IResult<&str, StyledText> {
    let eat_quoted_arg = delimited(tag("\""), take_till(|c| c == '"'), tag("\""));
    let eat_word = take_till(|c| c == ' ');

    let (rest, arg) = preceded(space1, alt((eat_quoted_arg, eat_word)))(given)?;
    let (_, styled_text) = eat_text(arg)?;

    Ok((rest, styled_text))
}
