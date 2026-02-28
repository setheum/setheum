import * as React from 'react';
import Box from '@mui/material/Box';
import InputLabel from '@mui/material/InputLabel';
import MenuItem from '@mui/material/MenuItem';
import FormControl from '@mui/material/FormControl';
import Select from '@mui/material/Select';
import  TextField  from '@mui/material/TextField';

export default function DomainSelect({ setToAddress }) {

    const [domain, setDomain] = React.useState('');

    const handleChange = (event) => {
        setDomain(event.target.value);
        setToAddress(event.target.value);
    };

    return (
        <Box sx={{ minWidth: 120 }}>
            <FormControl fullWidth>
                <TextField id="filled-basic" label="Destination address" variant="filled" required />
            </FormControl>
        </Box>
    );
}