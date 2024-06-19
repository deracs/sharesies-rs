use crate::domain::{
    entities::instrument::{InstrumentRequest, InstrumentResponse},
    errors::SharesiesError,
};

use super::sharesies::Sharesies;

impl Sharesies {
    pub async fn get_instruments(
        &self,
        data: InstrumentRequest,
    ) -> Result<InstrumentResponse, SharesiesError> {
        self.get_instruments_use_case.execute(data).await
    }
}
