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