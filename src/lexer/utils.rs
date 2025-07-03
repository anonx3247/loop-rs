use crate::lexer::token::Comment;
use crate::lexer::lexer::LexerError;

pub fn index_until_boundary_excluding(literal: &str, excluding: Vec<char>) -> (&str, usize) {
    let mut index = 0;
    let mut char_indices = literal.char_indices();
    while let Some((i, c)) = char_indices.next() {
        if (c.is_whitespace() || !c.is_alphanumeric()) && !excluding.contains(&c) {
            break;
        }
        index = i + c.len_utf8();
    }
    (&literal[..index], index)

}

pub fn index_until_boundary(literal: &str) -> (&str, usize) {
    index_until_boundary_excluding(literal, Vec::new())
}

pub fn index_until_char(literal: &str, char: char) -> (&str, usize) {
    let mut index = 0;
    let mut char_indices = literal.char_indices();
    while let Some((i, c)) = char_indices.next() {
        if c == char {
            break;
        }
        index = i + c.len_utf8();
    }
    (&literal[..index], index) 
}

pub fn tokenize_comment(comment: &String) -> Result<(Comment, usize), LexerError> {
    match comment {
        _ if comment.starts_with("--") => {
            let (comment, index) = index_until_char(comment.as_str(), '\n');
            Ok((Comment::SingleLine(String::from(comment)), index))
        }
        _ => Err(LexerError::InvalidComment(comment.to_string())),
    }
}

pub fn find_matching_bracket(literal: &String, opening_bracket: char, closing_bracket: char) -> Result<usize, LexerError> {
    let mut cursor = 1;
    let mut open_brackets = 1;
    while open_brackets > 0 {
        if cursor >= literal.len() {
            return Err(LexerError::NoMatchingBracket(opening_bracket.to_string()));
        }
        if literal[cursor..].starts_with(opening_bracket) {
            open_brackets += 1;
        }
        if literal[cursor..].starts_with(closing_bracket) {
            open_brackets -= 1;
        }
        cursor += 1;
    }
    Ok(cursor)
}

pub fn preview(source: &String) -> String {
    let mut index = 0;
    let mut char_indices = source.char_indices();
    while let Some((i, c)) = char_indices.next() {
        if c.is_whitespace() {
            break;
        }
        index = i + c.len_utf8();
    }
    source[..index].to_string()
}

pub fn get_string_interpolations(string: &String) -> Vec<(String, usize)> {
    let mut interpolations = Vec::new();
    let mut cursor = 0;
    while cursor < string.len() {
        if string[cursor..].starts_with('{') {
            let match_index = find_matching_bracket(&string[cursor..].to_string(), '{', '}').unwrap();
            interpolations.push((string[cursor+1..cursor+match_index-1].to_string(), cursor));
            cursor += match_index + 1;
        } else {
            cursor += 1;
        }
    }
    interpolations
}