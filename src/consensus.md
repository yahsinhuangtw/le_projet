---
status: idea
---

# Consensus: The Hivemind at Work

A peer-to-peer network has no leaders or "master nodes"—all nodes are equal. Yet they must reach global consensus using only broadcast messages to peers. How do they achieve this?

Consider a network of three nodes where you initially have a balance of 10. You are one of the nodes, and you decide to be mischievous. You tell the left node you want to send Alice 7, then tell the right node you want to send Bob 7. Since negative balances are prohibited, only one message can succeed—either Alice gets 7 or Bob does. How do the nodes make this decision? This is what consensus mechanisms attempt to resolve. We'll return to this scenario with a concrete example at the end.

In this chapter, we unpack Ethereum's hivemind and show how the peer-to-peer network "puts itself together." By the end, you should be able to interpret the security meaning of $100 million in staked value.

## Notes on Communication to Readers

Collective emergent behavior is not easy to describe. Imagine an ant bridge—ants chain themselves together to form a crossing. The bridge is the emergent structure of interest, allowing fellow ants to pass. But to understand the bridge, we must first understand individual ant behaviors—how they hold onto each other's bodies.

We'll channel readers' attention in the following order:

- **Emergent structure**: The desired global view that individual local views converge toward. Note that this global view takes time to solidify. Nodes agree on what happened 10 minutes ago but diverge on events from 3 seconds ago.
- **Protocol-abiding behaviors** (often called "honest")
- **Malicious action prevention**
- **Performance**

A **consensus mechanism** is an algorithm or rule that all nodes follow. Subject to network delays, disruptions, or malicious behaviors, and under certain assumptions, they can make decisions about data. People use the word "protocol" as a subject. When readers see "the protocol will do X," it means each protocol-abiding node will do X. People also use "chain" to represent the protocol. A chain contains data that is the outcome of the protocol.

When we encounter actors ending with "-er" or "-or," think of them as cyborgs—combinations of humans and machines. A validator is a human who responds to incentives, seeking rewards and avoiding deposit losses. They are also computers that stay online 24/7, sending messages and performing cryptographic actions like signing digital signatures.

# The Global View: Chains and Checkpoints

Ethereum's consensus operates under the **Gasper** protocol, a specific proof-of-stake implementation. The system creates two outcomes:

- **Short-term (Slots)**: Every 12 seconds, the system agrees on a new block. These blocks can be reverted depending on nodes' local views.
- **Long-term (Epochs)**: Every 32 slots (~10 minutes), the system *finalizes* a checkpoint (the block at an epoch boundary). Once reached, nodes never revert the history behind the checkpoint—those transactions are set in stone.

The chain provides a backbone for frequent data inclusion with minimal reversion risk, while checkpoints provide certainty.

This process involves specialized nodes called validators, coordinated by the **beacon chain**—a system-level chain managing all consensus responsibilities.

## Validators: Consensus Participants

Validators vote on blocks and occasionally propose new ones. Anyone can become a validator by depositing 32 ETH. Misbehavior is penalized, while honest participation earns newly minted Ether as rewards.

Validators send messages called **attestations** containing two key pieces of information:

1. The most recent block they've seen
2. The checkpoint they support for finalization

Due to network delays, full nodes constantly resolve uncertainty. When they receive two blocks (A and B) at the same height, they apply a **fork-choice rule**: choose the block with the most attestation support. Once a checkpoint is finalized, nodes never revert it—treating it as irreversible.

### Validator Lifecycle

Validator onboarding and offboarding are strictly controlled so attackers can't perform sudden entry-and-exit attacks.

## Security: Preventing Malicious Validator Actions

### Stakes: Their Purpose

The point of anti-Sybil mechanisms is making write permission acquisition costly. Usually, this specifies a particular asset type, and nodes prove asset ownership to gain write permission. For example, Bitcoin uses proof of work, where miners attach block solutions to hash puzzles—proof of heavy computational work—demonstrating mining machine ownership and justifying block write permission.

Proof of stake—a name derived from proof of work—seemingly makes write permission costly by proving native token ownership. But this is misleading. First, proving native asset ownership is trivial—a validator's digital signature suffices. Second, staking means not just making write permission costly, but making abuse costly. An attacker's assets are indestructible physical mining machines in the proof-of-work world, but they exist as destructible on-chain deposits in proof of stake.

### Checkpoint Security

A healthy system should build consistent checkpoints without interruption. We can break this down into two security properties:

- **Safety**: The network doesn't split by creating conflicting checkpoints
- **Liveness**: The network continues finalizing new checkpoints

Note that Gasper doesn't guarantee these properties under all circumstances—it sets clear boundaries on violation costs. We'll discuss recovery strategies later.

What can an attacker owning substantial asset shares do? Since full nodes must follow state transition rules, they can reject invalid blocks that fail basic checks.

