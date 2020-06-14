use nom::{
    IResult,
    Err,
    error::ErrorKind,
    sequence::preceded,
    character::complete::{digit1, space1},
};

use super::styled_text::{eat_text, StyledText};

pub fn txt_arg(given: &str) -> IResult<&str, StyledText> {
    preceded(space1, eat_text)(given)
}

pub fn num_arg(given: &str) -> IResult<&str, u8> {
    let (rest, num_as_str) = preceded(space1, digit1)(given)?;

    match num_as_str.parse::<u8>() {
        Ok(num) => Ok((rest, num)),
        Err(_) => Err(Err::Error((rest, ErrorKind::Digit)))
    }
}
