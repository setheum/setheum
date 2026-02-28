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