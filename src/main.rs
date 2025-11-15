mod lexer;
mod parser;
mod runtime;
mod builtins;

fn main() {
    println!("Python-like-Rust Interpreter v0.1");
    let code = r#"
x = 10
y = x + 20
print(y)
"#;

    let tokens = lexer::lex(code);
    let ast = parser::parse(tokens);
    runtime::run(ast);
}
