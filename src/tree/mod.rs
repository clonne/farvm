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
    ConstBool(bool),
    Id(String),
    LP0,                    // (
    RP0,                    // )
    LP1,                    // [
    RP1,                    // ]
    LP2,                    // {
    RP2,                    // }
    COLON,                  // :
    COMMA,                  // ,
    DOT,                    // .
    KeyNil,
    Eof,
}
#[derive(Clone,Debug,Default)]
pub struct Token {
    loc:TokenLoc,
    val:TokenValue,
}

#[derive(Debug)]
pub enum Node {
    Empty,
}

pub fn build(code:&String) -> Node {
    Node::Empty
}
