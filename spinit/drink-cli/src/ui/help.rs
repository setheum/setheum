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

use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use crate::{app_state::AppState, ui::layout::section};

pub(super) fn build(_app_state: &AppState) -> impl Widget {
    Paragraph::new(vec![
        command("cd <dir>", "change directory do <dir>"),
        command("clear / c", "clear output tab"),
        command(
            "build / b",
            "build contract from the sources in the current directory",
        ),
        command(
            "deploy / d [--constructor <name>] [--salt <salt>]",
            "deploy contract using <constructor> (`new` by default) and <salt> (empty by default)",
        ),
        command("call <message>", "call contract's message"),
        command(
            "next-block / nb [count]",
            "build next <count> blocks (by default a single block)",
        ),
        command(
            "add-tokens <recipient> <value>",
            "add <value> tokens to <recipient>",
        ),
        command(
            "set-actor <account>",
            "set <account> as the current actor (transaction sender)",
        ),
        command(
            "set-gas-limit <ref_time> <proof_size>",
            "set gas limits to <ref_time> and <proof_size>",
        ),
    ])
    .block(section("Help"))
}

fn command(command: &'static str, description: &'static str) -> Line<'static> {
    Line::from(vec![
        Span::styled(command, Style::default().fg(Color::Green)),
        format!(": {description}").into(),
    ])
}
