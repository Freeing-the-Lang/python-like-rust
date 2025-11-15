use std::collections::HashMap;
use crate::parser::Node;

pub fn run(nodes: Vec<Node>) {
    let mut env = HashMap::new();

    for node in nodes {
        match node {
            Node::Assign(name, expr) => {
                let value = eval(&expr, &env);
                env.insert(name, value);
            }
            _ => {}
        }
    }

    if let Some(v) = env.get("y") {
        println!(">>> {}", v);
    }
}

fn eval(node: &Node, env: &HashMap<String, i64>) -> i64 {
    match node {
        Node::Number(n) => *n,
        Node::Ident(id) => *env.get(id).unwrap_or(&0),
        Node::Add(a, b) => eval(a, env) + eval(b, env),
        _ => 0,
    }
}
