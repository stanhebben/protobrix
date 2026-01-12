use actix_web::{FromRequest, HttpRequest};
use futures_util::StreamExt;
use prost::Message;
use std::future::Future;
use std::pin::Pin;

use crate::error::ProtobrixError;
use crate::proto::AdvancedTableRequest;

impl FromRequest for AdvancedTableRequest {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut actix_web::dev::Payload) -> Self::Future {
        let content_type = req
            .headers()
            .get(actix_web::http::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_lowercase();

        let mut payload = payload.take();

        Box::pin(async move {
            // Collect the payload bytes
            let mut body = actix_web::web::BytesMut::new();
            while let Some(chunk) = payload.next().await {
                let chunk = chunk.map_err(|e| {
                    actix_web::error::ErrorBadRequest(ProtobrixError::Payload(format!(
                        "Failed to read payload: {}",
                        e
                    )))
                })?;
                body.extend_from_slice(&chunk);
            }

            let bytes = body.freeze();

            // Deserialize based on content type
            if content_type.contains("application/json") {
                serde_json::from_slice(&bytes)
                    .map_err(|e| actix_web::error::ErrorBadRequest(ProtobrixError::Json(e)))
            } else if content_type.contains("application/x-protobuf")
                || content_type.contains("application/protobuf")
            {
                AdvancedTableRequest::decode(bytes).map_err(|e| {
                    actix_web::error::ErrorBadRequest(ProtobrixError::ProtobufDecode(e))
                })
            } else if content_type.is_empty() {
                Err(actix_web::error::ErrorNotAcceptable(
                    ProtobrixError::MissingContentType,
                ))
            } else {
                Err(actix_web::error::ErrorNotAcceptable(
                    ProtobrixError::UnsupportedContentType(content_type),
                ))
            }
        })
    }
}
