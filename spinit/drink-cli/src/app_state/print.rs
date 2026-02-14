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

use drink::pallet_revive::ContractResult;
use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
};

use crate::app_state::AppState;

impl AppState {
    pub fn print_command(&mut self, command: &str) {
        self.ui_state.output.push("".into());
        self.ui_state.output.push(
            Span::styled(
                format!("Executing `{command}`"),
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::ITALIC),
            )
            .into(),
        );
    }

    pub fn print(&mut self, msg: &str) {
        self.print_sequence(
            msg.split('\n'),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        );
    }

    pub fn print_error(&mut self, err: &str) {
        self.print_sequence(
            err.split('\n'),
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        );
    }

    fn print_sequence<'a, I: Iterator<Item = &'a str>>(&mut self, seq: I, style: Style) {
        for line in seq {
            self.ui_state
                .output
                .push(Span::styled(line.to_string(), style).into());
        }
    }
}

pub fn format_contract_action<R>(result: &ContractResult<R, u128>) -> String {
    format!(
        "Gas consumed: {:?}\nGas required: {:?}\n",
        result.gas_consumed, result.gas_required
    )
}
