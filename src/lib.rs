// TODO: fail gracefully, use errors
// TODO: docs
#![feature(int_log)]
#![cfg_attr(
    debug_assertions,
    allow(clippy::missing_errors_doc, clippy::missing_panics_doc)
)]
pub mod code;
mod decode;
mod encode;
mod error;

pub use code::Code;
pub use decode::decode;
pub use encode::encode;
pub use error::HammingError as Error;
