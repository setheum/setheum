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