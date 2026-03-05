---
slug: /sdk/quickstart/txstatus
id: quickstart-txstatus
title: Checking A Transaction Status
description: The following section details how to check a transaction status using the Sygma SDK and the Explorer.
sidebar_position: 6
draft: false
---

### Using the Sygma Explorer

You can easily check the status of a cross-chain transfer invoked through the Sygma SDK using the [Sygma Explorer](../../04-integrating-with-sygma/02-explorer.md) UI.

### Using the `getTransferStatusData` function

Developers can also quickly integrate transaction status checks into their Sygma SDK-based applications.

1. Implement the `getStatus` function to use `getTransferStatusData` for retrieving transaction status:

```ts
const getStatus = async (
  txHash: string
): Promise<{ status: string; explorerUrl: string } | void> => {
  try {
    return await getTransferStatusData(Environment.TESTNET, txHash); 
  } catch (e) {
    console.log("error:", e);
  }
};
```

2. After initiating a transaction, use the `getStatus` function in a polling mechanism to continuously check the transaction status:

```ts
  if (status.isInBlock) {
      console.log(
        `Transaction included at blockHash ${status.asInBlock.toString()}`
      );
    } else if (status.isFinalized) {
      console.log(
        `Transaction finalized at blockHash ${status.asFinalized.toString()}`
      );
      unsub();
    }

    let dataResponse: undefined | { status: string; explorerUrl: string };

    const id = setInterval(() => {
      getStatus(status.asInBlock.toString())
        .then((data) => {
          if (data) {
            dataResponse = data;
            console.log(data);
          }
        })
        .catch(() => {
          console.log("Transfer still not indexed, retrying...");
        });
    }, 5000);

    if (dataResponse && dataResponse.status === "executed") {
      console.log("Transfer executed successfully");
      clearInterval(id);
      process.exit(0);
    }
  });
};
```

3. Execute your script to start the status check. The script will output the status of the transaction every 5 seconds and provide an explorer URL once the transaction is executed.