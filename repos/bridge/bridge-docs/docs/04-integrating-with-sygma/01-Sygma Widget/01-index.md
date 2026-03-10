---
id: sygma-widget-index
slug: /sygma-widget
title: Introduction To The Sygma Widget
description: The following section provides details on the Sygma Widget.
sidebar_position: 1
---

The Sygma protocol provides an open source, customizable frontend UI, the **Sygma Widget**, to make integration of the Sygma interoperability protocol easier.

The source code can be found in the [sygma-widget repo](https://github.com/sygmaprotocol/sygma-widget).

The repository is organized into two packages and one example:
- The [Widget](https://github.com/sygmaprotocol/sygma-widget/tree/main/packages/widget) package is a web component built using the Lit framework.
- The [React](https://github.com/sygmaprotocol/sygma-widget/tree/main/packages/react) package is a wrapper around the Lit widget that allows developers to use this application inside React projects. 
- The [examples/react-widget-app] package is an example of a stylized React widget.

## Quick Setup

1. Create a Vite template project called `my-dapp`:
   
```bash
yarn create vite my-dapp --template react-ts
cd ./my-dapp
touch yarn.lock
```

2. Install dependencies: 

```bash
yarn install
yarn add @buildwithsygma/sygmaprotocol-react-widget @buildwithsygma/sygma-sdk-core
``` 

3. Boot up the local server: 

```bash
yarn dev
```

## How To Integrate

Follow the [Lit](02-lit.md) or [React](03-react.md) documentation for framework-specific instructions.