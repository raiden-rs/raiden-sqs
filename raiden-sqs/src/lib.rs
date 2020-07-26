pub use raiden_sqs_derive::*;
pub use rusoto_credential::*;

#[cfg(feature = "default")]
use rusoto_core_default as rusoto_core;
#[cfg(feature = "rustls")]
use rusoto_core_rustls as rusoto_core;

pub use rusoto_core::*;

#[cfg(feature = "default")]
use rusoto_sqs_default as rusoto_sqs;
#[cfg(feature = "rustls")]
use rusoto_sqs_rustls as rusoto_sqs;

pub use rusoto_sqs::*;
