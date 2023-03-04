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

use std::collections::btree_map::Values;

use super::{TokenLoc,TokenValue,Token,Lexer};
use farvm_utils::{Diag, Pool, pool};

pub struct Object {
    code: Vec<u8>,
    i: usize,
    row: u32,
    col: u32,
    take_loc: TokenLoc,
    take_raw: String,
    back_char: char,
}

pub fn make(code:&str) -> Object {
    Object {
        code: Vec::from(code.as_bytes()),
        i: 0,
        row: 1,
        col: 1,
        take_loc: TokenLoc::default(),
        take_raw: String::new(),
        back_char: '\0',
    }
}

enum JumpTo {
    First,
    Symbol2,
    Comment,
    Id,
    String,
    StringSlash,
    Take(TokenValue),
}

impl Object {
    fn now_are_token_loc(&mut self) -> &mut Self {
        self.take_loc = TokenLoc{ row: self.row, col: self.col };
        self
    }
    fn reset_take_raw(&mut self) -> &mut Self {
        self.take_raw.clear();
        self
    }
    fn take_raw_push(&mut self, a:char) -> &mut Self {
        self.take_raw.push(a);
        self
    }
    fn take_string(&mut self) -> TokenValue {
        TokenValue::LiteralString(self.take_raw.clone())
    }
    fn take_pool_id(&self, pool:&mut Pool) -> pool::Id {
        pool.add_str(self.take_raw.clone())
    }
    fn match_id_by_take(&self, pool:&mut Pool) -> TokenValue {
        match self.take_raw.as_str() {
            "~" => {TokenValue::TILDE}
            "auto" => {TokenValue::KeyAuto}
            "for" => {TokenValue::KeyFor}
            "nil" => {TokenValue::KeyNil}
            "true" => {TokenValue::KeyTrue}
            "false" => {TokenValue::KeyFalse}
            _ => {TokenValue::Id(self.take_pool_id(pool))}
        }
    }
    fn back_one(&mut self, a:char) -> &mut Self {
        assert!(self.take_raw.len() > 0);
        assert!(self.back_char == '\0');
        assert!(a != '\0');
        self.back_char = a;
        self
    }
    fn pop_back(&mut self) -> char {
        assert!(self.back_char != '\0');
        let a = self.back_char;
        self.back_char = '\0';
        a
    }
    fn has_back(&self) -> bool {
        self.back_char != '\0'
    }

    // Comment = (order (one '#') (any-not (one-in "\n\0")))
    fn on_comment(&self, a:char) -> JumpTo {
        match a {
            '\n' | '\0' => {JumpTo::First}
            _ => {JumpTo::Comment}
        }
    }

    // Id = (one-or-many (and (one-not-in "()[]{}<>:.,") (> ASCII 32)))
    fn on_id(&mut self, a:char, pool:&mut Pool) -> JumpTo {
        match a {
            '(' | ')' | '[' | ']' | '{' | '}' | '<' | '>' | ':' | '.' | ',' => {self.back_one(a);}
            _ => {
                if a > ' ' {
                    self.take_raw_push(a);
                    return JumpTo::Id
                }
            }
        }
        JumpTo::Take(self.match_id_by_take(pool))
    }

    // StringSlash = (order (one '\\') (or (one-in "tnr0") (one-any)))
    fn on_string_slash(&mut self, a:char) -> JumpTo {
        match a {
            't' => {self.take_raw_push('\t'); JumpTo::String}
            'n' => {self.take_raw_push('\n'); JumpTo::String}
            'r' => {self.take_raw_push('\r'); JumpTo::String}
            '0' => {self.take_raw_push('\0'); JumpTo::String}
            _ => {
                self.take_raw_push(a); JumpTo::String
            }
        }
    }

    // String = (order (one '"') (any-or StringSlash (not '"')) (one '"'))
    fn on_string(&mut self, a:char) -> JumpTo {
        match a {
            '\\' => {JumpTo::StringSlash}
            '\"' => {JumpTo::Take(self.take_string())}
            _ => {
                self.take_raw_push(a); JumpTo::String
            }
        }
    }

