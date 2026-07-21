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

import * as React from "react";
import Box from "@mui/material/Box";
import InputLabel from "@mui/material/InputLabel";
import MenuItem from "@mui/material/MenuItem";
import FormControl from "@mui/material/FormControl";
import Select from "@mui/material/Select";

export default function NetworkSelect({ networkTypes, setSelectedNetworkType }) {
  const [network, setNetwork] = React.useState("");
 
  const handleChange = (event) => {
    setSelectedNetworkType(event.target.value);
    setNetwork(event.target.value);
  };

  return (
    <Box sx={{ minWidth: 120, marginTop: 2, marginBottom: 2 }}>
      <FormControl fullWidth>
        <InputLabel id="network-select-label">Network Type</InputLabel>
        <Select
          labelId="network-type-label"
          id="network-type-select"
          value={network}
          label="Network Type"
          onChange={handleChange}
          required
          sx={{ textTransform: "capitalize" }}
        >

          {networkTypes.map((networkType) => (
            <MenuItem key={networkType} value={networkType}>{networkType}</MenuItem>
          ))}
        </Select>
      </FormControl>
    </Box>
  );
}