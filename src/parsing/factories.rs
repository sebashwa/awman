use super::doc_structure::PageTitle;
use super::styled_text::{StyledString, StringStyle, StyledText};

pub fn styled_str(content: &str, style: StringStyle) -> StyledString {
    StyledString { style, content: String::from(content) }
}

pub fn roman_str(given: &str) -> StyledString {
    styled_str(given, StringStyle::Roman)
}

pub fn bold_str(given: &str) -> StyledString {
    styled_str(given, StringStyle::Bold)
}

pub fn italic_str(given: &str) -> StyledString {
    styled_str(given, StringStyle::Italic)
}

pub fn roman_txt(given: &str) -> StyledText {
    vec!(roman_str(given))
}

pub fn default_page_title() -> PageTitle {
    PageTitle {
        title: roman_txt("Title"),
        section: None,
        footer_middle: None,
        footer_outside: None,
        header_middle: None,
    }
}

