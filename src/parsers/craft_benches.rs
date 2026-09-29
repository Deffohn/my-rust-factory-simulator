use crate::core::simulation::craft_benches::{CraftBenchKind, new_craft_bench_kind};
use crate::parsers::config::FromYAMLConfig;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CraftBenchConfig {
  id: String,
  name: String,

  #[serde(default)]
  description: String,
}

impl FromYAMLConfig for CraftBenchConfig {
  
  const YAML_DIRECTORY_RELATIVE_PATH: &str = "resources/crafts/benches";
  type Target = CraftBenchKind;

  fn load_from_origin_config(&self) -> Self::Target {
    return new_craft_bench_kind(
      self.id.clone(),
      self.name.clone(),
      Some(self.description.clone())
    );
  }
}