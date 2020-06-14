use nom::{
    IResult,
    combinator::opt,
    sequence::tuple,
    bytes::complete::tag,
};

use super::helpers::{txt_arg, num_arg};
use super::styled_text::StyledText;

type PageSection = u8;
#[derive(Debug, PartialEq, Eq)]
pub struct PageTitle {
    pub title: StyledText,
    pub section: Option<PageSection>,
    pub footer_middle: Option<StyledText>,
    pub footer_outside: Option<StyledText>,
    pub header_middle: Option<StyledText>
}

pub fn eat_title(given: &str) -> IResult<&str, PageTitle>  {
    let (rest, (_, title, section, footer_middle, footer_outside, header_middle)) =
        tuple((tag(".TH"), txt_arg, opt(num_arg), opt(txt_arg), opt(txt_arg), opt(txt_arg)))(given)?;

    let title = PageTitle { title, section, footer_middle, footer_outside, header_middle };

    Ok((rest, title))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsing::factories::{default_page_title, roman_txt};


    #[test]
    fn eat_title_returns_page_title() {
        let title_line = ".TH Title";

        let title = roman_txt("Title");
        let expected = PageTitle { title, ..default_page_title() };

        assert_eq!(eat_title(title_line), Ok(("", expected)));
    }

    #[test]
    fn eat_title_works_with_quoted_title() {
        let title_line = r#".TH "Multi Word Title""#;

        let title = roman_txt("Multi Word Title");
        let expected = PageTitle { title, ..default_page_title() };

        assert_eq!(eat_title(title_line), Ok(("", expected)));
    }

    #[test]
    fn eat_title_works_with_all_options() {
        let title_line = r#".TH "Multi Title" 7 "Ftr middle" "Ftr left" Header"#;

        let expected = PageTitle {
            title: roman_txt("Multi Title"),
            section: Some(7),
            footer_middle: Some(roman_txt("Ftr middle")),
            footer_outside: Some(roman_txt("Ftr left")),
            header_middle: Some(roman_txt("Header")),
        };

        assert_eq!(eat_title(title_line), Ok(("", expected)));
    }
}
