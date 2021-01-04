pub mod v1;
pub mod v2;

use crate::{
    config::{DataType, GenerateConfig, SinkConfig, SinkContext, SinkDescription},
    sinks::{Healthcheck, VectorSink},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
enum V1 {
    #[serde(rename = "1")]
    V1,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct VectorSinkConfigV1 {
    version: Option<V1>,
    #[serde(flatten)]
    config: v1::VectorSinkConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum VectorSinkConfig {
    V1(VectorSinkConfigV1),
    V2(VectorSinkConfigV2),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
enum V2 {
    #[serde(rename = "2")]
    V2,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct VectorSinkConfigV2 {
    version: V2,
    #[serde(flatten)]
    config: v2::VectorSinkConfig,
}

inventory::submit! {
    SinkDescription::new::<VectorSinkConfig>("vector")
}

impl GenerateConfig for VectorSinkConfig {
    fn generate_config() -> toml::Value {
        toml::from_str(
            r#"version = "2"
            hooks.process = """#,
        )
        .unwrap()
    }
}

#[async_trait::async_trait]
#[typetag::serde(name = "vector")]
impl SinkConfig for VectorSinkConfig {
    async fn build(&self, cx: SinkContext) -> crate::Result<(VectorSink, Healthcheck)> {
        match self {
            VectorSinkConfig::V1(v1) => v1.config.build(cx).await,
            VectorSinkConfig::V2(v2) => v2.config.build(cx).await, //           VectorSinkConfig::V2(v2) => v2.config.build(),                                                                 //           VectorSinkConfig::V2(v2) => v2.config.build(),
        }
    }

    fn input_type(&self) -> DataType {
        match self {
            VectorSinkConfig::V1(v1) => v1.config.input_type(),
            VectorSinkConfig::V2(v2) => v2.config.input_type(),
        }
    }

    fn sink_type(&self) -> &'static str {
        match self {
            VectorSinkConfig::V1(v1) => v1.config.sink_type(),
            VectorSinkConfig::V2(v2) => v2.config.sink_type(),
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn generate_config() {
        crate::test_util::test_generate_config::<super::VectorSinkConfig>();
    }
}
