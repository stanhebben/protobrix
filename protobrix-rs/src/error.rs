#[derive(Debug, thiserror::Error)]
pub enum ProtobrixError {
    #[error("Protobuf decode error: {0}")]
    ProtobufDecode(#[from] prost::DecodeError),

    #[error("Protobuf encode error: {0}")]
    ProtobufEncode(#[from] prost::EncodeError),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Unsupported content type: {0}")]
    UnsupportedContentType(String),

    #[error("Missing content type header")]
    MissingContentType,

    #[error("Payload error: {0}")]
    Payload(String),

    #[error("Builder error: {0}")]
    Builder(String),
}

// Actix-web integration
#[cfg(feature = "actix")]
impl actix_web::ResponseError for ProtobrixError {
    fn error_response(&self) -> actix_web::HttpResponse {
        use actix_web::HttpResponse;
        use actix_web::http::StatusCode;

        match self {
            ProtobrixError::ProtobufDecode(_) | ProtobrixError::Json(_) => {
                HttpResponse::build(StatusCode::BAD_REQUEST).body(format!("Bad Request: {}", self))
            }
            ProtobrixError::UnsupportedContentType(_) | ProtobrixError::MissingContentType => {
                HttpResponse::build(StatusCode::NOT_ACCEPTABLE)
                    .body(format!("Not Acceptable: {}", self))
            }
            ProtobrixError::Payload(msg) => {
                HttpResponse::build(StatusCode::BAD_REQUEST).body(format!("Payload Error: {}", msg))
            }
            ProtobrixError::ProtobufEncode(_) | ProtobrixError::Builder(_) => {
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(format!("Internal Server Error: {}", self))
            }
        }
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        use actix_web::http::StatusCode;

        match self {
            ProtobrixError::ProtobufDecode(_)
            | ProtobrixError::Json(_)
            | ProtobrixError::Payload(_) => StatusCode::BAD_REQUEST,
            ProtobrixError::UnsupportedContentType(_) | ProtobrixError::MissingContentType => {
                StatusCode::NOT_ACCEPTABLE
            }
            ProtobrixError::ProtobufEncode(_) | ProtobrixError::Builder(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}
