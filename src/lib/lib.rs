// src/lib/lib.rs

// module declarations
pub mod config;
pub mod error;
pub mod renderer;
pub mod routes;
pub mod service;
pub mod telemetry;

// re-exports
pub use config::*;
pub use error::*;
pub use renderer::*;
pub use service::*;
pub use telemetry::*;
