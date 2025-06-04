---
status: idea
---

# Scaling Ethereum




## Scaling Consensus: Shorter finality time and more validators 

We want chains finalized as quickly as possible with maximum validators. What prevents this?

The bottleneck is signature verification. To tally validator votes, the system must verify digital signatures, creating tradeoffs between validator size and finality time.

More validators mean more signatures to tally. How do you limit validator size? Raise minimum deposit requirements. The 32 ETH requirement is high, but chosen so that even converting all outstanding Ether to validators, the system retains processing capacity.

More efficient signature schemes would reduce finality time or deposit requirements.

Once signature verification is resolved, network propagation speed becomes the next bottleneck, determining slot time and chain limitations.

