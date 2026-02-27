use axum::{
    extract::{State, Multipart},
    Json,
    http::StatusCode,
};
use serde_json::{json, Value};
use uuid::Uuid;
use aws_sdk_s3::primitives::ByteStream;
use crate::AppState;

pub async fn upload_image(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<Value>, StatusCode> {
    while let Some(field) = multipart.next_field().await.map_err(|_| StatusCode::BAD_REQUEST)? {
        let name = field.name().unwrap_or("").to_string();
        if name == "file" {
            let data = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?;
            let filename = format!("books/{}.webp", Uuid::new_v4());

            let aws_config = aws_config::load_from_env().await;
            let s3_client = aws_sdk_s3::Client::new(&aws_config);

            s3_client
                .put_object()
                .bucket(&state.config.aws_s3_bucket)
                .key(&filename)
                .body(ByteStream::from(data))
                .content_type("image/webp")
                .send()
                .await
                .map_err(|e| {
                    tracing::error!("S3 upload error: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                })?;

            let url = format!(
                "https://{}.s3.{}.amazonaws.com/{}",
                state.config.aws_s3_bucket,
                state.config.aws_region,
                filename
            );

            return Ok(Json(json!({ "url": url })));
        }
    }
    Err(StatusCode::BAD_REQUEST)
}
