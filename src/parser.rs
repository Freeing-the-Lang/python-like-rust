use crate::lexer::Token;

#[derive(Debug)]
pub enum Node {
    Assign(String, Box<Node>),
    Add(Box<Node>, Box<Node>),
    Number(i64),
    Ident(String),
    Noop,
}

pub fn parse(tokens: Vec<Token>) -> Vec<Node> {
    let mut nodes = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        match &tokens[i] {
            Token::Ident(name) => {
                if let Token::Assign = tokens[i + 1] {
                    let value = match &tokens[i + 2] {
                        Token::Number(n) => Node::Number(*n),
                        Token::Ident(id) => Node::Ident(id.clone()),
                        _ => Node::Noop,
                    };
                    nodes.push(Node::Assign(name.clone(), Box::new(value)));
                    i += 3;
                } else {
                    i += 1;
                }
            }
            _ => i += 1,
        }
    }

    nodes
}
