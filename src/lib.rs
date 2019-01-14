extern crate serde_json;
extern crate jsonrpc_client_core;
extern crate jsonrpc_client_http;

pub use serde_json::{Value, Map, Number};
pub use serde_json::json;

use jsonrpc_client_core::call_method;

use jsonrpc_client_http::HttpHandle;
use jsonrpc_client_http::HttpTransport;

const LBRYNET_SERVER_ADDRESS: &str = "http://localhost:5279";

pub struct LbrynetApi {
  transport_handle: HttpHandle,
}

impl LbrynetApi {
  pub fn new() -> Self {
    let transport = HttpTransport::new().standalone().unwrap();
    LbrynetApi {transport_handle: transport.handle(LBRYNET_SERVER_ADDRESS).unwrap(),}
  }
  pub fn call(&mut self, method: &str, params: Value) -> Value {
    call_method(&mut self.transport_handle, method.to_string(), params).call().unwrap()
  }
}

#[cfg(test)]
mod tests {
    use crate::LbrynetApi;
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

