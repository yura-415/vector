use crate::{
    config::{DataType, GenerateConfig, SinkConfig, SinkContext, SinkDescription},
    sinks::{
        http::{HttpMethod, HttpSinkConfig},
        util::{
            encoding::EncodingConfig, http::RequestConfig, BatchConfig, Compression, Concurrency,
            TowerRequestConfig, UriSerde,
        },
    },
    tls::TlsOptions,
};
use serde::{Deserialize, Serialize};
use snafu::Snafu;

// Experimentation here. Not sure what the batch size really should max out at.
// Setting it to 1MB (10^6 bytes) for now
const MAX_PAYLOAD_SIZE: usize = 1_000_000 as usize;

#[derive(Debug, Snafu)]
enum BuildError {
    #[snafu(display("Missing host in address field"))]
    MissingHost,
    #[snafu(display("Missing port in address field"))]
    MissingPort,
    #[snafu(display(
        "Too high batch max size. The value must be {} bytes or less",
        MAX_PAYLOAD_SIZE
    ))]
    BatchMaxSize,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct VectorSinkConfig {
    pub address: Option<UriSerde>,
    #[serde(default)]
    pub compression: Compression,
    pub encoding: EncodingConfig<Encoding>,
    #[serde(default)]
    pub batch: BatchConfig,
    #[serde(default)]
    pub request: TowerRequestConfig,
    pub tls: Option<TlsOptions>,
}

inventory::submit! {
    SinkDescription::new::<VectorSinkConfig>("vector")
}

impl GenerateConfig for VectorSinkConfig {
    fn generate_config() -> toml::Value {
        toml::Value::try_from(Self::with_encoding(Encoding::Json)).unwrap()
    }
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Encoding {
    Json,
}

impl From<Encoding> for crate::sinks::http::Encoding {
    fn from(v: Encoding) -> crate::sinks::http::Encoding {
        match v {
            Encoding::Json => crate::sinks::http::Encoding::Json,
        }
    }
}

#[async_trait::async_trait]
#[typetag::serde(name = "vector")]
impl SinkConfig for VectorSinkConfig {
    async fn build(
        &self,
        cx: SinkContext,
    ) -> crate::Result<(super::VectorSink, super::Healthcheck)> {
        let http_conf = self.create_config()?;
        http_conf.build(cx).await
    }

    fn input_type(&self) -> DataType {
        DataType::Any
    }

    fn sink_type(&self) -> &'static str {
        "vector"
    }
}

impl VectorSinkConfig {
    fn with_encoding(encoding: Encoding) -> Self {
        Self {
            address: None,
            compression: Compression::default(),
            encoding: encoding.into(),
            batch: BatchConfig::default(),
            request: TowerRequestConfig::default(),
            tls: None,
        }
    }

    fn create_config(&self) -> crate::Result<HttpSinkConfig> {
        let uri = self
            .address
            .clone()
            .unwrap_or("http://localhost:5000".parse().unwrap());
        let batch = self.batch.use_size_as_bytes()?;

        // Limit batch's max_bytes to 1MB
        let max_payload_size = batch.max_bytes.unwrap_or(MAX_PAYLOAD_SIZE);
        if max_payload_size > MAX_PAYLOAD_SIZE {
            return Err(Box::new(BuildError::BatchMaxSize));
        }

        let batch = BatchConfig {
            max_bytes: Some(batch.max_bytes.unwrap_or(MAX_PAYLOAD_SIZE)),
            max_events: None,
            ..batch
        };
        let tls = self.tls.clone().or(None);
        let tower = TowerRequestConfig {
            // Honestly not too sure on what these values _should_ be just yet,
            // defaults look on the light side so increasing them a bit.
            concurrency: (self.request.concurrency).if_none(Concurrency::Fixed(75)),
            rate_limit_num: Some(self.request.rate_limit_num.unwrap_or(75)),
            ..self.request
        };

        let request = RequestConfig {
            tower,
            ..Default::default()
        };

        Ok(HttpSinkConfig {
            uri,
            method: Some(HttpMethod::Post),
            auth: None,
            headers: None,
            compression: self.compression,
            encoding: self.encoding.clone().into_encoding(),
            batch,
            request,
            tls,
        })
    }
}
