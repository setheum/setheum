import * as React from 'react';
import Box from '@mui/material/Box';
import InputLabel from '@mui/material/InputLabel';
import MenuItem from '@mui/material/MenuItem';
import FormControl from '@mui/material/FormControl';
import Select from '@mui/material/Select';

export default function TokenSelect({tokenArray, setSelectedToken, disabled, selectedToken}) {

  const handleChange = (event) => {
    setSelectedToken(event.target.value);
  };

  return (
    <Box sx={{ minWidth: 120, marginTop: 2, marginBottom: 2}}>
      <FormControl fullWidth>
        <InputLabel id="token-select-label">Token</InputLabel>
        <Select
          labelId="token-select-label"
          id="token-select"
          value={selectedToken || ""}
          label="Token"
          onChange={handleChange}
          disabled={disabled}
          required
        >
          {tokenArray.map(token => (
            <MenuItem id={token.address} key={token.address} value={token}>{token.symbol}</MenuItem>
          ))}
        </Select>
      </FormControl>
    </Box>
  );
}