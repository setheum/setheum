import { Environment } from '@buildwithsygma/sygma-sdk-core';
import React from 'react';
import { Container } from '@mui/material';
import BrowserOnly from '@docusaurus/BrowserOnly';

function App(){
  return (
    <BrowserOnly fallback={<div>Loading...</div>}>
        {() => {
          const {SygmaProtocolReactWidget} = require('@buildwithsygma/sygmaprotocol-react-widget/build/WidgetReact'); 
          return <Container sx={{display: "flex", justifyContent: "center", alignItems: "center"}}>
              <SygmaProtocolReactWidget environment={Environment.TESTNET} />
              </Container>
        }}
    </BrowserOnly>
  )
}

export default App; 