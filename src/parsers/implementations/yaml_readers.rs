use serde::de::DeserializeOwned;
use std::{fs::File, io::{self, Read}, path::Path};

pub fn load_from_yaml_path<T: DeserializeOwned>(yaml_relative_path: &Path) -> Result<T, std::io::Error> {

  let mut file = match File::open(&yaml_relative_path) {
    Ok(file) => file,
    Err(err) => {
      return Err(io::Error::new(err.kind(), format!(
        "File {file} cannot open and failed with error : \n {error}",
        file = yaml_relative_path.display(),
        error = err.to_string()
      )));
    }
  };

  let mut contents = String::new();
  match file.read_to_string(&mut contents) {
    Ok(_) => {},
    Err(err) => {
      return Err(io::Error::new(err.kind(), format!(
        "Error while reading YAML file {file} content to string at : \n {error}",
        file = yaml_relative_path.display(),
        error = err.to_string()
      )));
    }
  };

  let config: T = match yaml_serde::from_str(&contents) {
    Ok(config) => {config},
    Err(err) => {
      return Err(io::Error::new(io::ErrorKind::InvalidData, format!(
        "Error : YAML format is invalid inside file {file} : \n {error}",
        file = yaml_relative_path.display(),
        error = err.to_string()
      )));
    }
  };
  
  return Ok(config);
}