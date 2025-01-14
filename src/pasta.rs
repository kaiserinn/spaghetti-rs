use crate::error::ApiError;
use crate::{extractor::Json, AppState};
use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use base64ct::{Base64, Encoding};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Sha256, Digest};
use sqlx::FromRow;
use std::sync::Arc;

#[derive(FromRow, Serialize, Debug)]
struct Pasta {
    id: u32,
    title: String,
    content: String,
    slug: String,
    view_key: Option<String>,
    edit_key: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct GetPastaPayload {
    view_key: String,
}

pub async fn get_pasta(
    Path(slug): Path<String>,
    State(state): State<Arc<AppState>>,
    payload: Result<Json<GetPastaPayload>, ApiError>,
) -> Result<Json<Value>, ApiError> {
    let result = sqlx::query_as::<_, Pasta>(
        r#"
SELECT id, title, content, slug, view_key, edit_key
FROM pasta
WHERE slug = ?
        "#,
    )
    .bind(slug)
    .fetch_optional(&state.db)
    .await
    .unwrap();

    let pasta = result.ok_or_else(|| ApiError::new(
        StatusCode::NOT_FOUND,
        "Pasta not found.")
    )?;

    if let Some(stored_key) = pasta.view_key {
        let payload = payload
            .map_err(|_| ApiError::new(
                StatusCode::BAD_REQUEST,
                "View key is required."
            ))?
            .0;

        let key = Base64::encode_string(&Sha256::digest(payload.view_key));
        if key != stored_key {
            return Err(ApiError::new(
                StatusCode::BAD_REQUEST,
                "View key is invalid.",
            ));
        }
    }

    Ok(Json(json!({
        "id": pasta.id,
        "title": pasta.title,
        "content": pasta.content,
        "slug": pasta.slug
    })))
}
