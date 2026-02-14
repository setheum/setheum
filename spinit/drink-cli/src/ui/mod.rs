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

mod contracts;
mod current_env;
mod footer;
mod help;
mod layout;
mod output;
mod user_input;

use std::{io, io::Stdout, path::PathBuf};

use anyhow::{anyhow, Result};
use crossterm::{
    event,
    event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use layout::layout;
use ratatui::backend::CrosstermBackend;

use crate::{
    app_state::{
        AppState,
        Mode::{Drinking, Managing},
    },
    executor::execute,
};

type Terminal = ratatui::Terminal<CrosstermBackend<Stdout>>;

pub fn run_ui(cwd: Option<PathBuf>) -> Result<()> {
    let mut terminal = setup_dedicated_terminal()?;
    let app_result = run_ui_app(&mut terminal, cwd);
    restore_original_terminal(terminal)?;
    app_result
}

fn setup_dedicated_terminal() -> Result<Terminal> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend).map_err(|e| anyhow!(e))
}

fn restore_original_terminal(mut terminal: Terminal) -> Result<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor().map_err(|e| anyhow!(e))
}

fn run_ui_app(terminal: &mut Terminal, cwd_override: Option<PathBuf>) -> Result<()> {
    let mut app_state = AppState::new(cwd_override);

    loop {
        terminal.draw(|f| layout(f, &mut app_state))?;

        let mode = &mut app_state.ui_state.mode;
        if let Event::Key(key) = event::read()? {
            match (*mode, key.code) {
                (_, KeyCode::Esc) => *mode = Managing,

                (Managing, KeyCode::Char('q')) => break,
                (Managing, KeyCode::Char('i')) => {
                    *mode = Drinking;
                    app_state.ui_state.show_help = false;
                }
                (Managing, KeyCode::Char('h')) => {
                    app_state.ui_state.show_help = !app_state.ui_state.show_help
                }
                (Managing, KeyCode::Down) => app_state.ui_state.output.scroll_down(),
                (Managing, KeyCode::Up) => app_state.ui_state.output.scroll_up(),

                (Drinking, KeyCode::Char(c)) => app_state.ui_state.user_input.push(c),
                (Drinking, KeyCode::Backspace) => {
                    app_state.ui_state.user_input.pop();
                }
                (Drinking, KeyCode::Tab) => {
                    let prev_path = match app_state.contracts.current_contract() {
                        Some(c) => c.base_path.clone(),
                        None => continue,
                    };

                    let new_path = &app_state
                        .contracts
                        .next()
                        .expect("There is at least one contract - just checked")
                        .base_path;

                    if *new_path != prev_path {
                        let base_path = new_path.to_str().unwrap();
                        app_state.ui_state.user_input.set(format!("cd {base_path}"));
                        execute(&mut app_state)?;
                        app_state.ui_state.user_input.set(String::new());
                    }
                }
                (Drinking, KeyCode::Up) => app_state.ui_state.user_input.prev_input(),
                (Drinking, KeyCode::Down) => app_state.ui_state.user_input.next_input(),
                (Drinking, KeyCode::Enter) => {
                    execute(&mut app_state)?;
                    app_state.ui_state.user_input.apply();
                    app_state.ui_state.output.reset_scrolling();
                }

                _ => {}
            }
        }
    }
    Ok(())
}
