use crate::tests::assert_no_errors;

#[test]
fn collects_only_functions_marked_with_formal_attribute() {
    let src = r#"
        #[formal]
        fn foo() {}

        fn bar() {}

        fn main() {
            foo();
            bar();
        }
    "#;
    let context = assert_no_errors(src);
    let formal_functions = context.get_all_formal_functions_in_crate(context.root_crate_id());
    let names: Vec<&str> = formal_functions.iter().map(|(name, _)| name.as_str()).collect();
    assert_eq!(names, vec!["foo"]);
}

#[test]
fn collects_no_functions_when_none_are_marked_with_formal_attribute() {
    let src = r#"
        fn bar() {}

        fn main() {
            bar();
        }
    "#;
    let context = assert_no_errors(src);
    let formal_functions = context.get_all_formal_functions_in_crate(context.root_crate_id());
    assert!(formal_functions.is_empty());
}
