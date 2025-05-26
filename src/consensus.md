---
status: idea
---

# How Ethereum Node form consensus

In the p2p system and where money balance is an important application, nodes must agree on the state of the balance. 

Network has delays and transactions take time to broadcast, so nodes form local view of the data they've seen.

They don't need to agree on them real time. But they should have high certainty about 10 minites ago.

because of commitments, agreing on state means agreing on state root. Agreing on a batch of transactions means agreeing on their merkle tree root. Usually its more efficient to agree on a batch of transactions than individual ones. Nodes agree on blocks.

To agree on something you can ask computers to vote, but we have to define what count as one vote. 

## Anti-Sybil and historical remarks

For decentralization, it is desirable to split the data writing role to as many independent entities as possible. You could nominate 10 people and ask them to register their public keys. They propose blocks in a round-robin fashion.

This work in simple cases. Most testnets use this approach. Some ancient business facing consordium chain use this approach too. But for public chain system, we want to open this opportunity to people -- they can freely join or exit.

This can be achieved in two ways:

- You can build an identity system, when people prove they are a real human, they can register their public key. They can propose block with their computer, with signature attached for verification.
- You could distribute this right to write data to people holding certain kinds of assets. You lottery draw them and they propose blocks when drawn.

The identity approach is hard to achieve. People tried and failed.

The second approach is generally called Proof of Asset. There are two main categories: Objective and subjective ones.

Objective ones are proofs attached on a block, that demenstrates it is produced with someone owning some kind of hard wares. Proof of work is the most important case. Bitcoin is the first known case of using proof of work. An hash puzzle solution is attached on the block, where a node can easily verify. The only way to find a hash puzzle solution is to grind it with computations, thus proving the proposer owns a computer. This check is objective, meaning without relying on the current state of data, a node can already validate the block proposing right.

Other than computations, people also tried proving the ownership of disk or memories.

Subjective ones are proofs that relies on the current state. Proof of Stake is the most famous example. Users become a validator by depositing chain native tokens and register a public key. Users can also exit the validator role. That means for a node to validate the data write right, they need to check the chain state to know if a validator is already entered or exited.

The benefit of subjectivity is that the asset lies on your db records instead of real world. 

- [ ] Finality, checkpoints
- [ ] Attributable fault, non-attributable fault

## The meaning of staking security

What does it mean when we have 34 million Ether staked?

50% attack. 33% attack.

The more stake the more secure.




## Fact check todos

- [ ] Definition of objective and subjective. 


## Potential references

- https://blog.ethereum.org/2014/11/25/proof-stake-learned-love-weak-subjectivity for subjectivity discussions.
- https://beaconcha.in/ for current stake numbers
