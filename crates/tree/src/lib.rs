// MIT License

// Copyright (c) 2023 clonne

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use core::fmt;
use farvm_utils::{Diag, Pool,pool, Emit};

mod lexer_v1;
mod parser_v1;

#[derive(Clone,Debug)]
pub struct TokenLoc {
    row:u32,
    col:u32,
}
impl fmt::Display for TokenLoc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.row, self.col)
    }
}
impl Default for TokenLoc {
    fn default() -> Self {
        TokenLoc { row: 1, col: 0 }
    }
}

#[derive(Clone,Debug,PartialEq,Eq)]
#[allow(dead_code)]
pub enum TokenValue {
    LiteralInteger(pool::Id),
    LiteralFloat(pool::Id),
    LiteralString(String),
    Id(pool::Id),
    LP0,                    // (
    RP0,                    // )
    LP1,                    // [
    RP1,                    // ]
    LP2,                    // {
    RP2,                    // }
    LP3,                    // <
    RP3,                    // >
    ASSIGN,                 // <-
    AT,                     // @
    COLON,                  // :
    COMMA,                  // ,
    DOT,                    // .
    EQUAL,                  // =
    TILDE,                  // ~
    QUOTE1,                 // '
    QUOTE2,                 // "
    KeyAuto,                // auto
    KeyFor,                 // For
    KeyNil,                 // nil
    KeyTrue,                // true
    KeyFalse,               // false
    _EOF,
}

impl Default for TokenValue {
    fn default() -> Self {
        TokenValue::_EOF
    }
}

impl Emit for TokenValue {
    fn emit(&self, pool:&Pool) -> String {
        match self {
            TokenValue::LiteralInteger(a) => {
                format!("(Integer {})", pool.str_at(*a))
            }
            TokenValue::LiteralFloat(a) => {
                format!("(Float {})", pool.str_at(*a))
            }
            TokenValue::LiteralString(a) => {
                format!("(String \"{}\")", a)
            }
            TokenValue::Id(a) => {
                format!("(Id {})", pool.str_at(*a))
            }
            //
            TokenValue::LP0 => {format!("( LP0")}
            TokenValue::RP0 => {format!(") RP0")}
            TokenValue::LP1 => {format!("[ LP1")}
            TokenValue::RP1 => {format!("] RP1")}
            TokenValue::LP2 => {format!("{{ LP2")}
            TokenValue::RP2 => {format!("}} RP2")}
            TokenValue::LP3 => {format!("< LP3")}
            TokenValue::RP3 => {format!("> RP3")}
            TokenValue::ASSIGN => {format!("<- ASSIGN")}
            TokenValue::AT => {format!("@ AT")}
            TokenValue::COLON => {format!(": COLON")}
            TokenValue::COMMA => {format!(", COMMA")}
            TokenValue::DOT => {format!(". DOT")}
            TokenValue::EQUAL => {format!("= EQUAL")}
            TokenValue::TILDE => {format!("~ TILDE")}
            TokenValue::QUOTE1 => {format!("' QUOTE1")}
            TokenValue::QUOTE2 => {format!("\" QUOTE2")}
            //
            TokenValue::KeyAuto => {format!("auto")}
            TokenValue::KeyFor => {format!("for")}
            TokenValue::KeyNil => {format!("nil")}
            TokenValue::KeyTrue => {format!("true")}
            TokenValue::KeyFalse => {format!("false")}
            //
            TokenValue::_EOF => {format!("<!EOF!>")}
        }
    }
}

#[derive(Clone,Debug,Default)]
pub struct Token {
    loc:TokenLoc,
    val:TokenValue,
}

impl Emit for Token {
    fn emit(&self, pool:&Pool) -> String {
        format!("[{} {}]", self.loc, self.val.emit(pool))
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Node {
    Define{loc:TokenLoc,},
    _Empty,
}

impl Default for Node {
    fn default() -> Self {
        Node::_Empty
    }
}

impl Emit for Node {
    fn emit(&self, pool:&Pool) -> String {
        format!("(Node)")
    }
}

pub trait Lexer {
    fn lexer_pass(&mut self, pool:&mut Pool, diag:&mut Diag) -> Token;
}

pub trait Parser {
    fn parser_pass(&mut self, lexer:&mut impl Lexer, pool:&mut Pool, diag:&mut Diag) -> Node;
}

pub fn build_v1(code:&String, pool:&mut Pool, diag:&mut Diag) -> Node {
    let mut lex = lexer_v1::make(code);
    parser_v1::pass(&mut lex, pool, diag)
}
