#!/bin/bash

set -e

cargo build --release -p setheum-node
docker build --tag setheum-node:latest -f ./docker/Dockerfile .

cargo build --release -p chain-bootstrapper --features short_session
docker build --tag chain-bootstrapper:latest -f ./bin/chain-bootstrapper/Dockerfile .

# run the chain and the tests in two separate tmux windows
tmux new-session -d -s setheum0 './.github/scripts/run_consensus.sh';
tmux new-window -t "setheum0:1";
tmux send-keys -t "setheum0:1" './scripts/run_e2e.sh' Enter;

tmux a;

exit $?
