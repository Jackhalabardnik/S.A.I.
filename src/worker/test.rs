use super::*;

#[test]
fn propely_splits_command_with_reverse_quotes() {
    let expected_result = vec!["name", "arg1", "\"`Text\" \"in\" \"quotes`\"", "arg2"];
    assert_eq!(
        split_command_into_parts(&"name arg1 \"`Text\" \"in\" \"quotes`\" arg2".to_string()),
        expected_result
    );
}
#[test]
fn propely_splits_command_with_quotes() {
    let expected_result = vec!["name", "arg1", "\"Text in quotes\"", "arg2"];
    assert_eq!(
        split_command_into_parts(&"name arg1 \"Text in quotes\" arg2".to_string()),
        expected_result
    );
}

#[test]
fn splits_at_char_appending_chars() {
    let mut expected_result = vec!["AA", "CAAC", "", "CAC", "A", "CAC", "A"];
    assert_eq!(
        split_at_char_and_append_it(&"AACAACCACACACA".to_string(), 'C'),
        expected_result
    );

    expected_result = vec!["AA", "CAAC", "", "CAC", "A", "CAC"];
    assert_eq!(
        split_at_char_and_append_it(&"AACAACCACACAC".to_string(), 'C'),
        expected_result
    );
}
