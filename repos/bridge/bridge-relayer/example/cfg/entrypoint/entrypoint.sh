#!/bin/bash

# Start bitcoind in the background
bitcoind -regtest -daemon -rpcuser=user -rpcpassword=password -rpcport=18443 -rpcallowip=0.0.0.0/0 -rpcbind=0.0.0.0

# Wait for bitcoind to start
sleep 5

bitcoin-cli -regtest -rpcuser=user -rpcpassword=password loadwallet "test" true

bitcoin-cli -regtest -rpcuser=user -rpcpassword=password importdescriptors '[{ "desc": "addr(bcrt1pdf5c3q35ssem2l25n435fa69qr7dzwkc6gsqehuflr3euh905l2sjyr5ek)#qyrrxhja", "timestamp":"now", "label": "ray"}]'
bitcoin-cli -regtest -rpcuser=user -rpcpassword=password importdescriptors '[{ "desc": "addr(bcrt1pja8aknn7te4empmghnyqnrtjqn0lyg5zy3p5jsdp4le930wnpnxsrtd3ht)#n807x7zd", "timestamp":"now", "label": "ray"}]'
bitcoin-cli -regtest -rpcuser=user -rpcpassword=password importdescriptors '[{ "desc": "addr(mrheH3ouZNyUbpp9LtWP28xqv1yhNQAsfC)#wmef0tpn", "timestamp":"now", "label": "ray"}]'

# Mine some blocks to fund the wallet (101 blocks to ensure the funds are spendable)
bitcoin-cli -regtest -rpcuser=user -rpcpassword=password generatetoaddress 101 "bcrt1pdf5c3q35ssem2l25n435fa69qr7dzwkc6gsqehuflr3euh905l2sjyr5ek"
bitcoin-cli -regtest -rpcuser=user -rpcpassword=password generatetoaddress 150 "mrheH3ouZNyUbpp9LtWP28xqv1yhNQAsfC"

bitcoin-cli -regtest -rpcuser=user -rpcpassword=password listunspent

# Check balance
BALANCE=$(bitcoin-cli -regtest -rpcuser=user -rpcpassword=password getbalance)
# Check if BALANCE is assigned properly
if [ -z "$BALANCE" ]; then
  echo "Failed to retrieve balance"
  exit 1
fi
echo "Wallet Balance: $BALANCE BTC"

# Keep the container running
tail -f /dev/null