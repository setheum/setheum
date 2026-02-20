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
    backend::Backend,
    layout::{Constraint, Direction, Layout, Margin},
    widgets::{Block, BorderType, Borders, Padding},
    Frame,
};

use crate::{
    app_state::AppState,
    ui::{contracts, current_env, footer, help, output, user_input},
};

pub(super) fn section(title: &str) -> Block {
    Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::horizontal(1))
}

pub(super) fn layout<B: Backend>(f: &mut Frame<B>, app_state: &mut AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            // current env
            Constraint::Ratio(4, 20),
            // output / help
            Constraint::Ratio(12, 20),
            // user input
            Constraint::Length(3),
            // footer
            Constraint::Ratio(2, 20),
        ])
        .split(f.size());

    let subchunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(chunks[0].inner(&Margin {
            horizontal: 0,
            vertical: 0,
        }));
    f.render_widget(current_env::build(app_state), subchunks[0]);
    f.render_widget(contracts::build(app_state), subchunks[1]);

    if app_state.ui_state.show_help {
        f.render_widget(help::build(app_state), chunks[1]);
    } else {
        app_state
            .ui_state
            .output
            .note_display_height(chunks[1].height - 2);
        f.render_widget(output::build(app_state), chunks[1]);
    }

    f.render_widget(user_input::build(app_state), chunks[2]);
    f.render_widget(footer::build(app_state), chunks[3]);
}
