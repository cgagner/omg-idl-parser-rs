# OMG IDL Parser
A pure rust implementation of an OMG Interface Definition Language (IDL) 
version 4.2. For more information, see [OMG IDL 4.2](https://www.omg.org/spec/IDL/4.2/PDF).


## Build Test Cases
This repository utilizes [eProsima DDS Type Test](git remote add origin git@github.com:cgagner/omg-idl-parser-rs.git) as an optional submodule for testing. To enable testing with that repository, use:

```bash
git submodule update --init
cargo test
```

***Work in Progress***: This repository is not complete. At the time of 
writing this, the parse can only validate an IDL file. 

