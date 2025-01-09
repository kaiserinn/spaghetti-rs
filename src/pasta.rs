use crate::AppState;
use axum::{
    extract::{rejection::JsonRejection, Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
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
    payload: Result<Json<GetPastaPayload>, JsonRejection>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let result = sqlx::query_as::<_, Pasta>(
        "SELECT id, title, content, slug, view_key, edit_key FROM pasta WHERE slug = ?",
    )
    .bind(slug)
    .fetch_optional(&state.db)
    .await
    .unwrap();

    let pasta = result.ok_or_else(|| (StatusCode::NOT_FOUND, String::from("Pasta not found.")))?;

    if let Some(key) = pasta.view_key {
        let payload = payload
            .map_err(|_| {
                (
                    StatusCode::BAD_REQUEST,
                    String::from("View key is required."),
                )
            })?
            .0;

        if payload.view_key != key {
            return Err((
                StatusCode::BAD_REQUEST,
                String::from("View key is invalid."),
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
