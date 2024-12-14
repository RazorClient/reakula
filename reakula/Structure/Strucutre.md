# Layout 

- Specs : Tamaghna 
- Networking : Bartik + Sawan
- DB : Tamaghna 
- codecs/types - bhaskr

maharshi+ others -> sp1 helios,helios and trin (tamaghna will look into trin too)

maybe ask kolby ??idk
rupam -> lighthouse lightclient writer so can help 

get stuff ready in a week 

# Goal 
Get the skeleton for consensus light client to stand in 2 months 
then iterz

## Number of sub crates ??
 I say 4-5
 - specs

 - db (eihter rocksdb or mdbx)
    - https://erthink.github.io/libmdbx/
    - https://docs.rs/rocksdb/latest/rocksdb/

    - set the key value store etc etc mpt->db figure out in 7 days
    - read the nimbus db engine and lodestar/lighthouse too!

 - networking 
    - look into specs
    - gossip handling 
    - pub/sub ??
    - discovery protocol 
    - every other thing 
    (basically libp2p)- will be hardest in entire project 

 - codecs (ssz , rlp ??)
    - we use alloy primitives (https://docs.rs/alloy-primitives/latest/alloy_primitives/)
    - sszb (https://docs.rs/sszb/latest/sszb/  https://github.com/ghiliweld/sszb)
    - finish up our libs o drop in as replacement to rlp 
    - 
- draft out the stcuture of the workspace in a week with the boilerplate 

- set up  ci to test on empty project 


## We follow lamdbaclass philosophy for project 

#next meet on 18th dec

