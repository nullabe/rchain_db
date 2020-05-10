# rchain_db
Coin transaction blockchain developed in rust

## Implemented features

* RESTful HTTP Api (GET & POST): /blocks, /transactions
* Decentralized with nodes
* Proof of work block validation
* Mining a new block rewards 1 coin
* File storage to save state

## Not implemented
#### Transform this into a non-relational blockchain database

* Use of named registry (like phone_list, addresses, ...), any of thoses are blockchains liked to a master chain
* Each block contain his own json struct type (permits handling data versionning)
* Full JSON search (with lucent ?)
* Pagination with blocks chuncks
