use async_trait::async_trait;

#[async_trait]
pub trait Injector {
    async fn inject(&self, payload: Vec<Vec<u8>>) -> Result<(), ()>;
}
