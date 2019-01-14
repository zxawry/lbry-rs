extern crate serde_json;
extern crate jsonrpc_client_core;
extern crate jsonrpc_client_http;

pub use serde_json::{Value, Map, Number};
pub use serde_json::json;

pub mod lbrynet_api;

#[cfg(test)]
mod tests {
    use crate::lbrynet_api::LbrynetApi;
    use serde_json::Value;
    use serde_json::json;
    #[test]
    fn it_works() {
        let mut lbry = LbrynetApi::new();
        let result0: Value = lbry.call("status", json!({}));
        let result1: Value = lbry.call("resolve", json!({"uri":"lbrytv"}));
        
        assert!(result0.is_object(), "{}", result0);
        assert!(result1.is_object(), "{}", result1);
    }
}

