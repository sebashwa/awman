use nom::{
    IResult,
    branch::alt,
    bytes::complete::{escaped_transform, tag, take_until, take_till, is_not},
    combinator::{opt, value},
    character::complete::{not_line_ending},
    multi::{separated_list0},
    sequence::{delimited, tuple},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StringStyle { Roman, Bold, Italic }

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StyledString {
    pub style: StringStyle,
    pub content: String,

}

pub type StyledText = Vec<StyledString>;

fn prefix_to_string_style(opt_prefix: Option<&str>, mut previous_style: StringStyle) -> StringStyle {
    let style = match opt_prefix {
        Some(prefix) => match prefix {
            "B" => StringStyle::Bold,
            "I" => StringStyle::Italic,
            "R" => StringStyle::Roman,
            _ => previous_style,
        },
        None => previous_style
    };
    previous_style = style;
    style
}

fn escape_segment(given: &str) -> String {
    let content: IResult<&str, String> = escaped_transform(is_not(r"\"), '\\',
        alt((
            value(r"\", tag("e")),
            value("-", tag("-")),
            value("", tag("&")),
            value("~", tag("(ti")),
            value("^", tag("(ha")),
            value("`", tag("(ga")),
            value("–", tag("(en")),
            value("—", tag("(em")),
            value("“", tag("(lq")),
            value("”", tag("(rq")),
            value("\"", tag("(dq")),
            value("‘", tag("(oq")),
            value("’", tag("(cq")),
            value("'", tag("(aq")),
        ))
    )(given);

    match content {
        Ok((_, result)) => result,
        Err(_) => "".to_string()
    }
}

fn process_segment(given: &str, previous_style: StringStyle) -> StyledString {
    let style = prefix_to_string_style(given.get(0..1), previous_style);
    let content = escape_segment(given.get(1..).unwrap_or_default());

    StyledString { content, style }
}

fn process_first_segment(segment: &str, is_styled: bool, previous_style: StringStyle) -> StyledString {
    if is_styled {
        process_segment(segment, previous_style)
    } else {
        StyledString { content: escape_segment(segment), style: previous_style }
    }
}

pub fn eat_text(given: &str) -> IResult<&str, StyledText> {
    let separate_by_style_prefix = separated_list0(tag(r"\f"), alt((take_until(r"\f"), not_line_ending)));
    let previous_style = StringStyle::Roman;

    if given == "" { return Ok(("", vec![])) }

    let mut result;
    let (rest, (first_prefix, segments)) = tuple((opt(tag(r"\f")), separate_by_style_prefix))(given)?;

    let first_segment = segments.get(0);
    if first_segment.is_some() {
        let first_styled_string = process_first_segment(segments[0], first_prefix.is_some(), previous_style);
        let other_styled_strings = segments[1..].iter().map(|s| process_segment(s, previous_style));

        result = vec![first_styled_string];
        result.extend(other_styled_strings);
    } else {
        result = vec![];
    }

    Ok((rest, result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsing::factories::{roman_str, bold_str, italic_str, roman_txt};

    #[test]
    fn eat_text_can_digest_word() {
        assert_eq!(eat_text("Foo"), Ok(("", roman_txt("Foo"))));
    }

    #[test]
    fn eat_text_can_handle_empty_strings() {
        assert_eq!(eat_text(""), Ok(("", vec![])));
    }

    #[test]
    fn eat_text_works_with_umlauts() {
        assert_eq!(eat_text("Föö"), Ok(("", roman_txt("Föö"))));
    }

    #[test]
    fn eat_text_can_digest_quote_delimited_sentence() {
        assert_eq!(eat_text(r#""Some" Fi Fa "Foo Bär""#), Ok(("", roman_txt(r#""Some" Fi Fa "Foo Bär""#))));
    }

    #[test]
    fn eat_text_can_handle_a_style_mode() {
        let given = r#"\fBFoo"#;
        let expected = vec![bold_str("Foo")];

        assert_eq!(eat_text(given), Ok(("", expected)));
    }

    #[test]
    fn eat_text_can_handle_a_trailing_style_mode() {
        let given = r#"\fBFoo\fR"#;
        let expected = vec![bold_str("Foo"), roman_str("")];

        assert_eq!(eat_text(given), Ok(("", expected)));
    }

    #[test]
    fn eat_text_can_switch_between_different_style_modes() {
        let given = r#"\fRFoo\fBBar\fIBaz"#;
        let expected = vec![roman_str("Foo"), bold_str("Bar"), italic_str("Baz")];

        assert_eq!(eat_text(given), Ok(("", expected)));
    }

    #[test]
    fn eat_text_can_return_to_the_previous_style_mode() {
        let given = r#"\fRFoo\fBBar\fPBaz\fIBem\fPBum"#;
        let expected = vec![roman_str("Foo"), bold_str("Bar"), roman_str("Baz"), italic_str("Bem"), roman_str("Bum")];

        assert_eq!(eat_text(given), Ok(("", expected)));
    }

    #[test]
    fn eat_text_replaces_escape_sequences_correctly() {
        let given = r"\e \- \& \(ti \(ha \(ga \(en \(em \(lq \(rq \(dq \(oq \(cq \(aq";
        let expected = vec![roman_str(r#"\ -  ~ ^ ` – — “ ” " ‘ ’ '"#)];

        assert_eq!(eat_text(given), Ok(("", expected)));
    }
}
