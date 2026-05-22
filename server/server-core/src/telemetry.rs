//! Usage:
//!   let _guard = telemetry::init("my-service").await?;
//!
//!   let app = Router::new()
//!       .route("/", get(handler))
//!       .layer(telemetry::http_trace_layer());
//!
//! Environment variables:
//!   OTEL_EXPORTER_OTLP_ENDPOINT  — set to enable OTel export (e.g. http://localhost:4317)
//!                                   if unset, spans are logged locally only
//!   RUST_LOG                     — log filter (e.g. my_app=debug,tower_http=debug)
//!   APP_ENV=production           — switches fmt layer to JSON output
//!   OTEL_TRACES_SAMPLER          — always_on (default) | always_off | traceidratio | parentbased_*
//!   OTEL_TRACES_SAMPLER_ARG      — ratio for traceidratio (e.g. 0.1 = 10%)

use axum::body::Body;
use http::{Request, Response};
use opentelemetry::global;
use opentelemetry_otlp::{SpanExporter, WithExportConfig};
use opentelemetry_sdk::{
    Resource,
    propagation::TraceContextPropagator,
    trace::{
        BatchConfigBuilder, BatchSpanProcessor, RandomIdGenerator, Sampler, SdkTracerProvider,
    },
};
use std::{env, time::Duration};
use tower_http::{
    classify::{ServerErrorsAsFailures, SharedClassifier},
    trace::{DefaultOnBodyChunk, DefaultOnEos, DefaultOnRequest, MakeSpan, TraceLayer},
};
use tracing::{Level, Span, field};
use tracing_subscriber::{EnvFilter, Layer, layer::SubscriberExt, util::SubscriberInitExt};

// ── Public surface ────────────────────────────────────────────────────────────

pub async fn init(service_name: &'static str) -> anyhow::Result<OtelGuard> {
    global::set_text_map_propagator(TraceContextPropagator::new());

    // ── OTel pipeline (only when OTEL_EXPORTER_OTLP_ENDPOINT is set) ──────────
    let provider: Option<SdkTracerProvider> =
        if let Ok(endpoint) = env::var("OTEL_EXPORTER_OTLP_ENDPOINT") {
            let resource = Resource::builder().with_service_name(service_name).build();

            let exporter = SpanExporter::builder()
                .with_tonic()
                .with_endpoint(endpoint)
                .with_timeout(Duration::from_secs(3))
                .build()?;

            let batch_config = BatchConfigBuilder::default()
                .with_max_queue_size(4096)
                .with_scheduled_delay(Duration::from_millis(500))
                .with_max_export_batch_size(512)
                .build();

            let batch_processor = BatchSpanProcessor::builder(exporter)
                .with_batch_config(batch_config)
                .build();

            let p = SdkTracerProvider::builder()
                .with_span_processor(batch_processor)
                .with_sampler(sampler_from_env())
                .with_id_generator(RandomIdGenerator::default())
                .with_max_events_per_span(64)
                .with_max_attributes_per_span(32)
                .with_resource(resource)
                .build();

            let _tracer = opentelemetry::trace::TracerProvider::tracer(&p, service_name);
            global::set_tracer_provider(p.clone());

            // Register OTel layer only when we have a real pipeline.
            // We return it via a boxed layer below so the subscriber
            // composition is uniform whether OTel is enabled or not.
            Some(p)
        } else {
            tracing::debug!("OTEL_EXPORTER_OTLP_ENDPOINT not set — OTel export disabled");
            None
        };

    // ── fmt layer ─────────────────────────────────────────────────────────────
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true);

    let fmt_layer: Box<dyn Layer<_> + Send + Sync> =
        if env::var("APP_ENV").as_deref() == Ok("production") {
            Box::new(fmt_layer.json())
        } else {
            Box::new(fmt_layer.pretty())
        };

    // ── EnvFilter ─────────────────────────────────────────────────────────────
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        EnvFilter::new(format!(
            "{service_name}=debug,tower_http=debug,axum::rejection=trace,info"
        ))
    });

    // ── Compose subscriber (OTel layer is optional) ───────────────────────────
    // Box the OTel layer so both branches have the same type.
    let otel_layer: Box<dyn Layer<_> + Send + Sync> = if let Some(ref p) = provider {
        let tracer = opentelemetry::trace::TracerProvider::tracer(p, service_name);
        Box::new(tracing_opentelemetry::layer().with_tracer(tracer))
    } else {
        Box::new(tracing_subscriber::layer::Identity::new())
    };

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt_layer)
        .with(otel_layer)
        .init();

    tracing::info!(service = service_name, "telemetry initialised");

    Ok(OtelGuard { provider })
}

