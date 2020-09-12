use super::*;

#[test]
fn test_contains_special_word() {
    assert_eq!(contains_special_word(&"WORD".to_string()), false);
    assert_eq!(contains_special_word(&"COMPUTER".to_string()), true);
    assert_eq!(contains_special_word(&"COMPUTER RUN".to_string()), true);
}
