use mokoto::parser::{Parser};

use insta::{assert_snapshot, glob};

#[test]
fn parse_types() {
    use std::fs;

    glob!("passing/types/*.mo", |path| {
        let input = fs::read_to_string(path).unwrap();
        for inp in input.split("---\n") {
          let parse = Parser::new(&inp).parse_typ();
          assert_snapshot!(parse.debug_tree());
        }
    });
}
