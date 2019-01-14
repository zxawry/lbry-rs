use serde_json::Value;

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

