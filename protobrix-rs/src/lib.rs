// Generated protobuf code
#[allow(clippy::all)]
pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/main_element.rs"));
}

// Error types
pub mod error;

// Builder pattern wrappers
pub mod builders;

// Actix-web integration
#[cfg(feature = "actix")]
pub mod actix_integration;

// Re-export commonly used types
pub use builders::*;
pub use error::ProtobrixError;
pub use proto::*;

#[cfg(feature = "actix")]
pub use actix_integration::*;
