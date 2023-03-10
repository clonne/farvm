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

use std::collections::HashMap;

pub type Id = u32;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Object {
    str_index: HashMap<String, Id>,
    str_heap: Vec<String>,
}

impl Object {
    pub fn new() -> Object {
        Object {
            str_index: HashMap::new(),
            str_heap: Vec::new(),
        }
    }

    pub fn add_str(&mut self, value:impl ToString) -> Id {
        let value = value.to_string();
        match self.str_index.get(&value) {
            Some(&i) => {i}
            None => {
                let id = self.str_heap.len() as Id;
                self.str_heap.push(value.clone());
                self.str_index.insert(value, id);
                id
            }
        }
    }

    pub fn str_at(&self, id:Id) -> String {
        let id = id as usize;
        if id < self.str_heap.len() {
            self.str_heap[id].clone()
        } else {
            String::new()
        }
    }
}

pub fn make() -> Object { Object::new() }
