use parser::parse;

use std::{io::{self, Write}, path::Path};

fn request_input() -> io::Result<()>{
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut input = String::new();

    loop {
        write!(stdout, "→ ")?;
        stdout.flush()?;

        stdin.read_line(&mut input)?;

        let parse = parse(&input);
        println!("{}", parse.debug_tree());

        let root = ast::Root::cast(parse.syntax()).unwrap();

        dbg!(root
            .stmts()
            .filter_map(|stmt| if let ast::Stmt::VariableDef(var_def) = stmt {
                Some(var_def.value())
            } else {
                None
            })
            .collect::<Vec<_>>());

            dbg!(hir::lower(root));

        input.clear();
    }
}

fn read_code_from_file() {
    let path = Path::new("./prate/test_programm.pr");
    let input = std::fs::read_to_string(&path).unwrap();
    let parse = parse(&input);
    println!("{}", parse.debug_tree());

    let root = ast::Root::cast(parse.syntax()).unwrap();

    dbg!(root
        .stmts()
        .filter_map(|stmt| if let ast::Stmt::VariableDef(var_def) = stmt {
            Some(var_def.value())
        } else {
            None
        })
        .collect::<Vec<_>>());

        dbg!(hir::lower(root));

}
fn main() {//-> io::Result<()>{
   // request_input()
   read_code_from_file()
}
