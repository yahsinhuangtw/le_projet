---
status: idea
---

# Pricing: How Ethereum control congesture?

Any computation service can be breakdown to the following three distinct elements:

- CPU computation
- Network bandwidth
- Storage: This means storing data in a full node. Memory is a different issue, we'll cover it in other context. 

That is processing, sending, and storing data.

Ethereum is no different from Amazon Web Service in this perspective. Given a period of time, say an hour, the whole network has only finite amount of resources to offer. They are bound to the computationally weakest part of the system -- the full nodes that target consumer grade hardwares -- and their ability to verify a block.

What if there are more demands for the services than the network can supply?

You have two ways adapt the scarcity:

- **Cap the quantity**: You tell people to wait. This is like a Disney park. You queue up for the ride when the facility is at capacity.
- **Raise the price**: You offer resources to the highest paying users.

Ethereum applies a mixture of both.

Before we come back to that question, let's answer another: is there a way we can measure the size of the service?

## Ethereum measure its resources with gas

Think about how AWS bill your cloud usage? It bills CPU usage by hours, depending on how powerful your computation instance is. It bills network usage by gauging how much data you have sent or received over the datacenter. It bills the disk usage by renting you a disk space.

AWS is able to measure the computation resources like that because they own the hardwares. They can fully trust the hours of CPU, data sent over wire, disk space used reported by the datacenter. Same can not be said for a peer to peer network consisting of strangers' computers.

The computation usuage must be measured in a way the protocol can form a consensus on.

A user's computation request is represented with the transaction it sends. Some transactions are small, like a native Ether transfer, while some are big, like a complicated Defi swap. Each of it can be breakdown to tiny instructions in EVM to carry out the computation. EVM instruction is like a price table and provides gas metering.

- A simple bitwise operation is 1 gas
- reading or loading from storage is 2100 gas
- compute a Keccak hash function is 36 gas

The cost of data is not reflected in the instructions. They are reflected in two places:

- Transaction Calldata: Data attached on the transaction to involk a function call on a contract. This data is charged by bytes.
- Blob: As Ethereum recognize offering more network load is a viable scaling strategy, blob is a special object to carry data but not accessible on EVM.

Gas are constantly benchmarked and repriced. An opcode is priced at X gas, because assuming an attacker packed a full load of block using only this opcode, a full node still able to re-execute this block within the blocktime.

Reader might notice that CPU, Network bandwidth, and storage are all priced with the same unit gas, despite their very different nature. And defining the gas cost in the worst case is not an efficient way of utilizing resouces. For example, if we say in 12 second blocktime, the system can process X ADD opcode and Y GB of data, so we define a gas cost x and y for each unit of them. But in average case -- spending the block limit with half ADD opcode and half data -- is wasteful, because more ADD in the block doesn't impede its ability to carry data.

In the past, the reason to share CPU, Network bandwidth, and storage with same metering unit is for better packing decision for miners and better user experiences. But in the future, Ethereum will introduce new units of gas for each type of resources.

## Gas price: How much would you pay per gas?

After determining the size of the work, user can determine how eager they desire their work to be done.

The system has a hard and a soft limit on the blocksize. A block can't take more than the hard limit. But the soft limit allows the system to guage the demand for the service. 

The protocol sets a base fee, a portion that get burnt. If the protocol sees a higher demand, a block is more than half full, it raise the base fee; if the block has less than half full, it lowers the base fee.



## Breaking down your ERC20 Transaction

## Who gets all the fees?

## What about memories?

