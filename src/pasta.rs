use crate::error::ApiError;
use crate::{extractor::Json, AppState};
use axum::http::HeaderMap;
use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use base64ct::{Base64, Encoding};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
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
pub struct NewPastaPayload {
    title: String,
    content: String,
    slug: Option<String>,
    view_key: Option<String>,
    edit_key: Option<String>,
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

    let pasta = result.ok_or_else(|| {
        ApiError::new(StatusCode::NOT_FOUND, "Pasta not found.")
    })?;

    if let Some(stored_key) = pasta.view_key {
        let provided_key = headers.get("X-View-Key").ok_or_else(|| {
            ApiError::new(StatusCode::BAD_REQUEST, "View key is required.")
        })?;

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

pub async fn add_pasta(
    State(state): State<Arc<AppState>>,
    Json(pasta): Json<NewPastaPayload>,
) -> Result<Json<Value>, ApiError> {
    const DEFAULT_SLUG_LEN: usize = 8;
    let slug = pasta.slug.unwrap_or(nanoid!(DEFAULT_SLUG_LEN));
    if slug.len() < DEFAULT_SLUG_LEN {
        return Err(ApiError::new(
            StatusCode::BAD_REQUEST,
            "Slug must at least be 8 characters long.",
        ));
    }

    let is_exists = sqlx::query("SELECT 1 FROM pasta WHERE slug = ?")
        .bind(&slug)
        .fetch_optional(&state.db)
        .await
        .unwrap();

    if is_exists.is_some() {
        return Err(ApiError::new(
            StatusCode::BAD_REQUEST,
            "Pasta with the same slug already exist.",
        ));
    }

    let view_key = pasta
        .view_key
        .map(|key| Base64::encode_string(&Sha256::digest(key)));
    let edit_key = pasta
        .edit_key
        .map(|key| Base64::encode_string(&Sha256::digest(key)));

    let last_insert_id = sqlx::query(
        r#"
INSERT INTO pasta (title, content, slug, view_key, edit_key)
VALUES (?, ?, ?, ?, ?)
        "#,
    )
    .bind(pasta.title)
    .bind(pasta.content)
    .bind(slug)
    .bind(view_key)
    .bind(edit_key)
    .execute(&state.db)
    .await
    .unwrap()
    .last_insert_id();

    let new_pasta = sqlx::query_as::<_, Pasta>(
        r#"
SELECT id, title, content, slug, view_key, edit_key
FROM pasta
WHERE id = ?
        "#,
    )
    .bind(last_insert_id)
    .fetch_one(&state.db)
    .await
    .unwrap();

    Ok(Json(json!({
        "id": new_pasta.id,
        "title": new_pasta.title,
        "content": new_pasta.content,
        "slug": new_pasta.slug
    })))
}
