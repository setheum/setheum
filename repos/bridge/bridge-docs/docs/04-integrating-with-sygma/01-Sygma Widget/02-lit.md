---
id: sygma-widget-lit
slug: /sygma-widget/lit
title: Lit Framework
description: The following section provides details on the Sygma Lit Widget.
sidebar_position: 2
---

## Dependencies

Add the following dependencies to integrate the widget into any Lit project: 

```bash
yarn add @buildwithsygma/sygmaprotocol-widget @buildwithsygma/sygma-sdk-core
```

## Integrating The Widget Into Your Lit Code

Import the `sygmaprotocol-widget` dependency to add the widget to your existing codebase. Add the custom tag into your render method: 

```ts
import { LitElement, html } from 'lit'
import { customElement, property } from 'lit/decorators.js'
import '@buildwithsygma/sygmaprotocol-widget'

@customElement('my-element')
export class MyElement extends LitElement {
  render() {
    return html`
      <div>
        <sygmaprotocol-widget></sygmaprotocol-widget>
      </div>
    `
  }

}

declare global {
  interface HTMLElementTagNameMap {
    'my-element': MyElement
  }
}
```

You can also pass properties into the widget to customize its behaviour. See [Properties](05-properties.md) for more.