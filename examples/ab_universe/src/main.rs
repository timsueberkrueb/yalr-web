use yalr::*;

mod abc;
use crate::abc::*;

#[lalr(start = "Start")]
#[terminal_type(ABC)]
impl ABParser {
    #[rule(Start -> A B)]
    fn start_a_b(_a: &[ABC], _b: &[ABC]) -> String {
        "ab".to_owned()
    }

    #[rule(Start -> A Start B)]
    fn start_a_inner_b(_a: &[ABC], inner: String, _b: &[ABC]) -> String {
        "a".to_owned() + &inner + "b"
    }
}

fn main() {
    use self::ABC::*;
    let input = vec![A, A, A, B, B, B, End];
    let mut lexer = ABLexer::new(&input[..]);
    let result = ABParser::parse(&mut lexer);
    println!("{:?}", result);
}
