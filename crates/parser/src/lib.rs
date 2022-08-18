mod event;
mod grammar;
mod parser;
mod sink;
mod source;

use lexer::Lexer;
use parser::Parser;
use rowan::GreenNode;
use sink::Sink;
use source::Source;
use syntax::SyntaxNode;

pub fn parse(input: &str) -> Parse {
    let tokens: Vec<_> = Lexer::new(input).collect();
    let source = Source::new(&tokens);
    let parser = Parser::new(source);
    let events = parser.parse();
    let sink = Sink::new(&tokens, events);

    Parse {
        green_node: sink.finish(),
    }
}

pub struct Parse {
    green_node: GreenNode,
}

impl Parse {
    pub fn debug_tree(&self) -> String {
        let syntax_node = SyntaxNode::new_root(self.green_node.clone());
        let formatted = format!("{:#?}", syntax_node);

        formatted[0..formatted.len() - 1].to_string()
    }
}

///////////////////////////////////
#[cfg(test)]
pub fn check(input: &str, expected_tree: expect_test::Expect) {
    let parse = parse(input);
    expected_tree.assert_eq(&parse.debug_tree());
}

