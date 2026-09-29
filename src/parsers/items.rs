use crate::core::simulation::items::{ItemKind, new_item_kind};
use crate::parsers::config::FromYAMLConfig;

use serde::Deserialize;

pub const YAML_DIRECTORY_RELATIVE_PATH: &str = "resources/items";

#[derive(Debug, Deserialize)]
pub struct ItemConfig {
  id: String,
  name: String,

  #[serde(default)]
  description: String,
}

impl FromYAMLConfig for ItemConfig {
  
  const YAML_DIRECTORY_RELATIVE_PATH: &str = "resources/items";
  type Target = ItemKind;

  fn load_from_origin_config(&self) -> Self::Target {
    return new_item_kind(
      self.id.clone(),
      self.name.clone(),
      Some(self.description.clone())
    );
  }
}