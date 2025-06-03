---
status: idea
---

# Consensus: The hivemind at work

A peer to peer network has no leaders or "master node" among them. All nodes are equal. But they have to reach a global consensus. All they can do is to broadcast messages to peers. How do they acheive this?

Consider a network of 3 nodes and they agree you have balance of 10 in the beginning. You are one of the node and you are mischivious. You tell the lefthand node you want to send Alice 7 and then the righthand node you send Bob 7. Negative balance is prohibited, so only one of message can go through -- either Alice get 7 or Bob. How would nodes make decisions like this? This is consensus mechanism attempt to resolve. We'll come back to this scenario in a concret example at the end.

In this chapter, we unpack the hive mind of the Ethereum and show you how the p2p network "put themselve together." In the end, you should able to intepret the security meaning of 100 million USD stake.


## Notes on the communication to readers

A collective emergant behavior is not easy to describe. Imagine an ant bridge -- ants chain themselves together to form a bridge. The bridge is the emergant structure of interest, which allows other ant fellows to pass. But in order to understand the bridge, we'll need to understand the individual behaviors of ants too -- they hold each other's body. 

We'll channel the readers attention in the following order:

- emergant structure: the desired global view that individuals local view converged to. Note that this global view might take some time to take shape. Nodes agree on what happened 10 min ago but diverge on the 3 seconds ago.
- protocol abiding behaviors
- malicious actions prevention
- performance

A **consensus mechanism** is an algorithm or set of rules that all nodes in a network follow. Even in the presence of network delays, disruptions, or malicious behavior, these rules allow nodes to reach agreement on data—*under certain assumptions*. The term *protocol* is often used interchangeably with *consensus mechanism*. When you see “the protocol will do X,” it means that each node following the protocol will independently perform action X. The term *chain* is also frequently used to refer to the protocol, especially when discussing the data it produces. A chain contains data that reflects the outcome of the protocol.

Readers might worry that there are lot of data to form consensus on. Thanks to cryptographic hashes, nodes can agree on massive amounts of information—such as the balances of a million accounts—simply by agreeing on a single hash that commits to that data. Infact, The system reach consensus on chain head, which is the block head's hash. The consensus block's body has execution payload, a block hash of application chain, so that the consensus covers the execution client. Finally, rollups are covered by the consensus too because their rollup batch hash is commited to an Ethereum account.

When you see a role ending in “-er” or “-or” (like *validator*), think of it as a *cyborg*: part human, part machine. A validator is a person responding to economic incentives—they seek rewards and try to avoid penalties like losing their deposit. But a validator is also a computer, running 24/7, sending network messages, and signing cryptographic proofs.

- TODO: 
- [ ] blockchain paradigm / State transition

# The global view: Chains and Checkpoints

Ethereum's consensus is governed by the **Gasper** protocol, a specific implementation of proof of stake. The system attempts to create two outcomes:

- One a shorter time scale, **Slot** (currently 12 seconds), the system agrees on a new block. These blocks can be reverted depends on the nodes' local view.
- One a longer time scale, **Epoch** (32 slots, \~10 minutes): The system *finalize* a checkpoint (i.e. the block at the boundary of an epoch). When the checkpoint is reached, the node never revert the history behind the checkpoint. Transactions behind it are set in stone.

The chain provide a backbone for frequent inclusion of data, at the tiny risk of revert, while checkpoints provides certainty.

This process is faciliated by specialized nodes called validators and coordinated by the **beacon chain**, a system-level chain that manages all consensus responsibilities.

## Validators: Actors who participate consensus

Validators' job is to vote on blocks and occasionally propose new ones. Anyone can become a validator by depositing 32 ETH. Misbehavior is penalized, while honest participation earns newly minted Ether as a reward.



Validators send messages called **attestations**, which include two key pieces of information:

1. The most recent block they’ve seen.
2. The checkpoint they support for finalization.

Because of network delays, full nodes are constantly resolving uncertainty. They might receive two blocks (say, A and B) at the same height. In such cases, they apply a **fork-choice rule**: choose the block that has the most attestation support. Once a checkpoint is finalized, nodes will never revert it—they treat it as an irreversible decision.

### Validator life cycle

Validators' onboard and offboard are strictly controled so that attacker can't perform the attack with a sudden entry and exit.

## Security: What had things validators can do and how to prevent that?

### Stakes: What's good for that?

Allowing data to be proposed and written in a decentralized, peer-to-peer system is incredibly powerful. Since Bitcoin’s invention, many new consensus mechanisms have emerged to determine who gets *write access* to the ledger. This problem is closely related to what's called the **anti-Sybil** mechanism.

The goal of anti-Sybil design is to make acquiring write permission *costly*, thereby discouraging fake identities. Typically, the protocol specifies a type of scarce asset, and nodes must prove ownership of that asset to gain write rights. For instance, Bitcoin uses **proof of work**: miners append a solution to a hash puzzle as proof that they’ve spent significant computing resources. This demonstrates they own real-world mining hardware, justifying their right to propose a block.

Ethereum uses **proof of stake**—a name that imitates "proof of work" but with key differences. It’s misleading to say proof of stake merely makes write access costly by requiring ownership of native tokens. First, proving token ownership is easy: a digital signature suffices. More importantly, *staking* is not just about paying to participate—it’s about making misbehavior expensive. Unlike physical mining equipment, staked tokens are **on-chain deposits**, and they can be destroyed as punishment. This introduces real financial risk for attackers.

### Checkpoint security

- Safety: network don't split by making conflicting checkpoints 
- Liveness: network can keep finalizing new checkpoints

What can an attacker owning a susstaintiable shares of the asset pie can do? Since the full node must follow the state transition rule, they can reject an invalid block that failed some basic checks. 

For attacking the consensus [^censor], What the attacker left to do is the following:

- Equivocate: They can attempt to split the network by sending conflicting messgaes
- Censorship: They can attempt to halt the chain by not proposing blocks


Censorship is non-attributable
Equivocate is attributable


### The meaning of staking security

What does it mean when we have 34 million Ether staked?

Currently, the chain ask a quorum of two-third, or 66% stake to finalize the chain. The attacker can put 1/3 in checkpoint A in subnet A and 1/3 in  checkpoint B in subnet B, but then  

The more stake the more secure.


## Performance

Signature verification

Deposits

Finality time


## Potential references

- https://arxiv.org/pdf/2003.03052
- https://blog.ethereum.org/2014/11/25/proof-stake-learned-love-weak-subjectivity for subjectivity discussions.
- https://beaconcha.in/ for current stake numbers


 [^censor]: MEV

