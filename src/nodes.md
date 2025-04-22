---
status: idea
---

# Running a full node

Running a full node is the best way to understand the blockchain.

It takes lots of efforts for technical unsavvy people, but a very rewarding one.

But if you don't have time, I'll describe.


Ethereum is computers with a hive mind. Computers from strangers form a network, each contributes their CPU run time, memory space, and storage.

A full node is a computer who runs an Ethereum client -- a software that synchronizes states from the network and validates blocks.

Currently, an Ethereum full node needs two clients to work: an execution client and an consensus client. Each of them has different implementations.

We'll use Reth as execution client and Nimbus as consensus client as example.

You start both clients and configured a way for them to communicate.

The goal of your node is to acquire the latest Ethereum state, then wait for new blocks and validate them. To acheive that, your node need to:

- Get some friends who have historical data
- Get data from friends


The execution client will first look for hardcoded bootstrap nodes -- nodes maintained by Ethereum Foundation and various client teams -- who introduces peers in the network to your node.

The client then follows a "Discovery" protocol and knock on the peers, who might share blocks with you or introduce you to their peers, and then finally settle with a fixed number of peers in the network.

...

Now your node has become the one of the network, forever. You can shut the node down if you no longer like to do so.