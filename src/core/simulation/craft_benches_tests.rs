use super::*;

#[test]
fn craft_bench_kind_exists() {
  let craft_bench_kind = new_craft_bench_kind(
    "kind_test".to_string(),
    "kind_test_name".to_string(),
    Some("description test text".to_string())
  );

  assert_eq!(craft_bench_kind.id, "kind_test");
  assert_eq!(craft_bench_kind.name, "kind_test_name");
  assert_eq!(craft_bench_kind.description, "description test text");
}