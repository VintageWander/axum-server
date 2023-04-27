use std::str::FromStr;

use axum::{
    async_trait,
    extract::{FromRequestParts, Path},
    http::request::Parts,
};
use mongodb::bson::oid::ObjectId;

use crate::{error::Error, service::Service};

pub struct ParamID(pub ObjectId);

#[async_trait]
impl FromRequestParts<Service> for ParamID {
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Service,
    ) -> Result<Self, Self::Rejection> {
        let Path(id) = Path::<String>::from_request_parts(parts, state).await?;

        let id = ObjectId::from_str(&id)?;

        Ok(Self(id))
    }
}
