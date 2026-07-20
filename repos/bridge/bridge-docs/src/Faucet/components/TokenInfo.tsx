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
import FormControl from "@mui/material/FormControl";
import TextField from "@mui/material/TextField";
import Grid from "@mui/material/Grid";
import { capName } from "@site/src/utils";
import { Network } from "@buildwithsygma/sygma-sdk-core";

export default function TokenInfo({ tokenInfo, domainType }) {
  return (
    <>
      <Grid item xs={12} sm={12}>
        <FormControl fullWidth>
          <TextField
            disabled
            id="contract-address"
            label={domainType === Network.EVM ? "Contract address" : "Sender address"}
            value={tokenInfo.address}
          ></TextField>
        </FormControl>
      </Grid>
      <Grid item xs={12} sm={6}>
        <FormControl fullWidth>
          <TextField
            disabled
            id="token-type"
            label="Type"
            value={domainType === Network.EVM ? tokenInfo.type.toUpperCase() : capName(tokenInfo.type)}
          ></TextField>
        </FormControl>
      </Grid>
      <Grid item xs={12} sm={6}>
        <FormControl fullWidth>
          <TextField
            disabled
            id="mint-amount"
            label="Amount"
            value={tokenInfo.amount}
          />
        </FormControl>
      </Grid>
    </>
  );
}