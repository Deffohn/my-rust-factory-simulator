use crate::{core::simulation::{craft_benches::CraftBenchKind, items::ItemKind}, parsers};

use std::io;

pub struct Simulation {
  item_kinds: Vec<ItemKind>,
  craft_bench_kinds: Vec::<CraftBenchKind>
}

pub fn initialize_simulation() -> io::Result<Simulation> {

  let initialized_items: Vec::<ItemKind> = match parsers::config::initialize_from_yamls::<parsers::items::ItemConfig> () {
    Ok(initialized_items) => initialized_items,
    Err(err) => {
      return Err(io::Error::new(err.kind(), format!(
        "Items initializer failed with error : \n{error}",
        error = err.to_string()
      )));
    }
  };

  let initialized_craft_benches: Vec::<CraftBenchKind> = match parsers::config::initialize_from_yamls::<parsers::craft_benches::CraftBenchConfig> () {
    Ok(initialized_craft_benches) => initialized_craft_benches,
    Err(err) => {
      return Err(io::Error::new(err.kind(), format!(
        "CraftBenches initializer failed with error : \n{error}",
        error = err.to_string()
      )));
    }
  };

  let simulation = Simulation {
    item_kinds: initialized_items,
    craft_bench_kinds: initialized_craft_benches
  };
  
  return Ok(simulation);
}