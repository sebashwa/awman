mod helpers;
mod factories;
pub mod styled_text;
pub mod doc_structure;

use std::fs::File;
use std::io::{Lines, BufRead, BufReader};
use nom::IResult;
use doc_structure::{eat_page_title, PageTitle};
use styled_text::StyledText;

#[derive(Debug, PartialEq, Eq)]
pub enum ManfileComponent {
    PageTitle(PageTitle),
    StyledText(StyledText)
}

enum LineParser {
    MultilineParser(Box<dyn for<'a> Fn(&'a str, &'a Lines<BufReader<File>>, usize) -> (IResult<&'a str, ManfileComponent>, usize)>),
    SinglelineParser(Box<dyn Fn(&str) -> IResult<&str, ManfileComponent>>)
}

fn get_file_lines() -> Lines<BufReader<File>> {
    let file = match File::open("assets/already_working.man1") {
        Ok(f) => f,
        Err(e) => panic!(e)
    };

    let reader = BufReader::new(file);
    reader.lines()
}

fn get_line_parser(line: &str) -> LineParser {
    return LineParser::SinglelineParser(Box::new(eat_page_title));
}

fn parse(mut lines: Lines<BufReader<File>>, n: usize, mut result: Vec<ManfileComponent>) -> Vec<ManfileComponent> {
    let le_line = lines.nth(n);
    let line = match le_line {
        Some(l) => l.unwrap_or("".to_string()),
        None => return result
    };

    let line_parser = get_line_parser(&line);

    let (line_result, next_line) = match line_parser {
        LineParser::MultilineParser(p) => p(&line, &lines, n),
        LineParser::SinglelineParser(p) => (p(&line), n+1)
    };

    result.push(line_result.unwrap().1);

    parse(lines, next_line, result)
}

fn parse_file() -> Vec<ManfileComponent> {
    let lines = get_file_lines();
    parse(lines, 0, vec![])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_file_works() {
        assert_eq!(parse_file(), vec![]);
    }
}
