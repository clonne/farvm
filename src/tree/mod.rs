pub mod token;
pub mod node;

#[derive(Clone,Debug)]
pub struct TokenLoc {
    row:i32,
    col:i32,
}
#[derive(Clone,Debug)]
#[allow(dead_code)]
pub enum TokenValue {
    ConstInteger(String),
    ConstFloat(String),
    ConstString(String),
    Id(String),
    LP0,                    // (
    RP0,                    // )
    LP1,                    // [
    RP1,                    // ]
    LP2,                    // {
    RP2,                    // }
    LP3,                    // <
    RP3,                    // >
    COLON,                  // :
    COMMA,                  // ,
    DOT,                    // .
    KeyNil,                 // nil
    KeyTrue,                // true
    KeyFalse,               // false
    _Eof,
}
#[derive(Clone,Debug,Default)]
pub struct Token {
    loc:TokenLoc,
    val:TokenValue,
}

#[derive(Clone,Debug,Default)]
pub struct Error {
    loc:TokenLoc,
    txt:String,
}

#[derive(Debug)]
pub enum Node {
    Empty,
}

pub fn build(code:&String) -> (Node,Vec<Error>) {
    let mut errors:Vec<Error> = Vec::new();
    let tokens = token::build(code, &mut errors);
    if errors.len() > 0 {
        (Node::Empty, errors)
    } else {
        (node::build(&tokens, &mut errors), errors)
    }
}
