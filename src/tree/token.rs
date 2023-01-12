use core::fmt;
use super::{TokenLoc,TokenValue,Token,Error};

impl fmt::Display for TokenLoc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.row, self.col)
    }
}
impl Default for TokenLoc {
    fn default() -> Self {
        TokenLoc { row:0, col: 0 }
    }
}

impl fmt::Display for TokenValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenValue::ConstInteger(a) => {
                write!(f, "(Integer {})", a)
            }
            TokenValue::ConstFloat(a) => {
                write!(f, "(Float {})", a)
            }
            TokenValue::ConstString(a) => {
                write!(f, "(String \"{}\")", a)
            }
            TokenValue::Id(a) => {
                write!(f, "(Id {})", a)
            }
            //
            TokenValue::LP0 => {write!(f, "'(' LP0")}
            TokenValue::RP0 => {write!(f, "')' RP0")}
            TokenValue::LP1 => {write!(f, "'[' LP1")}
            TokenValue::RP1 => {write!(f, "']' RP1")}
            TokenValue::LP2 => {write!(f, "'{{' LP2")}
            TokenValue::RP2 => {write!(f, "'}}' RP2")}
            TokenValue::LP3 => {write!(f, "'<' LP3")}
            TokenValue::RP3 => {write!(f, "'>' RP3")}
            TokenValue::COLON => {write!(f, "':' COLON")}
            TokenValue::COMMA => {write!(f, "',' COMMA")}
            TokenValue::DOT => {write!(f, "'.' DOT")}
            //
            TokenValue::KeyNil => {write!(f, "nil")}
            TokenValue::KeyTrue => {write!(f, "true")}
            TokenValue::KeyFalse => {write!(f, "false")}
            //
            TokenValue::_Eof => {write!(f, "<!EOF!>")}
        }
    }
}
impl Default for TokenValue {
    fn default() -> Self {
        TokenValue::_Eof
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{} {}]", self.loc, self.val)
    }
}

pub fn text(tokens:&Vec<Token>) -> String {
    let a = tokens.iter().map(|a|a.to_string());
    let b = a.collect::<Vec<String>>().join("\n");
    b
}

