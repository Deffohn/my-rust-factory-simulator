#[cfg(test)]
#[path = "craft_benches_tests.rs"]
mod tests;

pub struct CraftBenchKind {
  id: String,
  name: String,
  description: String,
}

pub fn new_craft_bench_kind(
  id: String,
  name: String,
  description: Option<String>,
) -> CraftBenchKind {
  return CraftBenchKind {
    id: id,
    name: name,
    description: (description.as_deref().unwrap_or("").to_string())
  }
}