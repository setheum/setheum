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

const argv = require('node:process').argv;
const inLatency = argv.length <= 2 ? 0 : argv.at(2);
const outLatency = argv.length <= 3 ? 0 : argv.at(3);
console.log("setting in-latency to", inLatency);
console.log("setting out-latency to", outLatency);

const SyntheticNetwork = require('../vendor/synthetic-network/frontend');

async function setLatency(host, port, inLatency, outLatency) {
    const synthnet = new SyntheticNetwork({ hostname: host, port: port });
    synthnet.default_link.egress.latency(outLatency);
    synthnet.default_link.ingress.latency(inLatency);
    await synthnet.commit();
}

async function run(inLatency, outLatency) {
    for (let it = 0; it < 5; it++) {
        await setLatency('localhost', 3000 + it, inLatency, outLatency);
    }
}

run(inLatency, outLatency);
