# Topology map
A topology map is a configuration file that contains information about each relayer address so that relayers can communicate with each other. It also holds threshold information that is used in the tss process.

## Topology map initialization
The topology map has the following structure: 
```
{
    "peers": [
        {"peerAddress": "/dns1/relayer-0.relayer-0-STAGE/tcp/9000/p2p/QmVuMSb6unWs2m22sgEQF97XvShbrd9JAkX7Kh2xQ9EYGC"},
        {"peerAddress": "/dns1/relayer-1.relayer-1-STAGE/tcp/9000/p2p/QmcLn2tXGcYA1FUUWsRQoRGmWN17SncGuvjFL3h9azMRgB"},
        {"peerAddress": "/dns1/relayer-2.relayer-2-STAGE/tcp/9000/p2p/QmVF5HpD7oPkRGFF62pJC6w2QQgD5fZ6qVAzupamugjsTC"},
        {"peerAddress": "/dns1/relayer-3.relayer-3-STAGE/tcp/9000/p2p/QmZG9c35vUBehEDTkG1mLhw2J4jHG3VsYcJAuY1kqevohE"},
    ], 
    "threshold": "2"
}
```

After the topology map file is created, the file needs to be encrypted and uploaded to a remote service(ipfs).
On startup, relayers are fetching the topology map from the remote service, and store the data in a local file.
 
## Topology map update
To update the topology map, the map on the remote service needs to be updated. After we updated the topology map on ipfs, the `refreshKey` function needs to be called on the [bridge smart contract](https://github.com/sygmaprotocol/sygma-solidity/blob/master/contracts/Bridge.sol) (only Admin is allowed to trigger this function). `refreshKey` function is implemented only on the evm chain. The `refreshKey` function is called with the topology map hash. This hash is used to prevent relayers using invalid or compromised topology when updating it. Relayers will start using the new, updated topology only when the `KeyRefresh` event is processed which is emitted by the `refreshKey` function.

## Env variables
- SYG_RELAYER_MPCCONFIG_TOPOLOGYCONFIGURATION_ENCRYPTIONKEY - the key that is used to encrypt the topology map
- SYG_RELAYER_MPCCONFIG_TOPOLOGYCONFIGURATION_URL - topology map location
- SYG_RELAYER_MPCCONFIG_TOPOLOGYCONFIGURATION_PATH - local file where the topology map is stored after the download from the remote service
 
## Topology encryption/decryption details
Topology should be encrypted with AES using CTR mode.
IPFS should return hex formatted IV + data. To help you there are 2 utility CLI described below.

## Utility CLI
For more details on all CLI commands you can check out [CLI commands page](/docs/general/CLI.md).

`./relayer topology encrypt --path ./topology.json --encryptionKey 123` 
This command will encrypt provided topology and output corresponding hash and encrypted toplogy in hex representation (iv + data)

`./relayer topology test --hash 123  --url https://cloudflare-ipfs.com/ipfs/123  --decryptionKey 321` 
This command will fetch topology from IPFS and test it according to Relayers topology initialization flow. 
This allows to test correctnes of topology before actually calling `RefreshKey`

