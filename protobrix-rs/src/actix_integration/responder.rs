use actix_web::{HttpRequest, HttpResponse, Responder};
use prost::Message;

use crate::error::ProtobrixError;
use crate::proto::MainElement;

impl Responder for MainElement {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        // Check Accept header
        let accept = req
            .headers()
            .get(actix_web::http::header::ACCEPT)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("*/*")
            .to_lowercase();

        // Determine response format based on Accept header
        let use_protobuf =
            accept.contains("application/x-protobuf") || accept.contains("application/protobuf");
        let use_json =
            accept.contains("application/json") || accept.contains("*/*") || accept.is_empty();

        if use_protobuf {
            // Serialize as protobuf
            let mut buf = Vec::new();
            match self.encode(&mut buf) {
                Ok(_) => HttpResponse::Ok()
                    .content_type("application/x-protobuf")
                    .body(buf),
                Err(e) => {
                    let error = ProtobrixError::ProtobufEncode(e);
                    HttpResponse::InternalServerError()
                        .body(format!("Failed to encode protobuf: {}", error))
                }
            }
        } else if use_json {
            // Serialize as JSON
            match serde_json::to_string(&self) {
                Ok(json) => HttpResponse::Ok()
                    .content_type("application/json")
                    .body(json),
                Err(e) => {
                    let error = ProtobrixError::Json(e);
                    HttpResponse::InternalServerError()
                        .body(format!("Failed to encode JSON: {}", error))
                }
            }
        } else {
            // Unsupported Accept header - default to JSON
            match serde_json::to_string(&self) {
                Ok(json) => HttpResponse::Ok()
                    .content_type("application/json")
                    .body(json),
                Err(e) => {
                    let error = ProtobrixError::Json(e);
                    HttpResponse::InternalServerError()
                        .body(format!("Failed to encode JSON: {}", error))
                }
            }
        }
    }
}

/// Wrapper type for flexible response handling
/// Allows returning Result<ProtobrixResponse<MainElement>, Error> from handlers
#[derive(Debug, Clone)]
pub struct ProtobrixResponse<T>(pub T);

impl<T> ProtobrixResponse<T> {
    pub fn new(inner: T) -> Self {
        Self(inner)
    }
}

impl Responder for ProtobrixResponse<MainElement> {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        self.0.respond_to(req)
    }
}

impl<T> From<T> for ProtobrixResponse<T> {
    fn from(inner: T) -> Self {
        Self(inner)
    }
}
