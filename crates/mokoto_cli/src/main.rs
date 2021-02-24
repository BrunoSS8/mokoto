use mokoto::parser::{Parse, Parser};
use mokoto::syntax::nodes::Pattern;
use mokoto::syntax::{ast::AstNode, nodes::Path, nodes::Type};
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut input = String::new();

    loop {
        write!(stdout, "→ ")?;
        stdout.flush()?;

        stdin.read_line(&mut input)?;

        if let Some(input) = input.strip_prefix(":t ") {
            let parse: Parse = Parser::new(&input).parse_typ();
            println!("{}", parse.debug_tree());

            let ty: Type = AstNode::cast(parse.syntax()).unwrap();
            match ty {
                Type::PathType(path) => {
                    println!(
                        "{:?}<{:?}>",
                        path.path().unwrap().segments(),
                        path.type_args()
                    );
                }
                Type::FuncType(func) => {
                    println!(
                        "{:?} -> {:?}>",
                        func.func_arg().unwrap().ty(),
                        func.func_result().unwrap().ty()
                    );
                }
                ty => println!("{:?}", ty),
            }
        } else if let Some(input) = input.strip_prefix(":p ") {
            let parse: Parse = Parser::new(&input).parse_pattern();
            println!("{}", parse.debug_tree());

            let pat: Pattern = AstNode::cast(parse.syntax()).unwrap();
            let idents: Vec<String> = pat
                .idents()
                .iter()
                .map(|name| name.ident_token().unwrap().text().to_string())
                .collect();
            println!("Idents in pattern: {:?}", idents);
        }

        input.clear();
    }
}