For consensus attacks, attackers can:

- **Equivocate**: Attempt to split the network by sending conflicting messages
- **Go offline**: Attempt to halt the chain by withholding messages

Equivocation is attributable—conflicting messages can be collected and computationally verified for violations, triggering heavy penalties and high violation costs.

Going offline is non-attributable. When validators are accused of censoring by not voting on checkpoints, we can't distinguish between actual offline status and censorship. This behavior receives light initial penalties that gradually increase through "quadratic leaking." In practice, home stakers may go offline for legitimate reasons—traveling, power outages, client software updates. The protocol requires single validator uptime of at least two-thirds of a year, so minor offline periods don't significantly hurt profits.

### The Meaning of Staking Security

What does having 34 million Ether staked mean?

Currently, the chain requires a two-thirds (66%) stake quorum to finalize. To break safety—creating conflicting checkpoints that split the network—attackers need two-thirds of the stake. After such attacks, conflicting messages can be collected to trigger slashing, vaporizing the attacking capital.

The protocol assumes breakability but also recoverability. Safety faults causing two conflicting finalized checkpoints require minority soft forks to choose a recovery checkpoint.

Attackers can also initiate liveness attacks by acquiring more than one-third of the stake and stopping checkpoint building. The chain becomes unusable until attacking funds drain through quadratic leaking—until honest nodes regain two-thirds control.

There's a fundamental tradeoff between safety and liveness attack tolerance. Requiring X shares for finalization means X tolerance for safety attacks and 1-X for liveness attacks. Safety attacks are generally considered more severe, which is why the protocol is parameterized to tolerate them.

We need to discuss how attackers acquire X shares. Naively, getting two-thirds of 34 million Ether might require obtaining another 68 million Ether through the deposit process. Realistically, they could buy 22 million Ether equivalent from existing validators or infect those nodes through cyberattacks without purchasing them.

Staking service providers might already control substantial portions. The community advocates avoiding staking services, recommending home staking instead. Home stakers should also diversify their software and hardware stacks—client software, operating systems, hardware—to prevent cyberattacks from acquiring large portions.

#### How Much Stake Is Enough?

At first glance, more stake seems more secure. But the system also pays validator rewards for their service. The system is parameterized to reward stock market equivalent yields for X stake amounts. Yields decrease when stake size exceeds desired levels and increase when stake size is lower.

## Performance

We want chains finalized as quickly as possible with maximum validators. What prevents this?

The bottleneck is signature verification. To tally validator votes, the system must verify digital signatures, creating tradeoffs between validator size and finality time.

More validators mean more signatures to tally. How do you limit validator size? Raise minimum deposit requirements. The 32 ETH requirement is high, but chosen so that even converting all outstanding Ether to validators, the system retains processing capacity.

More efficient signature schemes would reduce finality time or deposit requirements.

Once signature verification is resolved, network propagation speed becomes the next bottleneck, determining slot time and chain limitations.

## Conclusion

Returning to our opening example: your transactions would have the same nonce. Assume one validator packs your transaction to Alice in their block, while another packs your transaction to Bob in theirs. When the left node sees your message, it adjusts its local view, deducting 7 from your balance and adding it to Alice's. The right node does similarly for the other transaction, deducting 7 from you and adding to Bob.

The block with Alice's transaction might outweigh the other—either by timing or gathering more attestations—making its way onto the chain. All nodes follow the fork choice rule, recognizing it as the latest chain head. The right node, originally applying Bob's transaction block, now sees Alice's transaction block and reorganizes to the latter. It rolls back the state to your balance of 10, then moves 7 to Alice. Now all nodes agree you have a balance of 3 and Alice has 7 more. Later, the block containing Alice's transaction falls behind a checkpoint block. The transaction is finalized and never changes.

Transaction order matters to state, and Ethereum consensus is designed to establish the system's "order."

Readers might worry about the vast amount of data requiring consensus. Thanks to cryptographic hashes, nodes can agree on massive information—like millions of account balances—by agreeing on a single hash committing to that data. The system reaches consensus on the chain head (the block head's hash). The consensus block's body contains an execution payload—an application chain block hash—so consensus covers the execution client. Finally, rollups are covered by consensus because their batch hashes are committed to Ethereum accounts.

From this, we see how consensus and staked Ether can "scale" to cover broader systems. Rollup states and other systems can piggyback on Ethereum consensus without requiring new capital for additional consensus security. This is Ethereum's goal: a prudent peer-to-peer network organization that reuses all available resources, letting application developers focus on what they do best—serving user needs.

## Potential references

- https://arxiv.org/pdf/2003.03052
- https://blog.ethereum.org/2014/11/25/proof-stake-learned-love-weak-subjectivity for subjectivity discussions.
- https://beaconcha.in/ for current stake numbers


 [^censor]: MEV

