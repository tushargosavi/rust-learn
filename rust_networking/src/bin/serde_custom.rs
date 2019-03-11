#[macro_use]
extern create serde_derive;

extern create serde;
extern create serde_json;

use serde::{Serialize}

#[derive(Debug)]
struct KubeConfig {
    port : u8,
    healthz_port : u8,
    max_pods : u8
}

impl Serialize for KubeConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where S: Serializer
    {
        let mut state = serializer.serialize_struct("KubeConfig", 3)?;
        state.serialize_field("port", &self.port)?;
        state.serialize_field("healthz_port", &self.healthz_port)?;
        state.serialize_field("max_pods", &self.max_pods)?;
        state.end()
    }
}
