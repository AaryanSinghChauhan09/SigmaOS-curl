//! SigmaOS-curl — HTTP client model with multiple protocols.
//!
//! Models HTTP/HTTPS/FTP requests, responses, authentication,
//! cookies, and transfer options. Deterministic and std-only.

pub mod client;
pub mod request;
pub mod response;
pub mod auth;
pub mod cookie;

pub use client::{CurlClient, CurlOption};
pub use request::{CurlRequest, CurlMethod};
pub use response::{CurlResponse, CurlStatus};
pub use auth::{AuthMethod, Credentials};
pub use cookie::{Cookie, CookieJar};
