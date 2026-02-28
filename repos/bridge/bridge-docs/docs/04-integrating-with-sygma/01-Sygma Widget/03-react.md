---
id: sygma-widget-react
slug: /sygma-widget/react
title: React Framework
description: The following section provides details on the Sygma React Widget.
sidebar_position: 3
---

## Dependencies

Add the following dependencies to integrate the widget into any React project: 

```bash
yarn add @polkadot/extension-inject # required for polkadot wallet injections
```

And similarly:

```bash
yarn add @buildwithsygma/sygmaprotocol-react-widget @buildwithsygma/sygma-sdk-core
```

### Integrating The Widget Into Your React Code

After installation, add the widget into your code using:

```ts
import React from "react";
import { SygmaProtocolReactWidget } from '@buildwithsygma/sygmaprotocol-react-widget';

function MyDapp(){
  return (
    <SygmaProtocolReactWidget />
  );
}

export default MyDapp;
```

You can also pass props to the widget component to customize widget behaviour. See [Properties](05-properties.md) for more.
