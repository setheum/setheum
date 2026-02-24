## 4 Differences between the implementation and the paper.

There are several differences between the Set as described in the [paper](https://arxiv.org/abs/1908.05156) and the version implemented in SetBFT. Many of them are already described in previous sections but for completeness we briefly list the differences here.

1. The main version of Set uses Reliable Broadcast to disseminate units. The SetBFT implementation is closer to QuickSet (in the Appendix of the paper) that uses Reliable Broadcast only for Alerts.
2. The specifics of alerts are different in the SetBFT implementation -- in particular they do not require to freeze the protocol at any moment and are generally simpler.
3. SetBFT uses its own variant of Reliable Broadcast -- see the section [Reliable Broadcast](reliable_broadcast.md##reliable-broadcast).
4. Differences in the use of randomness -- see [Randomness in SetBFT](how_setbft_does_it.md#24-randomness-in-setbft).
5. The main version in the paper uses a full list of parent hashes instead of control hashes -- the latter is described in the Appendix as an optimization.
6. The paper's Appendix proposes the use of random gossip as a method of disseminating units -- SetBFT uses repeated broadcast + a request/response mechanism instead, which according to our experience performs much better in practice.
