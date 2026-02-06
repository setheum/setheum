![license](https://img.shields.io/badge/License-Apache%202.0-blue?logo=apache&style=flat-square)
[![npm](https://img.shields.io/npm/v/@setheum.js/api?logo=npm&style=flat-square)](https://www.npmjs.com/package/@setheum.js/api)

# @setheum.js
Promise and RxJS APIs around Setheum RPC calls. 
This library provides additional typing for users to access Setheum networks by using [setheum.js](https://github.com/setheum-js/api) using [polkadot.js](https://github.com/polkadot-js/api).

# Getting Started

More documentation and examples on [setheum.js.org](https://setheum.js.org).

1. Install dependencies

```bash
yarn add @polkadot/api @setheum.js/api
```

2. Create API instance

```ts
import { ApiPromise } from '@polkadot/api';
import { WsProvider } from '@polkadot/rpc-provider';
import { options } from '@setheum.js/api';

async function main() {
    const provider = new WsProvider('wss://api.setheum.xyz');
    const api = new ApiPromise(options({ provider }));
    await api.isReady;

    // use api
}

main()
```

3. Use api to interact with node

```ts
// query and display account data
const data = await api.query.system.account('5F98oWfz2r5rcRVnP9VCndg33DAAsky3iuoBSpaPUbgN9AJn');
console.log(data.toHuman())
```

## Types

- Use Setheum types

```ts
import {setheumDefs} from '@setheum-js/type-definitions';

// Define FileInfo
export type FileInfo = typeof setheumDefs.market.types.FileInfo;

// Use FileInfo as `interface`
```

## Packages Overview

The API is split up into a number of internal packages -

- [@setheum.js/api](packages/api/) The API library, providing both Promise and RxJS Observable-based interfaces. This is the main user-facing entry point.
- [@setheum.js/api-derive](packages/api-derive/) Additional polkadot.js derives for Setheum Network.
Derived results that are injected into the API, allowing for combinations of various query results (only used internally and exposed on the Api instances via `api.derive.*`).
- [@setheum.js/app-util](./packages/app-util)
  - Utilities to work with Setheum.
- [@setheum.js/types](packages/types/) Codecs for all Polkadot.js type definations for Setheum primitives.

## Contribution
  
  Please send a PR(Pull Request) to contribute this repo and read the following rules:

  1. **No `--force` pushes** or modifying the master branch history in any way. If you need to rebase, ensure you do it in your own repo.
  2. **Non-main branches**, prefixed with a short name moniker (e.g. zik/my-feature) must be used for ongoing work.
  3. **All modifications** must be made in **pull-request** to solicit feedback from other contributors.
  4. A pull-request **must not be merged until CI** has finished successfully.
  5. Contributors should adhere to the [Google Typescript Style Guide](https://github.com/google/gts).
