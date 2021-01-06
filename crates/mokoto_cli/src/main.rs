use mokoto::parser::{Parse, Parser};
use mokoto::syntax::{ast::AstNode, nodes::Type, nodes::Path};
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut input = String::new();

    loop {
        write!(stdout, "→ ")?;
        stdout.flush()?;

        stdin.read_line(&mut input)?;

        let parse: Parse = Parser::new(&input).parse_typ();
        println!("{}", parse.debug_tree());

        let ty: Type = AstNode::cast(parse.syntax()).unwrap();
        match ty {
            Type::PathType(path) => {
                println!("{:?}<{:?}>", path.path().unwrap().segments(), path.type_args().unwrap());
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

        input.clear();
    }
}
