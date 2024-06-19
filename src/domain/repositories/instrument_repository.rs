use crate::domain::entities::instrument::{InstrumentRequest, InstrumentResponse};
use async_trait::async_trait;

#[async_trait]
pub trait InstrumentRepository {
    async fn get_instruments(&self, data: InstrumentRequest) -> Result<InstrumentResponse, String>;
}
