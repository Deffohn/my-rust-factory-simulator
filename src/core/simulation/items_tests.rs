use super::*;

#[test]
fn item_kind_exists() {
  let item_kind = new_item_kind(
    "kind_test".to_string(),
    "kind_test_name".to_string(),
    Some("description test text".to_string())
  );

  assert_eq!(item_kind.id, "kind_test");
  assert_eq!(item_kind.name, "kind_test_name");
  assert_eq!(item_kind.description, "description test text");
}