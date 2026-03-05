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