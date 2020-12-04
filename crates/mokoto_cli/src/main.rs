use mokoto::parser::{Parse, Parser};
use mokoto::syntax::{ast::AstNode, nodes::Type};
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut input = String::new();

    loop {
        write!(stdout, "→ ")?;
        stdout.flush()?;

        stdin.read_line(&mut input)?;

        let parse : Parse = Parser::new(&input).parse_typ();
        println!("{}", parse.debug_tree());

        let ty: Type = AstNode::cast(parse.syntax()).unwrap();

        println!("{:?}", ty);

        input.clear();
    }
}
