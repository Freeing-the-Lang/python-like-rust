use regex::Regex;

#[derive(Debug, Clone)]
pub enum Token {
    Number(i64),
    Ident(String),
    Assign,
    Plus,
    Newline,
}

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    let assign = Regex::new(r"=").unwrap();
    let plus = Regex::new(r"\+").unwrap();
    let number = Regex::new(r"[0-9]+").unwrap();
    let ident = Regex::new(r"[a-zA-Z_][a-zA-Z0-9_]*").unwrap();

    for line in input.lines() {
        let mut s = line.trim().to_string();

        while !s.is_empty() {
            if let Some(m) = assign.find(&s) {
                tokens.push(Token::Assign);
                s = s[m.end()..].trim().to_string();
            } else if let Some(m) = plus.find(&s) {
                tokens.push(Token::Plus);
                s = s[m.end()..].trim().to_string();
            } else if let Some(m) = number.find(&s) {
                let n = m.as_str().parse().unwrap();
                tokens.push(Token::Number(n));
                s = s[m.end()..].trim().to_string();
            } else if let Some(m) = ident.find(&s) {
                tokens.push(Token::Ident(m.as_str().into()));
                s = s[m.end()..].trim().to_string();
            } else {
                break;
            }
        }

        tokens.push(Token::Newline);
    }

    tokens
}
