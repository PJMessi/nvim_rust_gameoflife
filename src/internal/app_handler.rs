use rmpv::Value;

pub trait AppHandler {
    fn handle(&mut self) -> Result<Value, Value>;
}
