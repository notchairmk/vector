pub mod prelude;

mod adaptive_concurrency;
mod aggregate;
#[cfg(feature = "sources-apache_metrics")]
mod apache_metrics;
#[cfg(feature = "api")]
mod api;
#[cfg(feature = "aws-core")]
mod aws;
#[cfg(feature = "sinks-aws_cloudwatch_logs")]
mod aws_cloudwatch_logs;
#[cfg(feature = "transforms-aws_ec2_metadata")]
mod aws_ec2_metadata;
#[cfg(feature = "sources-aws_ecs_metrics")]
mod aws_ecs_metrics;
#[cfg(feature = "sources-aws_kinesis_firehose")]
mod aws_kinesis_firehose;
#[cfg(any(feature = "sources-aws_s3", feature = "sources-aws_sqs",))]
mod aws_sqs;
#[cfg(any(feature = "sinks-azure_blob", feature = "sinks-datadog_archives"))]
pub(crate) mod azure_blob;
mod batch;
mod codecs;
mod common;
mod conditions;
#[cfg(feature = "sinks-datadog_metrics")]
mod datadog_metrics;
#[cfg(feature = "sinks-datadog_traces")]
mod datadog_traces;
#[cfg(feature = "transforms-dedupe")]
mod dedupe;
#[cfg(feature = "sources-demo_logs")]
mod demo_logs;
#[cfg(feature = "sources-dnstap")]
mod dnstap;
#[cfg(feature = "sources-docker_logs")]
mod docker_logs;
mod elasticsearch;
mod encoding_transcode;
#[cfg(feature = "sources-eventstoredb_metrics")]
mod eventstoredb_metrics;
#[cfg(feature = "sources-exec")]
mod exec;
#[cfg(feature = "transforms-filter")]
mod filter;
#[cfg(feature = "sources-fluent")]
mod fluent;
#[cfg(feature = "sources-gcp_pubsub")]
mod gcp_pubsub;
#[cfg(feature = "transforms-geoip")]
mod geoip;
mod heartbeat;
mod http;
pub mod http_client;
#[cfg(feature = "sources-internal_logs")]
mod internal_logs;
#[cfg(all(unix, feature = "sources-journald"))]
mod journald;
#[cfg(any(feature = "sources-kafka", feature = "sinks-kafka"))]
mod kafka;
#[cfg(feature = "sources-kubernetes_logs")]
mod kubernetes_logs;
#[cfg(feature = "transforms-log_to_metric")]
mod log_to_metric;
mod logplex;
#[cfg(feature = "sinks-loki")]
mod loki;
#[cfg(feature = "transforms-lua")]
mod lua;
#[cfg(feature = "transforms-metric_to_log")]
mod metric_to_log;
#[cfg(feature = "sources-mongodb_metrics")]
mod mongodb_metrics;
#[cfg(feature = "sinks-nats")]
mod nats;
#[cfg(feature = "sources-nginx_metrics")]
mod nginx_metrics;
mod open;
#[cfg(any(
    feature = "sinks-datadog_events",
    feature = "sources-kubernetes_logs",
    feature = "transforms-geoip",
    feature = "transforms-log_to_metric",
))]
mod parser;
#[cfg(feature = "sources-postgresql_metrics")]
mod postgresql_metrics;
mod process;
#[cfg(any(feature = "sources-prometheus", feature = "sinks-prometheus"))]
mod prometheus;
#[cfg(any(feature = "sources-redis", feature = "sinks-redis"))]
mod redis;
#[cfg(feature = "transforms-reduce")]
mod reduce;
mod remap;
mod sample;
#[cfg(feature = "sinks-sematext")]
mod sematext_metrics;
mod socket;
#[cfg(any(feature = "sources-splunk_hec", feature = "sinks-splunk_hec"))]
mod splunk_hec;
#[cfg(feature = "sinks-statsd")]
mod statsd_sink;
#[cfg(feature = "sources-statsd")]
mod statsd_source;
#[cfg(feature = "sources-syslog")]
mod syslog;
#[cfg(feature = "transforms-tag_cardinality_limit")]
mod tag_cardinality_limit;
mod tcp;
mod template;
#[cfg(feature = "transforms-throttle")]
mod throttle;
mod udp;
mod unix;
mod vector;
#[cfg(feature = "sinks-websocket")]
mod websocket;

