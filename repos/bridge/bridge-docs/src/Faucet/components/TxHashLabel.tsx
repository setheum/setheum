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
import FormControl from "@mui/material/FormControl";
import TextField from "@mui/material/TextField";
import Alert from "@mui/material/Alert";

export default function TxHashLabel({ txHash }) {
  return (
    <Box sx={{ minWidth: 120, marginTop: 2, marginBottom: 2 }}>
      <Alert severity="success">
        The tokens were successfully sent to your address
      </Alert>
      <FormControl fullWidth color="success">
        <TextField
          color="success"
          variant="filled"
          id="tx-hash"
          label="Transaction hash"
          defaultValue={txHash}
          focused
          autoFocus
        ></TextField>
      </FormControl>
    </Box>
  );
}