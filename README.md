# lbry-rs

A Rust API wrapper for lbrynet

## Installation

### From crates

1. Add the following line to your `Cargo.toml` dependencies

```toml
lbry-rs = "0.0.1"
```

### From source

1. Clone the repository

```bash
$ git clone https://github.com/zxawry/lbry-rs
```

2. Add the following line to your `Cargo.toml` dependencies

```toml
lbry-rs = {path = "{path-to-where-you-cloned-the-repo}"}
```

## Usage

```rust
extern crate lbry_rs;

use lbry_rs::LbrynetApi;

//re-exported from serde_json
use lbry_rs::{Value, json};

fn main() {
    //instantiate a lbry client
    let mut lbry = LbrynetApi::new();

    //make requests by providing the method and params with cURL command-line syntax
    //params must be of the type serde_json::Value so use json! macro for conversion
    let result0: Value = lbry.call("status", json!({}));
    let result1: Value = lbry.call("resolve", json!({"urls":"lbrytv"}));
        
    println!("{}", result0);
    println!("{}", result1);
}
```

Note that `lbrynet` must be up and running, to use this API wrapper.

## Links

* LBRY project: https://github.com/lbryio
* LBRY official website: https://lbry.com
* LBRYnet API reference: https://lbry.tech/api/sdk

## License

MIT License
