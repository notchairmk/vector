//! The Datadog Logs [`VectorSink`]
//!
//! This module contains the [`VectorSink`] instance that is responsible for
//! taking a stream of [`Event`] instances and getting them flung out to the
//! Datadog Log API. The log API is relatively generous in terms of its
//! constraints, except that:
//!
//!   * a 'payload' is comprised of no more than 1,000 array members
//!   * a 'payload' may not be more than 5Mb in size, uncompressed and
//!   * a 'payload' may not mix API keys
//!
//! Otherwise per [the
//! docs](https://docs.datadoghq.com/api/latest/logs/#send-logs) there aren't
//! other major constraints we have to follow in this implementation. The sink
//! is careful to always send the maximum payload size excepting where we
//! violate the size constraint.
//!
//! The endpoint used to send the payload is currently being migrated from
//! `/v1/input` to `/api/v2/logs`, but the content of the above documentation
//! still applies for `/api/v2/logs`.

#[cfg(all(test, feature = "datadog-logs-integration-tests"))]
mod integration_tests;
#[cfg(test)]
mod tests;

mod config;
mod service;
mod sink;

pub(crate) use config::DatadogLogsConfig;

use crate::config::SinkDescription;

inventory::submit! {
    SinkDescription::new::<DatadogLogsConfig>("datadog_logs")
}
