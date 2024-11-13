// src/lib/lib.rs

// module declarations
pub mod error;
pub mod routes;
pub mod service;
pub mod telemetry;

// re-exports
pub use error::*;
pub use service::*;
pub use telemetry::*;
