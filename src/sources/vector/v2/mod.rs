use crate::{
    config::{
        log_schema, DataType, GenerateConfig, GlobalOptions, Resource, SourceConfig,
        SourceDescription,
    },
    event::{Event, Value},
    //internal_events::{VectorEventReceived, VectorProtoDecodeError},
    shutdown::ShutdownSignal,
    sources::{
        util::{add_query_parameters, ErrorMessage, HttpSource, HttpSourceAuthConfig},
        Source,
    },
    tls::TlsConfig,
    Pipeline,
};
use bytes::Bytes;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::{collections::HashMap, net::SocketAddr};

use warp::http::{HeaderMap, HeaderValue, StatusCode};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct VectorSourceConfig {
    pub address: SocketAddr,
    #[serde(default)]
    encoding: Encoding,
    #[serde(default)]
    headers: Vec<String>,
    #[serde(default)]
    query_parameters: Vec<String>,
    tls: Option<TlsConfig>,
    auth: Option<HttpSourceAuthConfig>,
}

inventory::submit! {
    SourceDescription::new::<VectorSourceConfig>("vector")
}

impl GenerateConfig for VectorSourceConfig {
    fn generate_config() -> toml::Value {
        toml::Value::try_from(Self {
            address: "0.0.0.0:9000".parse().unwrap(),
            encoding: Default::default(),
            headers: Vec::new(),
            query_parameters: Vec::new(),
            tls: None,
            auth: None,
        })
        .unwrap()
    }
}

#[async_trait::async_trait]
#[typetag::serde(name = "vector")]
impl SourceConfig for VectorSourceConfig {
    async fn build(
        &self,
        _name: &str,
        _globals: &GlobalOptions,
        shutdown: ShutdownSignal,
        out: Pipeline,
    ) -> crate::Result<Source> {
        let source = VectorSource {
            encoding: self.encoding,
            headers: self.headers.clone(),
            query_parameters: self.query_parameters.clone(),
        };

        source.run(self.address, "", &self.tls, &self.auth, out, shutdown)
    }

    fn output_type(&self) -> DataType {
        DataType::Any
    }

    fn source_type(&self) -> &'static str {
        "vector"
    }

    fn resources(&self) -> Vec<Resource> {
        vec![Resource::tcp(self.address.into())]
    }
}

#[derive(Debug, Clone)]
struct VectorSource {
    encoding: Encoding,
    headers: Vec<String>,
    query_parameters: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Clone, Copy, Derivative)]
#[serde(rename_all = "snake_case")]
#[derivative(Default)]
pub enum Encoding {
    #[derivative(Default)]
    Json,
}

impl HttpSource for VectorSource {
    fn build_event(
        &self,
        body: Bytes,
        header_map: HeaderMap,
        query_parameters: HashMap<String, String>,
    ) -> Result<Vec<Event>, ErrorMessage> {
        decode_body(body, self.encoding)
            .map(|events| add_headers(events, &self.headers, header_map))
            .map(|events| add_query_parameters(events, &self.query_parameters, query_parameters))
            .map(|mut events| {
                // Add source type
                let key = log_schema().source_type_key();
                for event in events.iter_mut() {
                    event.as_mut_log().try_insert(key, Bytes::from("http"));
                }
                events
            })
    }
}

fn add_headers(
    mut events: Vec<Event>,
    headers_config: &[String],
    headers: HeaderMap,
) -> Vec<Event> {
    for header_name in headers_config {
        let value = headers.get(header_name).map(HeaderValue::as_bytes);

        for event in events.iter_mut() {
            event.as_mut_log().insert(
                header_name as &str,
                Value::from(value.map(Bytes::copy_from_slice)),
            );
        }
    }

    events
}

fn decode_body(body: Bytes, enc: Encoding) -> Result<Vec<Event>, ErrorMessage> {
    match enc {
        Encoding::Json => {
            let parsed_json = serde_json::from_slice(&body)
                .map_err(|error| json_error(format!("Error parsing Json: {:?}", error)))?;
            json_parse_array_of_object(parsed_json)
        }
    }
}

