// src/lib/routes/mod.rs

// module declarations
pub mod health;
pub mod home;
pub mod openapi;
pub mod storage;

// re-exports
pub use health::*;
pub use home::*;
pub use openapi::*;
pub use storage::*;
