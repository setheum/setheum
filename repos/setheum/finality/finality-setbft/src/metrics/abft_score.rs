// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم

// This file is part of Setheum.

// Copyright (C) 2019-Present Afsall Labs.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use substrate_prometheus_endpoint::{register, Gauge, PrometheusError, Registry, U64};

#[derive(Clone)]
pub enum ScoreMetrics {
    Prometheus { my_score: Gauge<U64> },
    Noop,
}

impl ScoreMetrics {
    pub fn new(registry: Option<Registry>) -> Result<Self, PrometheusError> {
        match registry {
            Some(registry) => Ok(ScoreMetrics::Prometheus {
                my_score: register(
                    Gauge::new("my_sbft_score", "My sbft score observed in last batch")?,
                    &registry,
                )?,
            }),
            None => Ok(ScoreMetrics::Noop),
        }
    }

    pub fn noop() -> Self {
        ScoreMetrics::Noop
    }

    pub fn report_score(&self, score: u16) {
        if let ScoreMetrics::Prometheus { my_score } = self {
            my_score.set(score.into());
        }
    }
}
