#![warn(missing_docs)]
//! gnostr: hypbrid git+nostr relay
//!
/// pub mod port_selector;
pub mod port_selector;
/// pub mod context;
pub mod context;
/// pub mod handler;
pub mod handler;
/// pub mod route;
pub mod route;
/// pub mod router;
pub mod router;
/// pub type Response = hyper::Response<hyper::Body>;
pub type Response = hyper::Response<hyper::Body>;
/// pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
