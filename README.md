# cosmos-sdk-historical-query

Sample code for querying a Cosmos chain at a historical block height, in four languages:

- Rust
- Go
- JavaScript
- Bash

In addition, the example persented here is a paginated query, meaning it may take more than one query to retrieve the full result. We show how to do this using a loop.

## How to use

Rust:

```bash
cd rust
cargo run
```

Go:

```bash
cd go
go run main.go
```

JavaScript:

```bash
cd js
node main.js
```

Bash:

```bash
cd bash
bash main.sh
```

Result should look like this:

```plain
performing query... next=""
performing query... next="14614fecb276f5e00c2fd0db40130023bb3f832175"
performing query... next="14ab2bc85eaa365c4cf39f6a7525b44fef8264512e"
performing query... next="14dbfdb0af1e9dc4a05a94074e6c900b29550b2dad"
[
  {
    "delegation": {
      "delegator_address": "celestia15rpm3yhl76ps7s74nu5pg06atpz70slal4kdk2",
      "validator_address": "celestiavaloper1qx43f066sh6728avms4qq09cj2a3mg83dgjh22",
      "shares": "350000000000.000000000000000000"
    },
    "balance": {
      "denom": "utia",
      "amount": "350000000000"
    }
  },
  {
    "delegation": {
      "delegator_address": "celestia15rpm3yhl76ps7s74nu5pg06atpz70slal4kdk2",
      "validator_address": "celestiavaloper1qxeza0sa037u35p3ze8p7ka7emajvydnyjlp07",
      "shares": "350000000000.000000000000000000"
    },
    "balance": {
      "denom": "utia",
      "amount": "350000000000"
    }
  },
...more
```

If the node has already pruned the height you're querying at, an error like this should be returned:

```plain
codespace: sdk
code: 18
log: failed to load state at height 123; version mismatch on immutable IAVL tree; version does not exist. Version has either been pruned, or is for a future block height (latest height: 344122): invalid request
```
