use super::*;

#[test]
fn test_contains_special_word() {
    let parser = Parser::new("COMPUTER".to_string());
    assert_eq!(parser.contains_special_word(&"WORD".to_string()), false);
    assert_eq!(parser.contains_special_word(&"COMPUTER".to_string()), true);
    assert_eq!(
        parser.contains_special_word(&"COMPUTER RUN".to_string()),
        true
    );
}
