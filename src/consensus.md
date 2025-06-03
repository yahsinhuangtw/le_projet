---
status: idea
---

# Primer

A **consensus mechanism** is an algorithm or set of rules that all nodes in a network follow. Even in the presence of network delays, disruptions, or malicious behavior, these rules allow nodes to reach agreement on data—*under certain assumptions*. The term *protocol* is often used interchangeably with *consensus mechanism*. When you see “the protocol will do X,” it means that each node following the protocol will independently perform action X. The term *chain* is also frequently used to refer to the protocol, especially when discussing the data it produces. A chain contains data that reflects the outcome of the protocol.

Thanks to cryptographic hashes, nodes can agree on massive amounts of information—such as the balances of a million accounts—simply by agreeing on a single hash that commits to that data.

When you see a role ending in “-er” or “-or” (like *validator*), think of it as a *cyborg*: part human, part machine. A validator is a person responding to economic incentives—they seek rewards and try to avoid penalties like losing their deposit. But a validator is also a computer, running 24/7, sending network messages, and signing cryptographic proofs.

- TODO: 
- [ ] blockchain paradigm / State transition

# How Ethereum Nodes Form Consensus

Ethereum's consensus is governed by the **Gasper** protocol, a specific implementation of proof of stake. The system attempts to create two outcomes:

- One a shorter time scale, **Slot** (currently 12 seconds), the system agrees on a new block. These blocks can be reverted depends on the nodes' local view.
- One a longer time scale, **Epoch** (32 slots, \~10 minutes): The system *finalize* a checkpoint (i.e. the block at the boundary of an epoch). When the checkpoint is reached, the node never revert the history behind the checkpoint. Transactions behind it are set in stone.

This process is faciliated by specialized nodes called validators and coordinated by the **beacon chain**, a system-level chain that manages all consensus responsibilities.

## Validators: Actors who participate consensus

Validators' job is to vote on blocks and occasionally propose new ones. Anyone can become a validator by depositing 32 ETH. Misbehavior is penalized, while honest participation earns newly minted Ether as a reward.



Validators send messages called **attestations**, which include two key pieces of information:

1. The most recent block they’ve seen.
2. The checkpoint they support for finalization.

Because of network delays, full nodes are constantly resolving uncertainty. They might receive two blocks (say, A and B) at the same height. In such cases, they apply a **fork-choice rule**: choose the block that has the most attestation support. Once a checkpoint is finalized, nodes will never revert it—they treat it as an irreversible decision.

TODO: 

- [ ] Validator life cycle

## Why Is It Designed This Way?

### Anti-Sybil

Allowing data to be proposed and written in a decentralized, peer-to-peer system is incredibly powerful. Since Bitcoin’s invention, many new consensus mechanisms have emerged to determine who gets *write access* to the ledger. This problem is closely related to what's called the **anti-Sybil** mechanism.

The goal of anti-Sybil design is to make acquiring write permission *costly*, thereby discouraging fake identities. Typically, the protocol specifies a type of scarce asset, and nodes must prove ownership of that asset to gain write rights. For instance, Bitcoin uses **proof of work**: miners append a solution to a hash puzzle as proof that they’ve spent significant computing resources. This demonstrates they own real-world mining hardware, justifying their right to propose a block.

Ethereum uses **proof of stake**—a name that imitates "proof of work" but with key differences. It’s misleading to say proof of stake merely makes write access costly by requiring ownership of native tokens. First, proving token ownership is easy: a digital signature suffices. More importantly, *staking* is not just about paying to participate—it’s about making misbehavior expensive. Unlike physical mining equipment, staked tokens are **on-chain deposits**, and they can be destroyed as punishment. This introduces real financial risk for attackers.

### Threat Model

What can an attacker owning a susstaintiable shares of the asset pie can do? Since the full node must follow the state transition rule, they can reject an invalid block that failed some basic checks. 

What the attacker left to do is the following:

- They can attempt to halt the chain by not proposing blocks
- They can 

---
to clean up 


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
