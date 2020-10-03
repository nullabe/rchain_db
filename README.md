# rchain_db
Coin transaction blockchain developed in rust

## Implemented features

* RESTful HTTP Api (GET & POST): /blocks, /transactions
* Decentralized with nodes
* Proof of work block validation
* Mining a new block rewards 1 coin
* File storage to save state

## Crate used

* Tide
* Serde
* JSON

## Would be cool to implement

The project is fully functional and you can to use it in a container environment to deploy multiple instances and play ith consensus algo.

To permit this with docker
* Basic dockerfile with unix image
* Mount binary into
* Create a make directive to push a new blockchain instance thought a docker container given a port binding to 8880
* Register node using route POST /nodes
* Profit with your postman

## Special thanks

* https://hackernoon.com/learn-blockchains-by-building-one-117428612f46 (main inspiration from Python code)
* The book ! https://doc.rust-lang.org/book/
