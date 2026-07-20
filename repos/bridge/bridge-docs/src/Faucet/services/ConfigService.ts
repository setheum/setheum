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

import axios from "axios";

export async function getDomains(root) {
    var resp = await axios({
        url: root + "/domains",
        method: "GET"
    })

    if(resp.status == 200) {
        return resp.data;
    } else {
        return [];
    }
}

export async function getTokens(root, domainID) {
    var resp = await axios({
        url: root + "/domains/" + domainID+ "/resources",
        method: "GET"
    })

    if(resp.status == 200) {
        return resp.data;
    } else {
        return [];
    }
}

export async function mintRequest(root, domain, resourceId, recipient){
    var resp = await axios({
        url: root + "/domains/" + domain + "/resources/" + resourceId+"/drip",
        headers: {
            'Content-Type': 'application/json'
        },
        data: {
            recipient: recipient
        }, 
        method: "POST"
    })
    if(resp.status == 200) {
        return resp.data;
    } else {
        console.log(resp.status + " mint request failed: " + resp.data)
        return {};
    }
}