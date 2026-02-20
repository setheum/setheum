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

use ratatui::text::Line;

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct Output {
    content: Vec<Line<'static>>,
    offset: u16,
    scrolling: bool,
    window_height: u16,
}

impl Output {
    pub fn content(&self) -> &[Line<'static>] {
        &self.content
    }

    pub fn push(&mut self, line: Line<'static>) {
        self.content.push(line)
    }

    pub fn clear(&mut self) {
        *self = Default::default();
    }

    pub fn offset(&self) -> u16 {
        self.offset
    }

    fn max_offset(&self) -> u16 {
        (self.content.len() as u16).saturating_sub(self.window_height)
    }

    pub fn note_display_height(&mut self, height: u16) {
        self.window_height = height;
        if !self.scrolling {
            self.offset = self.max_offset();
        }
    }

    pub fn reset_scrolling(&mut self) {
        self.scrolling = false;
    }

    pub fn scroll_down(&mut self) {
        if self.offset < self.max_offset() {
            self.scrolling = true;
            self.offset += 1
        }
    }

    pub fn scroll_up(&mut self) {
        if self.offset > 0 {
            self.scrolling = true;
            self.offset -= 1;
        }
    }
}
