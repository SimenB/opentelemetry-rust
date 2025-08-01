//! OpenTelemetry semantic conventions are agreed standardized naming patterns
//! for OpenTelemetry things. This crate aims to be the centralized place to
//! interact with these conventions.
#![warn(
    future_incompatible,
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    unreachable_pub,
    unused
)]
#![cfg_attr(test, deny(warnings))]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/open-telemetry/opentelemetry-rust/main/assets/logo.svg"
)]

pub mod attribute;
pub mod metric;
pub mod resource;
pub mod trace;

/// The schema URL that matches the version of the semantic conventions that
/// this crate defines.
pub const SCHEMA_URL: &str = "https://opentelemetry.io/schemas/1.36.0";