pub fn build(code:&String, _:&mut Vec<Error>) -> Vec<Token> {
    #[derive(Debug,Default)]
    struct ParseStatus {
        val:TokenValue,
        takes:String,
        loc_unit:TokenLoc,
        loc:TokenLoc,
    }

    enum P {
        First = 0,
        Id,
        String,
        Integer,Float,
        Minus,              // (or <Id> <Number>)
        Comment,
        _Take = 0xFFFE,
        _TakeToFirst,
    }

    fn p_first(a:char, ps:&mut ParseStatus) -> usize {
        ps.val = TokenValue::ConstString(String::from(a));
        match a {
            '\0'..=' ' => {
                P::First as usize
            }
            '0' ..='9' => {
                ps.takes.push(a); ps.loc_unit = ps.loc.clone(); P::Integer as usize
            }
            '-' => {
                ps.takes.push(a); ps.loc_unit = ps.loc.clone(); P::Minus as usize
            }
            '(' => {
                ps.val = TokenValue::LP0; ps.takes = String::from(a); ps.loc_unit = ps.loc.clone(); P::_TakeToFirst as usize
            }
            ')' => {
                ps.val = TokenValue::RP0; ps.takes = String::from(a); ps.loc_unit = ps.loc.clone(); P::_TakeToFirst as usize
            }
            '[' => {
                ps.val = TokenValue::LP1; ps.takes = String::from(a); ps.loc_unit = ps.loc.clone(); P::_TakeToFirst as usize
            }
            ']' => {
                ps.val = TokenValue::RP1; ps.takes = String::from(a); ps.loc_unit = ps.loc.clone(); P::_TakeToFirst as usize
            }
            '{' => {
                ps.val = TokenValue::LP2; ps.takes = String::from(a); ps.loc_unit = ps.loc.clone(); P::_TakeToFirst as usize
            }
            '}' => {
                ps.val = TokenValue::RP2; ps.takes = String::from(a); ps.loc_unit = ps.loc.clone(); P::_TakeToFirst as usize
            }
            '<' => {
                ps.val = TokenValue::LP3; ps.takes = String::from(a); ps.loc_unit = ps.loc.clone(); P::_TakeToFirst as usize
            }
            '>' => {
                ps.val = TokenValue::RP3; ps.takes = String::from(a); ps.loc_unit = ps.loc.clone(); P::_TakeToFirst as usize
            }
            ':' => {
                ps.val = TokenValue::COLON; ps.takes = String::from(a); ps.loc_unit = ps.loc.clone(); P::_TakeToFirst as usize
            }
            ',' => {
                ps.val = TokenValue::COMMA; ps.takes = String::from(a); ps.loc_unit = ps.loc.clone(); P::_TakeToFirst as usize
            }
            '.' => {
                ps.val = TokenValue::DOT; ps.takes = String::from(a); ps.loc_unit = ps.loc.clone(); P::_TakeToFirst as usize
            }
            '"' => {
                ps.loc_unit = ps.loc.clone(); P::String as usize
            }
            '#' => {
                P::Comment as usize
            }
            _ => {
                ps.takes.push(a); ps.loc_unit = ps.loc.clone(); P::Id as usize
            }
        }
    }

    fn p_id(a:char, ps:&mut ParseStatus) -> usize {
        match a {
            '\0'..=' ' | '(' | ')' | '[' | ']' | '{' | '}' | '<' | '>' | ':' | ',' | '.' | '"' | '#' => {
                ps.val = match ps.takes.as_str() {
                    "nil" => {TokenValue::KeyNil}
                    "true" => {TokenValue::KeyTrue}
                    "false" => {TokenValue::KeyFalse}
                    _ => {TokenValue::Id(ps.takes.clone())}
                };
                P::_Take as usize
            }
            _ => {
                ps.takes.push(a); P::Id as usize
            }
        }
    }

    fn p_string(a:char, ps:&mut ParseStatus) -> usize {
        match a {
            '\0' | '"' => {
                ps.val = TokenValue::ConstString(ps.takes.clone()); P::_TakeToFirst as usize
            }
            _ => {
                ps.takes.push(a); P::String as usize
            }
        }
    }

    fn p_integer(a:char, ps:&mut ParseStatus) -> usize {
        match a {
            '0'..='9' => {
                ps.takes.push(a); P::Integer as usize
            }
            '.' => {
                ps.takes.push(a); P::Float as usize
            }
            _ => {
                ps.val = TokenValue::ConstInteger(ps.takes.clone()); P::_Take as usize
            }
        }
    }

    fn p_float(a:char, ps:&mut ParseStatus) -> usize {
        match a {
            '0'..='9' => {
                ps.takes.push(a); P::Float as usize
            }
            _ => {
                ps.val = TokenValue::ConstFloat(ps.takes.clone()); P::_Take as usize
            }
        }
    }

    fn p_minus(a:char, ps:&mut ParseStatus) -> usize {
        ps.takes.push(a);
        match a {
            '0'..='9' => {P::Integer as usize}
            _ => {P::Id as usize}
        }
    }

    fn p_comment(a:char, _:&mut ParseStatus) -> usize {
        match a {
            '\n' => {
                P::First as usize
            }
            _ => {
                P::Comment as usize
            }
        }
    }

    let pmaps = [
        p_first, p_id, p_string, p_integer, p_float, p_minus, p_comment,
    ];

    let mut tokens:Vec<Token> = vec!();

    let mut ps = ParseStatus{
        val: TokenValue::default(),
        takes: String::new(),
        loc_unit: TokenLoc::default(),
        loc: TokenLoc{row: 1, col: 0},
    };
    let mut pcur:usize = 0;
    for a in code.chars() {
        match a {
            '\n' => {
                ps.loc.row += 1;
                ps.loc.col = 0;
            }
            _ => {
                ps.loc.col += 1;
            }
        }
        pcur = pmaps[pcur](a, &mut ps);
        while pcur >= pmaps.len() {
            tokens.push(Token{loc: ps.loc_unit.clone(), val: ps.val});
            ps.val = TokenValue::_Eof;
            ps.takes.clear();
            if pcur == P::_TakeToFirst as usize {
                pcur = P::First as usize
            } else {
                pcur = p_first(a, &mut ps);
            }
        }
    }
    if pmaps[pcur]('\0', &mut ps) >= P::_Take as usize {
        tokens.push(Token{loc: ps.loc_unit.clone(), val: ps.val})
    }
    tokens.push(Token{loc: ps.loc.clone(), val: TokenValue::_Eof});
    tokens
}
