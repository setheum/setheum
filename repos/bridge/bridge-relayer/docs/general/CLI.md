# Sygma Relayer CLI Guide

### Introduction

This guide details specific Command Line Interface (CLI) commands for the Sygma relayer, focusing on functionalities provided in the `topology`, `peer`, `keygen` and `utils` modules.

## Topology commands

### Encrypt Topology Command (topology)

#### Usage:
`./sygma-relayer topology encrypt --path [path] --encryption-key [key]`

#### Description:
Encrypt the provided topology with AES CTR. Outputs IV and ciphertext in hex.

#### Flags:
- `--path`: Path to JSON file with network topology.
- `--encryption-key`: Password to encrypt topology.

### Test Topology Command (topology)

#### Usage:
`./sygma-relayer topology test --url [url] --decryption-key [key] --hash [hash]`

#### Description:
Test if the provided URL contains a topology that can be decrypted with the provided password and parsed accordingly.

#### Flags:
- `--decryption-key`: Password to decrypt topology.
- `--url`: URL to fetch topology.
- `--hash`: Hash of the topology.

## Libp2p (peer) commands

### Generate Key Command (peer)

#### Usage:
`./sygma-relayer peer gen-key`

#### Description:
Generate a libp2p identity key using RSA with a key length of 2048 bits.

### Peer Info Command (peer)

#### Usage:
`./sygma-relayer peer info --private-key [key]`

#### Description:
Calculate a libp2p peer ID from the provided base64 encoded libp2p private key.

#### Flags:
- `--private-key`: Base64 encoded libp2p private key.

## Keygen commands

### Generate ECDSA keypair

#### Usage: 
`./sygma-relayer keygen gen-key`

#### Description:
Generate a 256-bit ECDSA keypair and print it out. This keypair can be used as a relayer's execution keypair.

## Other util commands

### Derivate SS58 Command (utils)

#### Usage:
`./sygma-relayer utils derivateSS58 --privateKey [key] --networkID [id]`

#### Description:
Print an SS58 formatted address (Polkadot) for a given PrivateKey in hex.

#### Flags:
- `--privateKey`: Hex encoded private key.
- `--networkID`: Network ID for a checksum, as per the registry.
