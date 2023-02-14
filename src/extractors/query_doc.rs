use std::str::FromStr;

use axum::{
    async_trait,
    extract::{FromRequestParts, Query},
    http::request::Parts,
};
use mongodb::bson::{doc, oid::ObjectId, Document};
use serde::{Deserialize, Serialize};

use crate::{error::Error, SharedState};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserQuery {
    pub id: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub created_at: Option<i64>,
    pub updated_at: Option<i64>,
}

pub struct UserQueryDocument(pub Document);

#[async_trait]
impl FromRequestParts<SharedState> for UserQueryDocument {
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &SharedState,
    ) -> Result<Self, Self::Rejection> {
        let Query(query) = Query::<UserQuery>::from_request_parts(parts, state).await?;

        let mut query_doc = doc! {};

        if let Some(id) = query.id {
            let oid = ObjectId::from_str(&id)?;
            query_doc.insert("_id", oid);
        };

        if let Some(username) = query.username {
            query_doc.insert("username", username);
        }

        if let Some(email) = query.email {
            query_doc.insert("email", email);
        }

        if let Some(created_at) = query.created_at {
            query_doc.insert("createdAt", created_at);
        }

        if let Some(updated_at) = query.updated_at {
            query_doc.insert("updatedAt", updated_at);
        }

        Ok(Self(query_doc))
    }
}
