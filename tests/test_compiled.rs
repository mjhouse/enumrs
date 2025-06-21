#[cfg(test)]
mod tests {

    #[test]
    fn test_bad_name() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/compiled/test_bad_name.rs");
    }

    #[test]
    fn test_missing_variable() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/compiled/test_missing_variable.rs");
    }

    #[test]
    fn test_missing_name() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/compiled/test_missing_name.rs");
    }

    #[test]
    fn test_duplicate_tags() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/compiled/test_duplicate_tags.rs");
    }

    #[test]
    fn test_mismatched_types() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/compiled/test_mismatched_types.rs");
    }

    #[test]
    fn test_malformed_tag() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/compiled/test_malformed_tag.rs");
    }
}
