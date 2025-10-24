use crate::protocols::CppBinaryMessage;

pub trait CppController<T, R> {
    async fn handle_operation(&self, bn: CppBinaryMessage, model: T) -> Result<R, sqlx::Error>;
}
