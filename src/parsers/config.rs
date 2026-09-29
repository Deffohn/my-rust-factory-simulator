use serde::de::DeserializeOwned;

use crate::parsers::implementations::{yaml_readers};

use std::{fs, io, path::Path};

const YAML_EXTENSION: &str = "yaml";

pub trait FromYAMLConfig: DeserializeOwned {
  const YAML_DIRECTORY_RELATIVE_PATH: &str;
  type Target;

  fn load_from_origin_config(&self) -> Self::Target;
}

pub fn initialize_from_yamls<OriginConfig>() -> Result<Vec::<OriginConfig::Target>, std::io::Error>
    where OriginConfig: FromYAMLConfig
{
  
  let target_directory = Path::new(&OriginConfig::YAML_DIRECTORY_RELATIVE_PATH);

  println!(
    "Initializing YAML configs from \"{directory_path}\"",
    directory_path = target_directory.display()
  );

  let mut configs: Vec<OriginConfig> = Vec::new();
  if !target_directory.is_dir() {
    return Err(io::Error::new(io::ErrorKind::NotADirectory, format!(
      "Path at \"{directory_path}\" should be a directory",
      directory_path = target_directory.display()
    )));
  }

  for path_element in fs::read_dir(target_directory)? {
    let path_element = path_element?;
    let path = path_element.path();

    let path_extension = path.extension();

    if (!path.is_file()) || (path_extension.is_none()) || (path_extension.unwrap().ne(YAML_EXTENSION)) {
      // TODO warning logging handler
      println!(
        "[Warning] \"{path_display}\" isn't a YAML file! Directory \"{directory_path}\" shouldn't store anything else",
        path_display = path.display(),
        directory_path = target_directory.display()
      );
      continue;
    }

    match yaml_readers::load_from_yaml_path::<OriginConfig>(&path) {
      Ok(config) => configs.push(config),
      Err(err) => {
        return Err(io::Error::new(err.kind(), format!(
          "Loading {file} YAML file failed with error : \n{error}",
          file = path.display(),
          error = err.to_string()
        )));
      }
    };
  }

  let mut targets = Vec::<OriginConfig::Target>::new();
  for yaml_config in configs {
    let target: OriginConfig::Target = yaml_config.load_from_origin_config();
    targets.push(target);
  }

  Ok(targets)
}