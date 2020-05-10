use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

use crate::error::response::ErrorResponse;

const FIELDS_COUNT: usize = 2;

impl Serialize for ErrorResponse {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut error_response =
            serializer.serialize_struct("ErrorResponse", self::FIELDS_COUNT)?;

        error_response
            .serialize_field("message", self.message())
            .ok();
        error_response
            .serialize_field("status_code", &self.status_code())
            .ok();

        error_response.end()
    }
}
