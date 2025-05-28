---
status: idea
---

# Primer

A consensus mechanism is an algorithm or rule, that all the nodes follow, and subject to network delays or disruption, or malicious behaviors, under some assumptions, they could make a decsion on some data. People use the word "protocol" and use it as a subject. When readers see "the protocol will do X," that means each protocol byding nodes would do X. People use a specific "chain" to represent the protocol too. A chain contains data which is the outcome from the protocol.

Note that since we can commit data to hashes, nodes can agree on a million peoples' balance by agreeing on the hash of the balances.

When we read a specific actor end with "er" or "or," thinking them as a cyborg, a combination of human and machines. A validator is a human who respond to incentives, they like rewards and avoid lossing their deposits. They are also computers, who can stay up 24 hours sending messages and perform cryptographic actions like signing digital signatures.

# How Ethereum Node form consensus

Ethereum's consensus is handled by now called Gasper protocol, a specific proof of stake implementation. The beacon chain is a system chain taking care of all the consensus responsibilities.

Validators are specialized nodes who participate in consensus. Their work is to send messages to vote and occationally propose a new block. Anyone can become one by putting down 32 ETH for deposit, subject to punishable actions. They got rewarded with new minted Ether for the work and resources they contributed.

Roughly speaking, Gasper has two decision to make. One the short horizon called slots (12 seconds currently), nodes decide on what block to agree on. One a longer horizon, an epoch (10 min and 32 slots), a checkpoint -- an epoch boundary block -- must be finalized.

The messages the validators sent, are called attestations. They contains two pieces of information, a block they recently seen, and a checkpoint they'd like to finalize.

For a full node, they are always under confusion because of network broadcasting delays. They could receive a block A, but later a block B, under the same hight. They make a fork choice by chosing the block that backed by the most attested one. When they learn a checkpoint is finalized, they'll never change the finalized block again.


## Why designed in this way?

Being able to propose data to write in a peer-to-peer system is a very powerful. Since the inception of Bitcoin protocol, people have been coming up ways to form consensus and distribute writing permission to public. The later is also called Anti Sybil mechanism.

The point of Anti Sybil is to make acquiring the writing permission costly. Usually it specify a specific type of asset, and a node prove the ownership of the asset to get the write permission. For example, Bitcoin use proof of work mechanism, where a miner attaches on a block a solution to a hash puzzle, a proof of heavy computational work, to demenstrate owning a mining machine and convince nodes to justify the write permission of the block.

The proof of stake, a terrible name that derived from proof of work, is seemingly making the write permission costly by proving the ownership of native token. But that is kind of misleading. First, proving the ownership of native asset is much more trivial -- a digital signature from validator is suffice. Second, the meaning of staking is not just making the write permission costly, but also making the abuse costly. An attacker's asset is indestrutable in for form of physical miner machine in PoW world, but it is in digital form of on chain deposit.



Attackers acquiring 50% 

- You can halt the chain
- You can convince the network into one view and later into another view. Double spending is one way the attacker can profit from this way.
- You can censor people

------

# Unused



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

- https://arxiv.org/pdf/2003.03052
- https://blog.ethereum.org/2014/11/25/proof-stake-learned-love-weak-subjectivity for subjectivity discussions.
- https://beaconcha.in/ for current stake numbers
