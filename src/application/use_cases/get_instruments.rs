use crate::domain::entities::instrument::{InstrumentRequest, InstrumentResponse};
use crate::domain::repositories::instrument_repository::InstrumentRepository;

pub struct GetInstrumentsUseCase<R: InstrumentRepository> {
    pub repository: R,
}

impl<R: InstrumentRepository> GetInstrumentsUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, data: InstrumentRequest) -> Result<InstrumentResponse, String> {
        self.repository.get_instruments(data).await
    }
}
