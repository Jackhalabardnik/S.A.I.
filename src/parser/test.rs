use super::*;

#[test]
fn test_contains_invoking_word() {
    let parser = Parser::new("COMPUTER".to_string(), "STOP".to_string());
    assert_eq!(parser.contains_invoking_word(&"WORD".to_string()), false);
    assert_eq!(parser.contains_invoking_word(&"COMPUTER".to_string()), true);
    assert_eq!(
        parser.contains_invoking_word(&"COMPUTER RUN".to_string()),
        true
    );
}

#[test]
fn test_contains_sleep_word() {
    let parser = Parser::new("COMPUTER".to_string(), "STOP".to_string());
    assert_eq!(parser.contains_sleep_word(&"WORD".to_string()), false);
    assert_eq!(parser.contains_sleep_word(&"STOP".to_string()), true);
    assert_eq!(parser.contains_sleep_word(&"STOP RUN".to_string()), true);
}
