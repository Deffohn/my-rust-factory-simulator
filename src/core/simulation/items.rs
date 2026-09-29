#[cfg(test)]
#[path = "items_tests.rs"]
mod tests;

pub struct ItemKind {
  id: String,
  name: String,
  description: String,
}

pub fn new_item_kind(
  id: String,
  name: String,
  description: Option<String>,
) -> ItemKind {
  return ItemKind {
    id: id,
    name: name,
    description: (description.as_deref().unwrap_or("").to_string())
  }
}