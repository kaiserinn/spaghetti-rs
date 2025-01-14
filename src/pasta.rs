use crate::error::ApiError;
use crate::{extractor::Json, AppState};
use axum::http::HeaderMap;
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
}

pub async fn get_pasta(
    Path(slug): Path<String>,
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
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
        let provided_key = headers.get("X-View-Key")
            .ok_or_else(|| ApiError::new(
                StatusCode::BAD_REQUEST,
                "View key is required."
            ))?;

        let key = Base64::encode_string(&Sha256::digest(provided_key));
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
