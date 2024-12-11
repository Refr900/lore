use std::fmt::Debug;

use crate::lexer::{self, Kind};

mod declarations;
mod expressions;
mod statements;

pub use declarations::*;
pub use expressions::*;
pub use statements::*;

mod token_stream;
pub use token_stream::*;

mod error;
pub use error::*;

mod span;
pub use span::*;

mod ext;
pub use ext::*;

mod parser;
pub use parser::*;

pub trait Parse {
    type Parsed;
    type Error;

    fn parse(parser: &mut Parser<'_>) -> Result<Self::Parsed, Self::Error>;
}

#[test]
fn test() {
    let path = "main.rw";
    let source = std::fs::read_to_string(path).unwrap();
    let (kinds, spans) = tokenize(path, &source);
    let mut parser = Parser::new(&kinds);
    for _ in 0..1 {
        let _ = parse_with_parser::<StmtKind>(&mut parser, &spans, path, &source);
        parser.errors.clear();
    }
}

fn tokenize(path: &str, source: &str) -> (Vec<Kind>, Vec<lexer::Span>) {
    use crate::lexer::Lexer;
    let mut lexer = Lexer::new(source);
    let mut kinds = Vec::new();
    let mut spans = Vec::new();
    let mut count = 1u32;
    loop {
        let token = lexer.advance_token();
        let lexeme = &source[token.span.as_range()];
        println!("{:>5}: {:?}", count, token.kind);
        println!("     |  span: {:?}", token.span.as_range());
        println!("     |  lexeme: {:?}", lexeme);
        kinds.push(token.kind);
        spans.push(token.span);
        if matches!(token.kind, Kind::Eof) {
            break;
        }
        count += 1;
    }
    (kinds, spans)
}

fn parse_with_parser<P: Parse>(
    parser: &mut Parser,
    spans: &[lexer::Span],
    path: &str,
    source: &str,
) -> Result<P::Parsed, P::Error>
where
    P::Parsed: Debug,
    P::Error: Debug,
{
    let parsed = parser.parse::<P>();
    if parser.errors.is_empty() {
        println!("{:#?}", parsed);
    }
    print_errors(path, source, spans, &parser.errors);
    parsed
}

fn print_errors(path: &str, source: &str, spans: &[lexer::Span], errors: &[ParseError]) {
    for error in errors.iter() {
        let span = error.span();
        let start = spans[span.start.as_index()].start;
        let end = spans[span.end.as_index()].start;
        let span = lexer::Span::new(start, end);
        let location = span.location(source);
        println!("error: {}", error);
        println!("   --> {}:{}", path, location);
    }
}