fn json_parse_object(value: JsonValue) -> Result<Event, ErrorMessage> {
    let mut event = Event::new_empty_log();
    let log = event.as_mut_log();
    log.insert(log_schema().timestamp_key(), Utc::now()); // Add timestamp
    match value {
        JsonValue::Object(map) => {
            for (k, v) in map {
                log.insert_flat(k, v);
            }
            Ok(event)
        }
        _ => Err(json_error(format!(
            "Expected Object, got {}",
            json_value_to_type_string(&value)
        ))),
    }
}

fn json_parse_array_of_object(value: JsonValue) -> Result<Vec<Event>, ErrorMessage> {
    match value {
        JsonValue::Array(v) => v
            .into_iter()
            .map(json_parse_object)
            .collect::<Result<_, _>>(),
        JsonValue::Object(map) => {
            //treat like an array of one object
            Ok(vec![json_parse_object(JsonValue::Object(map))?])
        }
        _ => Err(json_error(format!(
            "Expected Array or Object, got {}.",
            json_value_to_type_string(&value)
        ))),
    }
}

fn json_error(s: String) -> ErrorMessage {
    ErrorMessage::new(StatusCode::BAD_REQUEST, format!("Bad JSON: {}", s))
}

fn json_value_to_type_string(value: &JsonValue) -> &'static str {
    match value {
        JsonValue::Object(_) => "Object",
        JsonValue::Array(_) => "Array",
        JsonValue::String(_) => "String",
        JsonValue::Number(_) => "Number",
        JsonValue::Bool(_) => "Bool",
        JsonValue::Null => "Null",
    }
}

#[cfg(feature = "sources-vector")]
#[cfg(test)]
mod test {
    use super::VectorSourceConfig;
    use crate::shutdown::ShutdownSignal;
    use crate::{
        config::{GlobalOptions, SinkConfig, SinkContext, SourceConfig},
        event::{
            metric::{MetricKind, MetricValue},
            Metric,
        },
        sinks::vector::VectorSinkConfig,
        test_util::{collect_ready, next_addr, wait_for_tcp},
        tls::{TlsConfig, TlsOptions},
        Event, Pipeline,
    };
    use futures::stream;
    use std::net::SocketAddr;
    use tokio::time::{delay_for, Duration};

    #[test]
    fn generate_config() {
        crate::test_util::test_generate_config::<VectorSourceConfig>();
    }

    async fn stream_test(addr: SocketAddr, source: VectorSourceConfig, sink: VectorSinkConfig) {
        let (tx, rx) = Pipeline::new_test();

        let server = source
            .build(
                "default",
                &GlobalOptions::default(),
                ShutdownSignal::noop(),
                tx,
            )
            .await
            .unwrap();
        tokio::spawn(server);
        wait_for_tcp(addr).await;

        let cx = SinkContext::new_test();
        let (sink, _) = sink.build(cx).await.unwrap();

        let events = vec![
            Event::from("test"),
            Event::from("events"),
            Event::from("to roundtrip"),
            Event::from("through"),
            Event::from("the native"),
            Event::from("sink"),
            Event::from("and"),
            Event::from("source"),
            Event::Metric(Metric {
                name: String::from("also test a metric"),
                namespace: None,
                timestamp: None,
                tags: None,
                kind: MetricKind::Absolute,
                value: MetricValue::Counter { value: 1.0 },
            }),
        ];

        sink.run(stream::iter(events.clone())).await.unwrap();

        delay_for(Duration::from_millis(50)).await;

        let output = collect_ready(rx).await;
        assert_eq!(events, output);
    }

    #[tokio::test]
    async fn it_works_with_vector_sink() {
        let addr = next_addr();
        stream_test(
            addr,
            VectorSourceConfig::new(addr.into(), None, None),
            VectorSinkConfig {
                address: format!("localhost:{}", addr.port()),
                keepalive: None,
                tls: None,
            },
        )
        .await;
    }

    #[tokio::test]
    async fn it_works_with_vector_sink_tls() {
        let addr = next_addr();
        stream_test(
            addr,
            VectorSourceConfig::new(addr.into(), None, Some(TlsConfig::test_config())),
            VectorSinkConfig {
                address: format!("localhost:{}", addr.port()),
                keepalive: None,
                tls: Some(TlsConfig {
                    enabled: Some(true),
                    options: TlsOptions {
                        verify_certificate: Some(false),
                        ..Default::default()
                    },
                }),
            },
        )
        .await;
    }
}
