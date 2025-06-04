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
- protocol abiding behaviors. (people call this "honest")
- malicious actions prevention
- performance

A consensus mechanism is an algorithm or rule, that all the nodes follow, and subject to network delays or disruption, or malicious behaviors, under some assumptions, they could make a decsion on some data. People use the word "protocol" and use it as a subject. When readers see "the protocol will do X," that means each protocol byding nodes would do X. People use a specific "chain" to represent the protocol too. A chain contains data which is the outcome from the protocol.

When we read a specific actor end with "er" or "or," thinking them as a cyborg, a combination of human and machines. A validator is a human who respond to incentives, they like rewards and avoid lossing their deposits. They are also computers, who can stay up 24 hours sending messages and perform cryptographic actions like signing digital signatures.


# The global view: The chain and their Checkpoints

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

The point of Anti Sybil is to make acquiring the writing permission costly. Usually it specify a specific type of asset, and a node prove the ownership of the asset to get the write permission. For example, Bitcoin use proof of work mechanism, where a miner attaches on a block a solution to a hash puzzle, a proof of heavy computational work, to demenstrate owning a mining machine and convince nodes to justify the write permission of the block.

The proof of stake, a terrible name that derived from proof of work, is seemingly making the write permission costly by proving the ownership of native token. But that is kind of misleading. First, proving the ownership of native asset is much more trivial -- a digital signature from validator is suffice. Second, the meaning of staking is not just making the write permission costly, but also making the abuse costly. An attacker's asset is indestrutable in for form of physical miner machine in PoW world, but it is in digital form of on chain deposit.

### Checkpoint security

A healthy system should build consistant checkpoints without interruption. We can breakdown this desired situation into  two security properties:

- Safety: network don't split by making conflicting checkpoints 
- Liveness: network can keep finalizing new checkpoints

Note that Gasper doesn't gurantee the two properties are always satisfied no matter what. It set a clear boundary on how costly to break it. We'll talk about a recovery strategy later.

What can an attacker owning a susstaintiable shares of the asset pie can do? Since the full node must follow the state transition rule, they can reject an invalid block that failed some basic checks. 

For attacking the consensus [^censor], What the attacker left to do is the following:

- Equivocate: They can attempt to split the network by sending conflicting messgaes
- Offline: They can attempt to halt the chain by not sending messages

Equivocate is attributable action, conflicting messages can be collected and computationally confirm the violation, so that the system apply a heavy penalty to the behavior, resulting a high cost in violation.

Offline is non-attributable action. When a validator is accused of censoring by not voting checkpoints, we can't tell if they are actually offline or people are censoring them. The behavior is penalized lightly initially, but gradually increasing, called quadratice leaking. In practice, a home staker could be offline for certain reasons -- traveling, power outage, client software update, etc. The protocol is designed to require uptime for a single validator to be at least 2/3 of a year so that a minor offline wouldn't hurt profit too much. 

### The meaning of staking security

What does it mean when we have 34 million Ether staked?

Currently, the chain ask a quorum of two-third, or 66% stake to finalize the chain. To break safety -- meaning creating conflicting checkpoints to split the network -- The attacker needs to acquire 2/3 of the stake. And surely after the attack, the conflicting messages can be collected and trigger the slashing process. The attacking capital would be vaporized.

The protocol is assumed to be breakable but also recoverable. A safety fault causes two conflicting checkpoints being finalized, which requires a minority softfork to choose a checkpoint for recovery.

The attacker can also initiate a liveness attack by acquiring more than 1/3 of the pie and stop building any checkpoints. The chain would be unusable for a while until the attacking fund is drained up by the quadratice leaking -- to the point that honest nodes get back to 2/3.

There's a fundamental tradeoff between tolerance to safety and to liveness attack -- requiring X shares for finalization means a tolerance for X in safety attack and 1-X for liveness attack. It is generally considered the safety attack is more sever than a liveness attack. That's why the protocol is parameterized to tolerate the former.

Here we need to talk about how the attacker get X shares of the pie. Naively, getting 2/3 of pie for 34 million Ether might mean getting another 68 million Ether and go through the deposit process. Realistically, they can buy 22 million Ether equivalent of nodes from existing validators. Or, they could just infect those nodes with cyberattacks without even buying them.

For staking service providers they might already get a sustansive pie. The community has been advocating people to avoid using staking services. Home staking is the recommended way of staking. It is also recommended for the home stakers to diversify their software and hardware stacks -- the client software they used, operating systems run, hardware they use -- to prevent a cyberattack to acquire a big pie.

#### How much stake is enough?

At the first glance, it seems the more stake the more secure. But the system also pays validator rewards for their service. The system is parameterized to reward a stock market equivalent of yield to a X amount of stake size. The yeild decreases when the stake size is greater than desired; and increases when the stake size is lower. 


## Performance

We want to the chain to be finalized as soon as possible. We also want the validators as more as possible. What prevent us from doing that?

The bottleneck is the signature verification. To tally validators' votes, the system need to verify their digital signatures. This creates a trade-offs in validator size and finality time.

The more validators, the more signatures to tally. How do you limit the size of validators? You raise the minimum deposit size. The 32 ETH is a really high number. But this number is chosen so that even if you buy all the outstanding Ethers and turn them into validators, the system still have the capacity to process them.

Imagine a more efficient siganture scheme is invented. This would bring down the finality time or deposit size.

Once the signature verification is resolved. The next bottlenck would be the network propogation speed. This determine the slot time and a bottlenexk for the chain. 

## Conclusion

Getting back to the examples in the beginning. Your transactions would have the same nonce. Assume a validator pack one of your transaction in their block, another packed another block.  When left node see your messages, it adjust its local view and deduct your balance 7 and add them to Alice's. The right node do the similar to the other transaction, deduct 7 from you and add to Bob. It could the case that the block with traction to Alice outweight the other -- either beating it by the time, either beating it by getting more attesations -- and made its way to the chain -- that all the nodes would follow the fork choice rule and recognize it as the latest chain head. The right node, originally applied the block containing Bob's transaction, now see the block with Alice's transaction, reorg to the later -- it rollback the state to where you have balance of 10 and then move 7 to alice. Now all the nodes agree that you now have balance of 3 and Alice has 7 more. A while later, the block containing tx to alice fall behind a checkpoint block. The transaction is finalized and never changed.

The order of the transactions matters to the state. And the Ethereum consensus is designed to set the "Order" of the system. 

Readers might worry that there are lot of data to form consensus on. Thanks to cryptographic hashes, nodes can agree on massive amounts of information—such as the balances of a million accounts—simply by agreeing on a single hash that commits to that data. Infact, The system reach consensus on chain head, which is the block head's hash. The consensus block's body has execution payload, a block hash of application chain, so that the consensus covers the execution client. Finally, rollups are covered by the consensus too because their rollup batch hash is commited to an Ethereum account.

From this we can see the consensus and the staked Ether can be "scaled" to cover a broader system. States of the rollups and other system can be piggybacked to Ethereum consensus, without finding another new capitals to build more consensus security defense. And this is the goal of Ethereum, a prudent organization of p2p network that reuse all resousces it can make application develpers focusing on what they really good at -- serving users needs. 


## Potential references

- https://arxiv.org/pdf/2003.03052
- https://blog.ethereum.org/2014/11/25/proof-stake-learned-love-weak-subjectivity for subjectivity discussions.
- https://beaconcha.in/ for current stake numbers


 [^censor]: MEV

