# rchain_db
Coin transaction blockchain developed in rust

## Implemented features

* RESTful HTTP Api (GET & POST)
    * GET /blockchain
        * Full blockchain data
    * GET /transactions/to_process
        * All transactions that will be added to the next block
    * POST /blocks
        * Mine a new block
    * POST /nodes
        * register a new node ("uuid": string, "url": string)
    * POST /nodes/resolve
        * Resolve conflicts between nodes by choosing the longest valid blockchain (consensus)
    * POST /transaction
        * add a new transaction ("amount": float, "receiver": string, "sender": string)

* Decentralized (nodes & consensus)
* Proof of work block validation
* Rewards for mining
* File storage to save state

## Crates used

* Tide
* Serde
* JSON

## Would be cool to implement

The project is fully functional and you can use it in a container environment to deploy multiple instances and play with consensus algo.

To permit this with docker
* Basic dockerfile
* Mount binary into
* Create a make directive to push a new blockchain instance thought a docker container given a port binding to 8880
* Register node using route POST /nodes
* Profit with your postman

## Special thanks

* https://hackernoon.com/learn-blockchains-by-building-one-117428612f46 (main inspiration from Python code)
* The book ! https://doc.rust-lang.org/book/