#[cfg(any(
    feature = "sources-file",
    feature = "sources-kubernetes_logs",
    feature = "sinks-file",
))]
mod file;
mod windows;

#[cfg(feature = "sources-mongodb_metrics")]
pub(crate) use mongodb_metrics::*;

#[cfg(feature = "transforms-aggregate")]
pub(crate) use self::aggregate::*;
#[cfg(feature = "sources-apache_metrics")]
pub(crate) use self::apache_metrics::*;
#[cfg(feature = "api")]
pub(crate) use self::api::*;
#[cfg(feature = "aws-core")]
pub(crate) use self::aws::*;
#[cfg(feature = "sinks-aws_cloudwatch_logs")]
pub(crate) use self::aws_cloudwatch_logs::*;
#[cfg(feature = "transforms-aws_ec2_metadata")]
pub(crate) use self::aws_ec2_metadata::*;
#[cfg(feature = "sources-aws_ecs_metrics")]
pub(crate) use self::aws_ecs_metrics::*;
#[cfg(feature = "sources-aws_kinesis_firehose")]
pub(crate) use self::aws_kinesis_firehose::*;
#[cfg(any(feature = "sources-aws_s3", feature = "sources-aws_sqs",))]
pub(crate) use self::aws_sqs::*;
pub(crate) use self::codecs::*;
#[cfg(feature = "sinks-datadog_metrics")]
pub(crate) use self::datadog_metrics::*;
#[cfg(feature = "sinks-datadog_traces")]
pub(crate) use self::datadog_traces::*;
#[cfg(feature = "transforms-dedupe")]
pub(crate) use self::dedupe::*;
#[cfg(feature = "sources-demo_logs")]
pub(crate) use self::demo_logs::*;
#[cfg(feature = "sources-dnstap")]
pub(crate) use self::dnstap::*;
#[cfg(feature = "sources-docker_logs")]
pub(crate) use self::docker_logs::*;
#[cfg(feature = "sinks-elasticsearch")]
pub(crate) use self::elasticsearch::*;
#[cfg(feature = "sources-eventstoredb_metrics")]
pub(crate) use self::eventstoredb_metrics::*;
#[cfg(feature = "sources-exec")]
pub(crate) use self::exec::*;
#[cfg(any(
    feature = "sources-file",
    feature = "sources-kubernetes_logs",
    feature = "sinks-file",
))]
pub(crate) use self::file::*;
#[cfg(feature = "transforms-filter")]
pub(crate) use self::filter::*;
#[cfg(feature = "sources-fluent")]
pub(crate) use self::fluent::*;
#[cfg(feature = "sources-gcp_pubsub")]
pub(crate) use self::gcp_pubsub::*;
#[cfg(feature = "transforms-geoip")]
pub(crate) use self::geoip::*;
#[cfg(any(
    feature = "sources-utils-http",
    feature = "sources-utils-http-encoding",
    feature = "sources-datadog_agent",
    feature = "sources-splunk_hec",
))]
pub(crate) use self::http::*;
#[cfg(feature = "sources-internal_logs")]
pub(crate) use self::internal_logs::*;
#[cfg(all(unix, feature = "sources-journald"))]
pub(crate) use self::journald::*;
#[cfg(any(feature = "sources-kafka", feature = "sinks-kafka"))]
pub(crate) use self::kafka::*;
#[cfg(feature = "sources-kubernetes_logs")]
pub(crate) use self::kubernetes_logs::*;
#[cfg(feature = "transforms-log_to_metric")]
pub(crate) use self::log_to_metric::*;
#[cfg(feature = "sources-heroku_logs")]
pub(crate) use self::logplex::*;
#[cfg(feature = "sinks-loki")]
pub(crate) use self::loki::*;
#[cfg(feature = "transforms-lua")]
pub(crate) use self::lua::*;
#[cfg(feature = "transforms-metric_to_log")]
pub(crate) use self::metric_to_log::*;
#[cfg(feature = "sinks-nats")]
pub(crate) use self::nats::*;
#[cfg(feature = "sources-nginx_metrics")]
pub(crate) use self::nginx_metrics::*;
#[cfg(any(
    feature = "sinks-datadog_events",
    feature = "sources-kubernetes_logs",
    feature = "transforms-geoip",
    feature = "transforms-log_to_metric",
))]
pub(crate) use self::parser::*;
#[cfg(feature = "sources-postgresql_metrics")]
pub(crate) use self::postgresql_metrics::*;
#[cfg(any(feature = "sources-prometheus", feature = "sinks-prometheus"))]
pub(crate) use self::prometheus::*;
#[cfg(any(feature = "sources-redis", feature = "sinks-redis"))]
pub(crate) use self::redis::*;
#[cfg(feature = "transforms-reduce")]
pub(crate) use self::reduce::*;
#[cfg(feature = "transforms-remap")]
pub(crate) use self::remap::*;
#[cfg(feature = "transforms-sample")]
pub(crate) use self::sample::*;
#[cfg(feature = "sinks-sematext")]
pub(crate) use self::sematext_metrics::*;
#[cfg(any(feature = "sources-splunk_hec", feature = "sinks-splunk_hec"))]
pub(crate) use self::splunk_hec::*;
#[cfg(feature = "sinks-statsd")]
pub(crate) use self::statsd_sink::*;
#[cfg(feature = "sources-statsd")]
pub(crate) use self::statsd_source::*;
#[cfg(feature = "sources-syslog")]
pub(crate) use self::syslog::*;
#[cfg(feature = "transforms-tag_cardinality_limit")]
pub(crate) use self::tag_cardinality_limit::*;
#[cfg(feature = "transforms-throttle")]
pub(crate) use self::throttle::*;
#[cfg(all(
    any(
        feature = "sinks-socket",
        feature = "sinks-statsd",
        feature = "sources-dnstap",
        feature = "sources-metrics",
        feature = "sources-statsd",
        feature = "sources-syslog",
        feature = "sources-socket"
    ),
    unix
))]
pub(crate) use self::unix::*;
#[cfg(feature = "sources-vector")]
pub(crate) use self::vector::*;
#[cfg(feature = "sinks-websocket")]
pub(crate) use self::websocket::*;
#[cfg(windows)]
pub(crate) use self::windows::*;
pub(crate) use self::{
    adaptive_concurrency::*, batch::*, common::*, conditions::*, encoding_transcode::*,
    heartbeat::*, open::*, process::*, socket::*, tcp::*, template::*, udp::*,
};

// this version won't be needed once all `InternalEvent`s implement `name()`
#[cfg(test)]
#[macro_export]
macro_rules! emit {
    ($event:expr) => {
        vector_common::internal_event::emit(vector_common::internal_event::DefaultName {
            event: $event,
            name: stringify!($event),
        })
    };
}

#[cfg(not(test))]
#[macro_export]
macro_rules! emit {
    ($event:expr) => {
        vector_common::internal_event::emit($event)
    };
}

#[cfg(test)]
#[macro_export]
macro_rules! register {
    ($event:expr) => {
        vector_common::internal_event::register(vector_common::internal_event::DefaultName {
            event: $event,
            name: stringify!($event),
        })
    };
}

#[cfg(not(test))]
#[macro_export]
macro_rules! register {
    ($event:expr) => {
        vector_common::internal_event::register($event)
    };
}
