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

#[derive(Debug,Default)]
#[allow(dead_code)]
pub struct DiagUnit {
    row: u32,
    col: u32,
    describe: String,
}
impl DiagUnit {
    pub fn row(&self) -> u32 {self.row}
    pub fn col(&self) -> u32 {self.col}
    pub fn describe(&self) -> String {self.describe.clone()}
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Object {
    warnings: Vec<DiagUnit>,
    errors: Vec<DiagUnit>,
}

impl Object {
    pub fn new() -> Object {
        Object { warnings: Vec::new(), errors: Vec::new() }
    }
    pub fn has_error(&self) -> bool {self.errors.len() > 0}
    pub fn push_error(&mut self, row:u32, col:u32, describe:&str) {
        self.errors.push(DiagUnit { row, col, describe: String::from(describe) })
    }
}

pub fn make() -> Object { Object::new() }

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", "ok")
    }
}
