---
id: sygma-props
slug: /sygma-widget/props
title: Properties
description: The following section provides details on utilizing widget properties
sidebar_position: 5
---

# Widget Properties

The behaviour of the Sygma Widget can be customized using configurable properties (props). A complete reference of the properties can be found under [packages/widget/src/widget.ts](https://github.com/sygmaprotocol/sygma-widget/blob/main/packages/widget/src/widget.ts).

:::info
Note that some properties require installing additional dependencies.
:::

The currently available properties are:
- **environment**
  - Desc: Determines whether the widget operates on the live network (mainnet) or a testing environment (testnet). 
  - Usage: 
      1. Install `@polkadot/types` dependency: `yarn add -D @polkadot/types` 
      2. Configure either MAINNET or TESTNET environment inside the SygmaProtocolReactWidget: `<SygmaProtocolReactWidget environment={Environment.MAINNET} />`
- **whitelistedSourceNetworks**
  - Desc: Specifies which blockchain networks can be selected as the source for transactions.
  - Usage: `<SygmaProtocolReactWidget whitelistedSourceNetworks={["sepolia"]} />`
- **whitelistedDestinationNetworks**
  - Desc: Specifies which blockchain networks can be selected as the destination for transactions.
  - Usage: `<SygmaProtocolReactWidget whitelistedDestinationNetworks={["cronos"]} />`
- **whitelistedSourceResources**
  - Desc: Defines which assets or resources (e.g., tokens) can be selected for transaction.
  - Usage: `<SygmaProtocolReactWidget whitelistedSourceResources={["resourceID1", "resourceID2"]} />`
- **substrateProviders**
  - Desc: Specifies the API endpoints or connection details for Substrate-based blockchain networks.
  - Usage:
    1. Install `@polkadot/api` dependency: `yarn add @polkadot/api`
    2. Add import statements to the top of the app: `import { ApiPromise, WsProvider } from '@polkadot/api';`
    3. Setup a `setState` action: `const [substrateProviders, setSubstrateProviders] = useState<ApiPromise[]>([]);`
    4. Initialize the Substrate provider:
      ```ts
        useEffect(() => {
          const provider = new WsProvider('[YOUR_SUBSTRATE_WSS_PROVIDER_HERE]');
          const api = new ApiPromise({ provider });
          api.isReady.then(() => {
            console.log('API is ready');
            setSubstrateProviders([api]);
          });
        }, []);
      ```
    5. Pass the provider into the component:
      ```ts
        <SygmaProtocolReactWidget
          substrateProviders={substrateProviders}
        />
      ```

## Using Props In An Example

Below you will find an example of props being passed to whitelist (i.e enable) the source and destination networks in the React component: 

```ts
// App.tsx
import React from "react";
import { SygmaProtocolReactWidget } from "@buildwithsygma/sygmaprotocol-react-widget";
import { Environment } from '@buildwithsygma/sygma-sdk-core';

function MyDapp() {
  const [count, setCount] = useState(0);

  return (
    <SygmaProtocolReactWidget
      whitelistedSourceNetworks={["sepolia"]}
      whitelistedDestinationNetworks={["cronos"]}
    />
  );
}
```
