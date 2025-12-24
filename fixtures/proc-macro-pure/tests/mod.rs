use anyhow::Result;

#[test]
fn proc_macro_pure() -> Result<()> {
    uniffi_dart::testing::run_library_mode_test("proc_macro_pure_uniffi", None)
}