    // Symbol2 = (or "<-")
    fn on_symbol2(&mut self, a:char) -> JumpTo {
        match self.take_raw.as_str() {
            "<" => {
                match a {
                    '-' => {JumpTo::Take(TokenValue::ASSIGN)}
                    _ => {self.back_one(a); JumpTo::Take(TokenValue::LP3)}
                }
            }
            _ => {panic!("incorrect-logic: control-flow should can't to here")}
        }
    }

    fn on_first(&mut self, a:char) -> JumpTo {
        match a {
            '(' => {self.now_are_token_loc(); JumpTo::Take(TokenValue::LP0)}
            ')' => {self.now_are_token_loc(); JumpTo::Take(TokenValue::RP0)}
            '[' => {self.now_are_token_loc(); JumpTo::Take(TokenValue::LP1)}
            ']' => {self.now_are_token_loc(); JumpTo::Take(TokenValue::RP1)}
            '{' => {self.now_are_token_loc(); JumpTo::Take(TokenValue::LP2)}
            '}' => {self.now_are_token_loc(); JumpTo::Take(TokenValue::RP2)}
            '<' => {self.now_are_token_loc().take_raw_push(a); JumpTo::Symbol2}
            '>' => {self.now_are_token_loc(); JumpTo::Take(TokenValue::RP3)}
            '@' => {self.now_are_token_loc(); JumpTo::Take(TokenValue::AT)}
            ':' => {self.now_are_token_loc(); JumpTo::Take(TokenValue::COLON)}
            ',' => {self.now_are_token_loc(); JumpTo::Take(TokenValue::COMMA)}
            '.' => {self.now_are_token_loc(); JumpTo::Take(TokenValue::DOT)}
            '=' => {self.now_are_token_loc(); JumpTo::Take(TokenValue::EQUAL)}
            '"' => {self.now_are_token_loc(); JumpTo::String}
            '#' => {JumpTo::Comment}
            _ => {
                if a > ' ' {
                    self.now_are_token_loc().take_raw_push(a);
                    JumpTo::Id
                } else {
                    JumpTo::First
                }
            }
        }
    }

    fn jump(&mut self, a:char, now:JumpTo, pool:&mut Pool) -> JumpTo {
        match now {
            JumpTo::First => {self.on_first(a)}
            JumpTo::Symbol2 => {self.on_symbol2(a)}
            JumpTo::Comment => {self.on_comment(a)}
            JumpTo::Id => {self.on_id(a, pool)}
            JumpTo::String => {self.on_string(a)}
            JumpTo::StringSlash => {self.on_string_slash(a)}
            _ => {now}
        }
    }

    fn get_char_by_pass(&mut self) -> char {
        let a = self.code[self.i] as char;
        self.i += 1;
        self.col += 1;
        if a == '\n' {
            self.row += 1;
            self.col = 0;
        }
        a
    }
}

impl Lexer for Object {
    fn lexer_pass(&mut self, pool:&mut Pool, diag:&mut Diag) -> Token {
        let mut to = JumpTo::First;
        self.now_are_token_loc().reset_take_raw();
        if self.has_back() {
            let a = self.pop_back();
            to = self.jump(a, to, pool);
            if let JumpTo::Take(val) = to {
                return Token { loc: self.take_loc.clone(), val }
            }
        }
        while self.i < self.code.len() {
            let a = self.get_char_by_pass();
            to = self.jump(a, to, pool);
            if let JumpTo::Take(val) = to {
                return Token { loc: self.take_loc.clone(), val }
            }
        }
        if let JumpTo::Take(val) = self.jump('\0', to, pool) {
            Token { loc: self.take_loc.clone(), val }
        } else {
            Token { loc: self.take_loc.clone(), val: TokenValue::_EOF }
        }
    }
}
