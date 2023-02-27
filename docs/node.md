
# Node Structure

Node has seven components

## cli
cli uses clap to define subcommand structure

## command 
command define the command action for each subcommand
Command load chain spec 
### load chain_spec

## service - Build node

Build node is based on chain_spec loaded in command

Three high level steps to build new node.


### create partial node -
partial node includes the following basic objects

 #### Telemetry
 Telemetry is for monitoring

 #### client and executor
 client includes executor, which execute runtime

 #### keystore

#### backend
backend includes persistent database to store blockchain


#### transaction pool

#### import queue

#### select chain


### full node 
full node has two parts:

#### (1) build network
build network includes
##### network
##### rpc handler
##### tx handler

#### (2) create consensus

to create consensus, it depends on what kind of node to create

##### parachain

##### relaychain

##### solochain






