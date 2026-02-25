// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم

// This file is part of Setheum.

// Copyright (C) 2019-Present Setheum Developers.
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

use std::error::Error;

use substrate_prometheus_endpoint::{
    register, Gauge, Histogram, HistogramOpts, PrometheusError, Registry, U64,
};

use crate::{BlockId, BlockNumber, SubstrateChainStatus};

#[derive(Clone)]
pub enum BestBlockMetrics {
    Prometheus {
        top_finalized_block: Gauge<U64>,
        best_block: Gauge<U64>,
        reorgs: Histogram,
        best_block_id: BlockId,
        chain_status: SubstrateChainStatus,
    },
    Noop,
}

impl BestBlockMetrics {
    pub fn new(
        registry: Option<Registry>,
        chain_status: SubstrateChainStatus,
    ) -> Result<Self, PrometheusError> {
        let registry = match registry {
            Some(registry) => registry,
            None => return Ok(Self::Noop),
        };

        Ok(Self::Prometheus {
            top_finalized_block: register(
                Gauge::new("setbft_top_finalized_block", "Top finalized block number")?,
                &registry,
            )?,
            best_block: register(
                Gauge::new(
                    "setbft_best_block",
                    "Best (or more precisely, favourite) block number",
                )?,
                &registry,
            )?,
            reorgs: register(
                Histogram::with_opts(
                    HistogramOpts::new("setbft_reorgs", "Number of reorgs by length")
                        .buckets(vec![1., 2., 4., 9.]),
                )?,
                &registry,
            )?,
            best_block_id: (Default::default(), 0u32).into(),
            chain_status,
        })
    }

    pub fn report_best_block_imported(&mut self, block_id: BlockId) {
        if let Self::Prometheus {
            best_block,
            ref mut best_block_id,
            reorgs,
            chain_status,
            ..
        } = self
        {
            let reorg_len = retracted_path_length(chain_status, best_block_id, &block_id);
            best_block.set(block_id.number() as u64);
            *best_block_id = block_id;
            match reorg_len {
                Ok(0) => {}
                Ok(reorg_len) => {
                    reorgs.observe(reorg_len as f64);
                }
                Err(e) => {
                    log::warn!("Failed to calculate reorg length: {:?}", e);
                }
            }
        }
    }

    pub fn report_block_finalized(&self, block_id: BlockId) {
        if let Self::Prometheus {
            top_finalized_block,
            ..
        } = self
        {
            top_finalized_block.set(block_id.number() as u64);
        }
    }
}

fn retracted_path_length(
    chain_status: &SubstrateChainStatus,
    from: &BlockId,
    to: &BlockId,
) -> Result<BlockNumber, Box<dyn Error>> {
    let lca = chain_status
        .lowest_common_ancestor(from, to)
        .map_err(Box::new)?;
    Ok(from.number().saturating_sub(lca.number()))
}
