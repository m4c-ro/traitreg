#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/api_misuse/not_register_impl.rs");
    t.compile_fail("tests/api_misuse/register_impl_for_array.rs");
    t.compile_fail("tests/api_misuse/register_impl_for_inferred.rs");
    t.compile_fail("tests/api_misuse/register_impl_for_never.rs");
    t.compile_fail("tests/api_misuse/register_impl_for_pointer.rs");
    t.compile_fail("tests/api_misuse/register_impl_for_reference.rs");
    t.compile_fail("tests/api_misuse/register_impl_for_tuple.rs");
    t.compile_fail("tests/api_misuse/register_self_impl.rs");
    t.compile_fail("tests/api_misuse/register_struct_with_missing_constructor.rs");
}
