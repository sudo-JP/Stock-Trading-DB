use crate::protocols::BinaryMessage;

pub trait CppController<T> {
    fn handle_operation(&self, bn: BinaryMessage, model: T);
}