/// Shutdown guard — keeps the OTel pipeline alive. Drop to flush.
pub struct OtelGuard {
    provider: Option<SdkTracerProvider>,
}

impl Drop for OtelGuard {
    fn drop(&mut self) {
        if let Some(ref provider) = self.provider {
            tracing::info!("flushing OpenTelemetry spans…");
            if let Err(e) = provider.shutdown() {
                eprintln!("OTel shutdown error: {e:?}");
            }
        }
    }
}

// ── HTTP trace layer ──────────────────────────────────────────────────────────

pub fn http_trace_layer()
-> TraceLayer<SharedClassifier<ServerErrorsAsFailures>, HttpMakeSpan, DefaultOnRequest, OnResponse>
{
    TraceLayer::new_for_http()
        .make_span_with(HttpMakeSpan)
        .on_response(OnResponse)
        .on_body_chunk(DefaultOnBodyChunk::new())
        .on_eos(DefaultOnEos::new())
}

// ── Span construction ─────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct HttpMakeSpan;

impl MakeSpan<Body> for HttpMakeSpan {
    fn make_span(&mut self, req: &Request<Body>) -> Span {
        let parent_cx =
            global::get_text_map_propagator(|p| p.extract(&HeaderExtractor(req.headers())));

        let span = tracing::span!(
            Level::INFO,
            "http.server.request",
            "http.request.method"       = req.method().as_str(),
            "url.path"                  = req.uri().path(),
            "url.query"                 = req.uri().query().unwrap_or(""),
            "network.protocol.version"  = ?req.version(),
            "server.address"            = req.uri().host().unwrap_or(""),
            "http.response.status_code" = field::Empty,
            "otel.status_code"          = field::Empty,
            "user_agent.original"       = req.headers()
                .get(http::header::USER_AGENT)
                .and_then(|v| v.to_str().ok())
                .unwrap_or(""),
            "http.request.id"           = req.headers()
                .get("x-request-id")
                .and_then(|v| v.to_str().ok())
                .unwrap_or(""),
        );

        use tracing_opentelemetry::OpenTelemetrySpanExt;
        let _ = span.set_parent(parent_cx);
        span
    }
}

// ── Response recorder ─────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct OnResponse;

impl<B> tower_http::trace::OnResponse<B> for OnResponse {
    fn on_response(self, res: &Response<B>, latency: Duration, span: &Span) {
        let status = res.status().as_u16();
        span.record("http.response.status_code", status);
        if status >= 500 {
            span.record("otel.status_code", "ERROR");
            tracing::error!(
                http.response.status_code = status,
                latency = ?latency,
                "request failed"
            );
        } else {
            span.record("otel.status_code", "OK");
            tracing::info!(
                http.response.status_code = status,
                latency = ?latency,
                "request complete"
            );
        }
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

struct HeaderExtractor<'a>(&'a http::HeaderMap);

impl<'a> opentelemetry::propagation::Extractor for HeaderExtractor<'a> {
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).and_then(|v| v.to_str().ok())
    }
    fn keys(&self) -> Vec<&str> {
        self.0.keys().map(|k| k.as_str()).collect()
    }
}

fn sampler_from_env() -> Sampler {
    let ratio = env::var("OTEL_TRACES_SAMPLER_ARG")
        .ok()
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or(1.0);

    match env::var("OTEL_TRACES_SAMPLER")
        .as_deref()
        .unwrap_or("always_on")
    {
        "always_off" => Sampler::AlwaysOff,
        "traceidratio" => Sampler::TraceIdRatioBased(ratio),
        "parentbased_always_on" => Sampler::ParentBased(Box::new(Sampler::AlwaysOn)),
        "parentbased_always_off" => Sampler::ParentBased(Box::new(Sampler::AlwaysOff)),
        "parentbased_traceidratio" => {
            Sampler::ParentBased(Box::new(Sampler::TraceIdRatioBased(ratio)))
        }
        _ => Sampler::AlwaysOn,
    }
}
