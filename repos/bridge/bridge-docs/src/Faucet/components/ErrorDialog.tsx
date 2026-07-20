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
import Button from "@mui/material/Button";
import Dialog from "@mui/material/Dialog";
import DialogActions from "@mui/material/DialogActions";
import DialogContent from "@mui/material/DialogContent";
import DialogContentText from "@mui/material/DialogContentText";
import DialogTitle from "@mui/material/DialogTitle";

export default function ErrorDialog({ response, isOpen, close }) {
  return (
    <Dialog
      open={isOpen}
      onClose={close}
    >
      <DialogTitle id="alert-dialog-title" sx={{color: 'error.dark', fontWeight: 'bold'}}>
        {`Error ${response.status}`}
      </DialogTitle>
      <DialogContent>
        <DialogContentText id="alert-dialog-description">
          {response.status === 429
            ? "Too many requests. Please wait at least 1 minute"
            : response.message}
        </DialogContentText>
      </DialogContent>
      <DialogActions>
        <Button onClick={close} autoFocus>
          Close
        </Button>
      </DialogActions>
    </Dialog>
  );
}