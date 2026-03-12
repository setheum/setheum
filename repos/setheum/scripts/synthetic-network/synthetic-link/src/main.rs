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

use clap::{arg, Parser};
use log::info;
use synthetic_link::{SyntheticNetwork, SyntheticNetworkClient};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value = "http://Node0:80/qos")]
    url: String,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let args = Args::parse();
    let synth_net_url = args.url;

    info!("reading SyntheticNetwork configuration from stdin");
    let deserializer = serde_json::Deserializer::from_reader(std::io::stdin());
    let synth_net_config: SyntheticNetwork = deserializer
        .into_iter()
        .next()
        .unwrap_or_else(|| panic!("no configuration on stdin"))
        .unwrap_or_else(|e| panic!("unable to parse SyntheticNetwork config: {e}"));
    info!("parsed SyntheticNetwork configuration");

    info!("commiting configuration");
    let mut synth_net_client = SyntheticNetworkClient::new(synth_net_url);
    synth_net_client
        .commit_config(&synth_net_config)
        .await
        .unwrap_or_else(|e| panic!("failed to commit SyntheticNetwork configuration: {e}"));
    info!("successfully committed new configuration");
}
