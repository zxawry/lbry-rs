# lbry-rs
a Rust API Wrapper for lbrynet

## Installation

### With cargo

It's not a published cargo yet, so in order to use it:

1. Manually Clone the Repository

```bash
$ git clone https://github.com/zxawry/lbry-rs
```

2. Add the following line to your Cargo.toml dependencies
```toml
lbry-rs = {path = "/{path-to-where-you-cloned-the-repo}"}
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
    let result1: Value = lbry.call("resolve", json!({"uri":"lbrytv"}));
    let result2: Value = lbry.call("resolve_name", json!({"name":"lbrytv","force":true}));
        
    println!("{}", result0);
    println!("{}", result1);
    println!("{}", result2);
}
```

Note that `lbrynet` must be up and running, to use the API.

## License
MIT License
