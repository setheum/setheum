// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of Setheum.

// Copyright (C) 2019-Present Setheum Developers.
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Alternatively, this file is available under the MIT License:
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
enum Position {
    #[default]
    Fresh,
    History(usize),
}

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct UserInput {
    history: Vec<String>,
    position: Position,
    current_input: String,
}

impl UserInput {
    pub fn push(&mut self, c: char) {
        self.current_input.push(c);
    }

    pub fn pop(&mut self) {
        self.current_input.pop();
    }

    pub fn set(&mut self, s: String) {
        self.current_input = s;
        self.position = Position::Fresh;
    }

    pub fn prev_input(&mut self) {
        match self.position {
            Position::Fresh if self.history.is_empty() => {}
            Position::Fresh => {
                self.position = Position::History(self.history.len() - 1);
                self.current_input = self.history[self.history.len() - 1].clone();
            }
            Position::History(0) => {}
            Position::History(n) => {
                self.position = Position::History(n - 1);
                self.current_input = self.history[n - 1].clone();
            }
        }
    }

    pub fn next_input(&mut self) {
        match self.position {
            Position::Fresh => {}
            Position::History(n) if n == self.history.len() - 1 => {
                self.position = Position::Fresh;
                self.current_input.clear();
            }
            Position::History(n) => {
                self.position = Position::History(n + 1);
                self.current_input = self.history[n + 1].clone();
            }
        }
    }

    pub fn apply(&mut self) {
        if !self.current_input.is_empty()
            && self.history.last().cloned().unwrap_or_default() != self.current_input
        {
            self.history.push(self.current_input.clone());
        }
        self.current_input.clear();
        self.position = Position::Fresh;
    }

    pub fn current_input(&self) -> &str {
        &self.current_input
    }
}
