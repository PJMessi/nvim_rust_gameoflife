use rmpv::Value;

use crate::internal::app_handler::AppHandler;

pub struct PingPongHandler {}

impl PingPongHandler {
    pub fn new() -> Self {
        PingPongHandler {}
    }
}

impl AppHandler for PingPongHandler {
    fn handle(&mut self) -> Result<Value, Value> {
        Ok(Value::from("pong"))
    }
}
