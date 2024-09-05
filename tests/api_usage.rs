#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.pass("tests/api_usage/empty_registry.rs");
    t.pass("tests/api_usage/register_impl_for_enum.rs");
    t.pass("tests/api_usage/register_impl_for_struct.rs");
    t.pass("tests/api_usage/register_impl_for_type.rs");
    t.pass("tests/api_usage/register_impl_for_union.rs");
    t.pass("tests/api_usage/registry_with_items.rs");
}
