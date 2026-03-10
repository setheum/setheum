# Relayers

_Relayers are entities that ensure bridge decentralization and execute asset bridging._

## Abstract

At the core of Sygma exists a set of relayers that are distributed among several legal entities and operate under a
trusted federation model. Spreading the relayer set across several legal entities creates a distribution of
responsibilities and mitigates risk by disincentivizing relayers in the set to act in an unfair manner.
Each relayer within the set listens to both the source and destination chains that are being bridged by Sygma. Based on
events that are emitted, these relayers then execute relevant actions.
This multi-relayer set is responsible for relaying messages from a source network to a destination network. Relayers are
operating with private key share and execution happens on the destination network with MPC private key.
